#!/bin/bash
cargo build -q || exit 1
RCC=./target/debug/rcc

# テストから呼び出す関数を gcc でコンパイルしておく
cat <<EOF | gcc -xc -c -o tmp2.o -
int ret3() { return 3; }
int ret5() { return 5; }
int add(int x, int y) { return x + y; }
int sub(int x, int y) { return x - y; }
int add6(int a, int b, int c, int d, int e, int f) { return a + b + c + d + e + f; }
// 呼び出し元が rsp を 16 バイトにそろえていれば 1 を返す
int aligned() { return (long)__builtin_frame_address(0) % 16 == 0; }
EOF

assert() {
  expected="$1"
  input="$2"
  $RCC "$input" > tmp.s || exit 1
  gcc -static -o tmp tmp.s tmp2.o
  ./tmp
  actual="$?"
  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

# ステップ1〜4
assert 0 "int main() { 0; }"
assert 42 "int main() { 42; }"
assert 21 "int main() { 5+20-4; }"
assert 41 "int main() { 12 + 34 - 5 ; }"

# ステップ5：四則演算とかっこ
assert 47 "int main() { 5+6*7; }"
assert 15 "int main() { 5*(9-6); }"
assert 4 "int main() { (3+5)/2; }"

# ステップ6：単項 + と -
assert 10 "int main() { -10+20; }"
assert 10 "int main() { - -10; }"
assert 10 "int main() { - - +10; }"

# ステップ7：比較演算子
assert 0 "int main() { 0==1; }"
assert 1 "int main() { 42==42; }"
assert 1 "int main() { 0!=1; }"
assert 0 "int main() { 42!=42; }"
assert 1 "int main() { 0<1; }"
assert 0 "int main() { 1<1; }"
assert 0 "int main() { 2<1; }"
assert 1 "int main() { 0<=1; }"
assert 1 "int main() { 1<=1; }"
assert 0 "int main() { 2<=1; }"
assert 1 "int main() { 1>0; }"
assert 0 "int main() { 1>1; }"
assert 0 "int main() { 1>2; }"
assert 1 "int main() { 1>=0; }"
assert 1 "int main() { 1>=1; }"
assert 0 "int main() { 1>=2; }"

# ステップ9：1文字のローカル変数
assert 3 "int main() { int a; a=3; a; }"
assert 8 "int main() { int a; int z; a=3; z=5; a+z; }"
assert 6 "int main() { int a; int b; a=b=3; a+b; }"
assert 14 "int main() { int a; int b; a=3; b=5*6-8; a+b/2; }"

# ステップ10：複数文字のローカル変数
assert 3 "int main() { int foo; foo=3; foo; }"
assert 6 "int main() { int foo; int bar; foo=1; bar=2+3; foo+bar; }"
assert 8 "int main() { int foo123; int bar; foo123=3; bar=5; foo123+bar; }"
assert 7 "int main() { int _x; int Hello_World; _x=3; Hello_World=4; _x+Hello_World; }"
assert 30 "int main() { int a; int b; int c; int d; int e; int f; int g; int h; int i; int j; int k; int l; int m; int n; int o; int p; int q; int r; int s; int t; int u; int v; int w; int x; int y; int z; int aa; int ab; int ac; int ad; a=1;b=2;c=3;d=4;e=5;f=6;g=7;h=8;i=9;j=10;k=11;l=12;m=13;n=14;o=15;p=16;q=17;r=18;s=19;t=20;u=21;v=22;w=23;x=24;y=25;z=26;aa=27;ab=28;ac=29;ad=30; ad; }"

# ステップ11：return
assert 1 "int main() { return 1; 2; 3; }"
assert 2 "int main() { 1; return 2; 3; }"
assert 3 "int main() { 1; 2; return 3; }"
assert 5 "int main() { int a; a=5; return a; }"
assert 3 "int main() { int returnx; returnx=3; return returnx; }"

# ステップ12：if / while / for
assert 3 "int main() { if (0) return 2; return 3; }"
assert 3 "int main() { if (1-1) return 2; return 3; }"
assert 2 "int main() { if (1) return 2; return 3; }"
assert 2 "int main() { if (2-1) return 2; return 3; }"
assert 4 "int main() { if (0) return 3; else return 4; }"
assert 3 "int main() { if (1) return 3; else return 4; }"
assert 7 "int main() { int a; a=0; if (a==0) if (a!=0) return 5; else return 7; return 9; }"
assert 10 "int main() { int i; i=0; while(i<10) i=i+1; return i; }"
assert 55 "int main() { int i; int j; i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j; }"
assert 3 "int main() { for (;;) return 3; return 5; }"
assert 11 "int main() { int i; i=0; for (; i<=10;) i=i+1; return i; }"
assert 5 "int main() { int x; x=0; while (x<5) if (x<10) x=x+1; return x; }"

# ステップ13：ブロック
assert 3 "int main() { {1; {2;} return 3;} }"
assert 5 "int main() { { {} return 5; } }"
assert 10 "int main() { int i; i=0; while(i<10) { i=i+1; } return i; }"
assert 55 "int main() { int i; int j; i=0; j=0; while(i<=10) { j=i+j; i=i+1; } return j; }"
assert 6 "int main() { int j; int i; j=0; for (i=0; i<=3; i=i+1) { j=j+i; } return j; }"
assert 7 "int main() { int a; a=1; if (a) { a=a+2; a=a*2; } else { a=100; } return a+1; }"
assert 100 "int main() { int a; a=0; if (a) { a=a+2; a=a*2; } else { a=100; } return a; }"

# ステップ14：関数呼び出し
assert 3 "int main() { return ret3(); }"
assert 5 "int main() { return ret5(); }"
assert 8 "int main() { return add(3, 5); }"
assert 2 "int main() { return sub(5, 3); }"
assert 21 "int main() { return add6(1, 2, 3, 4, 5, 6); }"
assert 66 "int main() { return add6(1, 2, add6(3, 4, 5, 6, 7, 8), 9, 10, 11); }"
assert 136 "int main() { return add6(1, 2, add6(3, add6(4, 5, 6, 7, 8, 9), 10, 11, 12, 13), 14, 15, 16); }"
assert 7 "int main() { int a; int b; a=3; b=add(a, 4); return b; }"
assert 1 "int main() { return aligned(); }"
assert 2 "int main() { return (1 + aligned()) * 1; }"
assert 3 "int main() { return ((1 + (1 + aligned())) * 1) * 1; }"

# ステップ15：関数の定義
assert 32 "int ret32() { return 32; } int main() { return ret32(); }"
assert 7 "int add2(int x, int y) { return x + y; } int main() { return add2(3, 4); }"
assert 1 "int sub2(int x, int y) { return x - y; } int main() { return sub2(4, 3); }"
assert 21 "int sum6(int a, int b, int c, int d, int e, int f) { return a + b + c + d + e + f; } int main() { return sum6(1, 2, 3, 4, 5, 6); }"
assert 55 "int fib(int n) { if (n <= 1) return n; return fib(n - 1) + fib(n - 2); } int main() { return fib(10); }"
assert 11 "int f(int x) { int a; a = x * 2; return a; } int main() { int a; int b; a = 1; b = f(5); return a + b; }"
assert 5 "int g() { return (1 + aligned()) * 1; } int main() { return g() + (1 + g()) * 1; }"
assert 3 "int main() { return later(); } int later() { return 3; }"

# ステップ16：単項 & と単項 *
assert 3 "int main() { int x; int *y; x=3; y=&x; return *y; }"
assert 3 "int main() { int x; int y; int *z; x=3; y=5; z=&y+8; return *z; }"
assert 5 "int main() { int x; int y; int *z; x=3; y=5; z=&x-8; return *z; }"
assert 7 "int main() { int x; int *y; x=3; y=&x; *y=7; return x; }"
assert 9 "int main() { int x; int *y; int **z; x=3; y=&x; z=&y; **z=9; return x; }"
assert 3 "int main() { int x; x=3; return *&x; }"
assert 6 "int set(int *p, int v) { *p = v; return 0; } int main() { int x; x=1; set(&x, 6); return x; }"
assert 53 "int swap(int *a, int *b) { int t; t=*a; *a=*b; *b=t; return 0; } int main() { int x; int y; x=3; y=5; swap(&x, &y); return x*10+y; }"

# ステップ17：int による宣言
assert 3 "int main() { int x; x=3; return x; }"
assert 7 "int main() { int x; int y; x=3; y=4; return x+y; }"
assert 5 "int main() { int x; x=5; { int y; y=x; } return x; }"
assert 8 "int add2(int a, int b) { return a+b; } int main() { return add2(3, 5); }"
assert 6 "int main() { int foo_bar1; foo_bar1=6; return foo_bar1; }"

# ステップ18：ポインタ型
assert 3 "int main() { int x; int *y; y=&x; *y=3; return x; }"
assert 3 "int main() { int x; int *y; int **z; x=3; y=&x; z=&y; return **z; }"
assert 11 "int main() { int a; int *p; int **pp; a=1; p=&a; pp=&p; *p=5; **pp=**pp+6; return a; }"
assert 4 "int deref(int *p) { return *p; } int main() { int x; x=4; return deref(&x); }"
assert 8 "int set2(int **pp, int v) { **pp = v; return 0; } int main() { int x; int *p; p=&x; set2(&p, 8); return x; }"

echo OK