#!/bin/bash
# rcc のテスト（ステップ28）
#   tests/test.c   … rcc でコンパイルする（テスト本体）
#   tests/helper.c … gcc でコンパイルする（assert とテスト用の関数）
set -e

cargo build -q
RCC=./target/debug/rcc

cargo test -q
mkdir -p ./tmp
gcc -c -o tmp/tmp-helper.o tests/helper.c

$RCC tests/test.c > tmp/tmp-test.s
gcc -static -o tmp/tmp-test tmp/tmp-test.s tmp/tmp-helper.o
./tmp/tmp-test