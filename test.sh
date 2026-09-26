#!/bin/bash
# rcc のテスト（ステップ28）
#   tests/test.c   … rcc でコンパイルする（テスト本体）
#   tests/helper.c … gcc でコンパイルする（assert とテスト用の関数）
set -e

cargo build -q
RCC=./target/debug/rcc

gcc -c -o tmp-helper.o tests/helper.c
$RCC tests/test.c > tmp-test.s
gcc -static -o tmp-test tmp-test.s tmp-helper.o
./tmp-test