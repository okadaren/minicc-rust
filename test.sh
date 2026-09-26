#!/bin/bash
cargo build -q || exit 1
RCC=./target/debug/rcc

assert() {
    expected="$1"
    input="$2"
    $RCC "$input" > tmp.s || exit 1
    gcc -static -o tmp tmp.s
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
assert 47 "5+6*7;"
assert 15 "5*(9-6);"
assert 4 "(3+5)/2;"

# ステップ6：単項 + と -
assert 10 "-10+20;"
assert 10 "- -10;"
assert 10 "- - +10;"

# ステップ7：比較演算子
assert 0 "0==1;"
assert 1 "42==42;"
assert 1 "0!=1;"
assert 0 "42!=42;"
assert 1 "0<1;"
assert 0 "1<1;"
assert 0 "2<1;"
assert 1 "0<=1;"
assert 1 "1<=1;"
assert 0 "2<=1;"
assert 1 "1>0;"
assert 0 "1>1;"
assert 0 "1>2;"
assert 1 "1>=0;"
assert 1 "1>=1;"
assert 0 "1>=2;"

# ステップ9：単一文字のローカル変数
assert 3 "a=3; a;"
assert 8 "a=3; z=5; a+z;"
assert 6 "a=b=3; a+b;"
assert 14 "a=3; b=5*6-8; a+b/2;"

# ステップ10：複数文字のローカル変数
assert 3 "foo=3; foo;"
assert 6 "foo=1; bar=2+3; foo+bar;"
assert 8 "foo123=3; bar=5; foo123+bar;"
assert 7 "_x=3; Hello_World=4; _x+Hello_World;"
assert 30 "a=1;b=2;c=3;d=4;e=5;f=6;g=7;h=8;i=9;j=10;k=11;l=12;m=13;n=14;o=15;p=16;q=17;r=18;s=19;t=20;u=21;v=22;w=23;x=24;y=25;z=26;aa=27;ab=28;ac=29;ad=30; ad;"

# ステップ11：return文
assert 1 "return 1; 2; 3;"
assert 2 "1; return 2; 3;"
assert 3 "1; 2; return 3;"
assert 5 "a=5; return a;"
assert 3 "returnx=3; return returnx;"

# ステップ12：if / while / for
assert 3 "if (0) return 2; return 3;"
assert 3 "if (1-1) return 2; return 3;"
assert 2 "if (1) return 2; return 3;"
assert 2 "if (2-1) return 2; return 3;"
assert 4 "if (0) return 3; else return 4;"
assert 3 "if (1) return 3; else return 4;"
assert 7 "a=0; if (a==0) if (a!=0) return 5; else return 7; return 9;"
assert 10 "i=0; while(i<10) i=i+1; return i;"
assert 55 "i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j;"
assert 3 "for (;;) return 3; return 5;"
assert 11 "i=0; for (; i<=10;) i=i+1; return i;"
assert 5 "x=0; while (x<5) if (x<10) x=x+1; return x;"

# ステップ13：ブロック
assert 3 "{1; {2;} return 3;}"
assert 5 "{ {} return 5; }"
assert 10 "i=0; while(i<10) { i=i+1; } return i;"
assert 55 "i=0; j=0; while(i<=10) { j=i+j; i=i+1; } return j;"
assert 6 "j=0; for (i=0; i<=3; i=i+1) { j=j+i; } return j;"
assert 7 "a=1; if (a) { a=a+2; a=a*2; } else { a=100; } return a+1;"
assert 100 "a=0; if (a) { a=a+2; a=a*2; } else { a=100; } return a;"

echo OK