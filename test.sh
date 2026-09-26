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

assert 3 "a=3; a;"
assert 8 "a=3; z=5; a+z;"
assert 6 "a=b=3; a+b;"
assert 14 "a=3; b=5*6-8; a+b/2;"

echo OK