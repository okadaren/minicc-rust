#!/bin/bash
cargo build -q || exit 1
RCC=./target/debug/rcc

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

# ステップ5：四則演算とかっこ
assert 47 "main() { 5+6*7; }"
assert 15 "main() { 5*(9-6); }"
assert 4 "main() { (3+5)/2; }"

# ステップ6：単項 + と -
assert 10 "main() { -10+20; }"
assert 10 "main() { - -10; }"
assert 10 "main() { - - +10; }"

# ステップ7：比較演算子
assert 0 "main() { 0==1; }"
assert 1 "main() { 42==42; }"
assert 1 "main() { 0!=1; }"
assert 0 "main() { 42!=42; }"
assert 1 "main() { 0<1; }"
assert 0 "main() { 1<1; }"
assert 0 "main() { 2<1; }"
assert 1 "main() { 0<=1; }"
assert 1 "main() { 1<=1; }"
assert 0 "main() { 2<=1; }"
assert 1 "main() { 1>0; }"
assert 0 "main() { 1>1; }"
assert 0 "main() { 1>2; }"
assert 1 "main() { 1>=0; }"
assert 1 "main() { 1>=1; }"
assert 0 "main() { 1>=2; }"

# ステップ9：単一文字のローカル変数
assert 3 "main() { a=3; a; }"
assert 8 "main() { a=3; z=5; a+z; }"
assert 6 "main() { a=b=3; a+b; }"
assert 14 "main() { a=3; b=5*6-8; a+b/2; }"

# ステップ10：複数文字のローカル変数
assert 3 "main() { foo=3; foo; }"
assert 6 "main() { foo=1; bar=2+3; foo+bar; }"
assert 8 "main() { foo123=3; bar=5; foo123+bar; }"
assert 7 "main() { _x=3; Hello_World=4; _x+Hello_World; }"
assert 30 "main() { a=1;b=2;c=3;d=4;e=5;f=6;g=7;h=8;i=9;j=10;k=11;l=12;m=13;n=14;o=15;p=16;q=17;r=18;s=19;t=20;u=21;v=22;w=23;x=24;y=25;z=26;aa=27;ab=28;ac=29;ad=30; ad; }"

# ステップ11：return文
assert 1 "main() { return 1; 2; 3; }"
assert 2 "main() { 1; return 2; 3; }"
assert 3 "main() { 1; 2; return 3; }"
assert 5 "main() { a=5; return a; }"
assert 3 "main() { returnx=3; return returnx; }"

# ステップ12：if / while / for
assert 3 "main() { if (0) return 2; return 3; }"
assert 3 "main() { if (1-1) return 2; return 3; }"
assert 2 "main() { if (1) return 2; return 3; }"
assert 2 "main() { if (2-1) return 2; return 3; }"
assert 4 "main() { if (0) return 3; else return 4; }"
assert 3 "main() { if (1) return 3; else return 4; }"
assert 7 "main() { a=0; if (a==0) if (a!=0) return 5; else return 7; return 9; }"
assert 10 "main() { i=0; while(i<10) i=i+1; return i; }"
assert 55 "main() { i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j; }"
assert 3 "main() { for (;;) return 3; return 5; }"
assert 11 "main() { i=0; for (; i<=10;) i=i+1; return i; }"
assert 5 "main() { x=0; while (x<5) if (x<10) x=x+1; return x; }"

# ステップ13：ブロック
assert 3 "main() { {1; {2;} return 3;} }"
assert 5 "main() { { {} return 5; } }"
assert 10 "main() { i=0; while(i<10) { i=i+1; } return i; }"
assert 55 "main() { i=0; j=0; while(i<=10) { j=i+j; i=i+1; } return j; }"
assert 6 "main() { j=0; for (i=0; i<=3; i=i+1) { j=j+i; } return j; }"
assert 7 "main() { a=1; if (a) { a=a+2; a=a*2; } else { a=100; } return a+1; }"
assert 100 "main() { a=0; if (a) { a=a+2; a=a*2; } else { a=100; } return a; }"

# ステップ14：関数呼び出し
assert 3 "main() { return ret3(); }"
assert 5 "main() { return ret5(); }"
assert 8 "main() { return add(3, 5); }"
assert 2 "main() { return sub(5, 3); }"
assert 21 "main() { return add6(1, 2, 3, 4, 5, 6); }"
assert 66 "main() { return add6(1, 2, add6(3, 4, 5, 6, 7, 8), 9, 10, 11); }"
assert 136 "main() { return add6(1, 2, add6(3, add6(4, 5, 6, 7, 8, 9), 10, 11, 12, 13), 14, 15, 16); }"
assert 7 "main() { a=3; b=add(a, 4); return b; }"
assert 1 "main() { return aligned(); }"
assert 2 "main() { return (1 + aligned()) * 1; }"
assert 3 "main() { return ((1 + (1 + aligned())) * 1) * 1; }"

# ステップ15：関数の定義
assert 32 "ret32() { return 32; } main() { return ret32(); }"
assert 7 "add2(x, y) { return x + y; } main() { return add2(3, 4); }"
assert 1 "sub2(x, y) { return x - y; } main() { return sub2(4, 3); }"
assert 21 "sum6(a, b, c, d, e, f) { return a + b + c + d + e + f; } main() { return sum6(1, 2, 3, 4, 5, 6); }"
assert 55 "fib(n) { if (n <= 1) return n; return fib(n - 1) + fib(n - 2); } main() { return fib(10); }"
assert 11 "f(x) { a = x * 2; return a; } main() { a = 1; b = f(5); return a + b; }"
assert 5 "g() { return (1 + aligned()) * 1; } main() { return g() + (1 + g()) * 1; }"
assert 3 "main() { return later(); } later() { return 3; }"

echo OK