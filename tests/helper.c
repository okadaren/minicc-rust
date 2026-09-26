// テストの補助関数。gcc でコンパイルし、rcc がコンパイルした test.c とリンクする
#include <stdio.h>
#include <stdlib.h>

// actual が expected と等しければ結果を表示し、違えば表示して終了する
int assert(int expected, int actual, char *code)
{
    if (expected == actual)
    {
        printf("%s => %d\n", code, actual);
        return 0;
    }
    printf("%s => %d expected, but got %d\n", code, expected, actual);
    exit(1);
}

int ret3() { return 3; }
int ret5() { return 5; }
int add(int x, int y) { return x + y; }
int sub(int x, int y) { return x - y; }
int add6(int a, int b, int c, int d, int e, int f) { return a + b + c + d + e + f; }

// 呼び出し元が rsp を 16 バイトにそろえていれば 1 を返す
int aligned() { return (long)__builtin_frame_address(0) % 16 == 0; }

// int を 4 つ並べた領域を確保し、その先頭アドレスを *p に入れる
void alloc4(int **p, int a, int b, int c, int d)
{
    *p = malloc(sizeof(int) * 4);
    (*p)[0] = a;
    (*p)[1] = b;
    (*p)[2] = c;
    (*p)[3] = d;
}