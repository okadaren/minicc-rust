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

assert 0 0
assert 42 42
assert 32 "15+22-5"
echo OK

cargo clean
rm -f tmp*