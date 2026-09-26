// rcc のテスト（ステップ28）。rcc でコンパイルし、tests/helper.c とリンクして実行する
// 各テストは t番号_main という関数にしてあり、main から assert で結果を確かめる

// ステップ1〜4
int t1_main() { 0; }
int t2_main() { 42; }
int t3_main() { 5 + 20 - 4; }
int t4_main() { 12 + 34 - 5; }

// ステップ5：四則演算とかっこ
int t5_main() { 5 + 6 * 7; }
int t6_main() { 5 * (9 - 6); }
int t7_main() { (3 + 5) / 2; }

// ステップ6：単項 + と -
int t8_main() { -10 + 20; }
int t9_main() { - -10; }
int t10_main() { - -+10; }

// ステップ7：比較演算子
int t11_main() { 0 == 1; }
int t12_main() { 42 == 42; }
int t13_main() { 0 != 1; }
int t14_main() { 42 != 42; }
int t15_main() { 0 < 1; }
int t16_main() { 1 < 1; }
int t17_main() { 2 < 1; }
int t18_main() { 0 <= 1; }
int t19_main() { 1 <= 1; }
int t20_main() { 2 <= 1; }
int t21_main() { 1 > 0; }
int t22_main() { 1 > 1; }
int t23_main() { 1 > 2; }
int t24_main() { 1 >= 0; }
int t25_main() { 1 >= 1; }
int t26_main() { 1 >= 2; }

// ステップ9：1文字のローカル変数
int t27_main()
{
    int a;
    a = 3;
    a;
}
int t28_main()
{
    int a;
    int z;
    a = 3;
    z = 5;
    a + z;
}
int t29_main()
{
    int a;
    int b;
    a = b = 3;
    a + b;
}
int t30_main()
{
    int a;
    int b;
    a = 3;
    b = 5 * 6 - 8;
    a + b / 2;
}

// ステップ10：複数文字のローカル変数
int t31_main()
{
    int foo;
    foo = 3;
    foo;
}
int t32_main()
{
    int foo;
    int bar;
    foo = 1;
    bar = 2 + 3;
    foo + bar;
}
int t33_main()
{
    int foo123;
    int bar;
    foo123 = 3;
    bar = 5;
    foo123 + bar;
}
int t34_main()
{
    int _x;
    int Hello_World;
    _x = 3;
    Hello_World = 4;
    _x + Hello_World;
}
int t35_main()
{
    int a;
    int b;
    int c;
    int d;
    int e;
    int f;
    int g;
    int h;
    int i;
    int j;
    int k;
    int l;
    int m;
    int n;
    int o;
    int p;
    int q;
    int r;
    int s;
    int t;
    int u;
    int v;
    int w;
    int x;
    int y;
    int z;
    int aa;
    int ab;
    int ac;
    int ad;
    a = 1;
    b = 2;
    c = 3;
    d = 4;
    e = 5;
    f = 6;
    g = 7;
    h = 8;
    i = 9;
    j = 10;
    k = 11;
    l = 12;
    m = 13;
    n = 14;
    o = 15;
    p = 16;
    q = 17;
    r = 18;
    s = 19;
    t = 20;
    u = 21;
    v = 22;
    w = 23;
    x = 24;
    y = 25;
    z = 26;
    aa = 27;
    ab = 28;
    ac = 29;
    ad = 30;
    ad;
}

// ステップ11：return
int t36_main()
{
    return 1;
    2;
    3;
}
int t37_main()
{
    1;
    return 2;
    3;
}
int t38_main()
{
    1;
    2;
    return 3;
}
int t39_main()
{
    int a;
    a = 5;
    return a;
}
int t40_main()
{
    int returnx;
    returnx = 3;
    return returnx;
}

// ステップ12：if / while / for
int t41_main()
{
    if (0)
        return 2;
    return 3;
}
int t42_main()
{
    if (1 - 1)
        return 2;
    return 3;
}
int t43_main()
{
    if (1)
        return 2;
    return 3;
}
int t44_main()
{
    if (2 - 1)
        return 2;
    return 3;
}
int t45_main()
{
    if (0)
        return 3;
    else
        return 4;
}
int t46_main()
{
    if (1)
        return 3;
    else
        return 4;
}
int t47_main()
{
    int a;
    a = 0;
    if (a == 0)
        if (a != 0)
            return 5;
        else
            return 7;
    return 9;
}
int t48_main()
{
    int i;
    i = 0;
    while (i < 10)
        i = i + 1;
    return i;
}
int t49_main()
{
    int i;
    int j;
    i = 0;
    j = 0;
    for (i = 0; i <= 10; i = i + 1)
        j = i + j;
    return j;
}
int t50_main()
{
    for (;;)
        return 3;
    return 5;
}
int t51_main()
{
    int i;
    i = 0;
    for (; i <= 10;)
        i = i + 1;
    return i;
}
int t52_main()
{
    int x;
    x = 0;
    while (x < 5)
        if (x < 10)
            x = x + 1;
    return x;
}

// ステップ13：ブロック
int t53_main()
{
    {
        1;
        {
            2;
        }
        return 3;
    }
}
int t54_main()
{
    {
        {
        }
        return 5;
    }
}
int t55_main()
{
    int i;
    i = 0;
    while (i < 10)
    {
        i = i + 1;
    }
    return i;
}
int t56_main()
{
    int i;
    int j;
    i = 0;
    j = 0;
    while (i <= 10)
    {
        j = i + j;
        i = i + 1;
    }
    return j;
}
int t57_main()
{
    int j;
    int i;
    j = 0;
    for (i = 0; i <= 3; i = i + 1)
    {
        j = j + i;
    }
    return j;
}
int t58_main()
{
    int a;
    a = 1;
    if (a)
    {
        a = a + 2;
        a = a * 2;
    }
    else
    {
        a = 100;
    }
    return a + 1;
}
int t59_main()
{
    int a;
    a = 0;
    if (a)
    {
        a = a + 2;
        a = a * 2;
    }
    else
    {
        a = 100;
    }
    return a;
}

// ステップ14：関数呼び出し
int t60_main() { return ret3(); }
int t61_main() { return ret5(); }
int t62_main() { return add(3, 5); }
int t63_main() { return sub(5, 3); }
int t64_main() { return add6(1, 2, 3, 4, 5, 6); }
int t65_main() { return add6(1, 2, add6(3, 4, 5, 6, 7, 8), 9, 10, 11); }
int t66_main() { return add6(1, 2, add6(3, add6(4, 5, 6, 7, 8, 9), 10, 11, 12, 13), 14, 15, 16); }
int t67_main()
{
    int a;
    int b;
    a = 3;
    b = add(a, 4);
    return b;
}
int t68_main() { return aligned(); }
int t69_main() { return (1 + aligned()) * 1; }
int t70_main() { return ((1 + (1 + aligned())) * 1) * 1; }

// ステップ15：関数の定義
int t71_ret32() { return 32; }
int t71_main() { return t71_ret32(); }
int t72_add2(int x, int y) { return x + y; }
int t72_main() { return t72_add2(3, 4); }
int t73_sub2(int x, int y) { return x - y; }
int t73_main() { return t73_sub2(4, 3); }
int t74_sum6(int a, int b, int c, int d, int e, int f) { return a + b + c + d + e + f; }
int t74_main() { return t74_sum6(1, 2, 3, 4, 5, 6); }
int t75_fib(int n)
{
    if (n <= 1)
        return n;
    return t75_fib(n - 1) + t75_fib(n - 2);
}
int t75_main() { return t75_fib(10); }
int t76_f(int x)
{
    int a;
    a = x * 2;
    return a;
}
int t76_main()
{
    int a;
    int b;
    a = 1;
    b = t76_f(5);
    return a + b;
}
int t77_g() { return (1 + aligned()) * 1; }
int t77_main() { return t77_g() + (1 + t77_g()) * 1; }
int t78_main() { return t78_later(); }
int t78_later() { return 3; }

// ステップ16：単項 & と単項 *
int t79_main()
{
    int x;
    int *y;
    x = 3;
    y = &x;
    return *y;
}
int t80_main()
{
    int x;
    int y;
    int *z;
    x = 3;
    y = 5;
    z = &y + 2;
    return *z;
}
int t81_main()
{
    int x;
    int y;
    int *z;
    x = 3;
    y = 5;
    z = &x - 2;
    return *z;
}
int t82_main()
{
    int x;
    int *y;
    x = 3;
    y = &x;
    *y = 7;
    return x;
}
int t83_main()
{
    int x;
    int *y;
    int **z;
    x = 3;
    y = &x;
    z = &y;
    **z = 9;
    return x;
}
int t84_main()
{
    int x;
    x = 3;
    return *&x;
}
int t85_set(int *p, int v)
{
    *p = v;
    return 0;
}
int t85_main()
{
    int x;
    x = 1;
    t85_set(&x, 6);
    return x;
}
int t86_swap(int *a, int *b)
{
    int t;
    t = *a;
    *a = *b;
    *b = t;
    return 0;
}
int t86_main()
{
    int x;
    int y;
    x = 3;
    y = 5;
    t86_swap(&x, &y);
    return x * 10 + y;
}

// ステップ17：int による宣言
int t87_main()
{
    int x;
    x = 3;
    return x;
}
int t88_main()
{
    int x;
    int y;
    x = 3;
    y = 4;
    return x + y;
}
int t89_main()
{
    int x;
    x = 5;
    {
        int y;
        y = x;
    }
    return x;
}
int t90_add2(int a, int b) { return a + b; }
int t90_main() { return t90_add2(3, 5); }
int t91_main()
{
    int foo_bar1;
    foo_bar1 = 6;
    return foo_bar1;
}

// ステップ18：ポインタ型
int t92_main()
{
    int x;
    int *y;
    y = &x;
    *y = 3;
    return x;
}
int t93_main()
{
    int x;
    int *y;
    int **z;
    x = 3;
    y = &x;
    z = &y;
    return **z;
}
int t94_main()
{
    int a;
    int *p;
    int **pp;
    a = 1;
    p = &a;
    pp = &p;
    *p = 5;
    **pp = **pp + 6;
    return a;
}
int t95_deref(int *p) { return *p; }
int t95_main()
{
    int x;
    x = 4;
    return t95_deref(&x);
}
int t96_set2(int **pp, int v)
{
    **pp = v;
    return 0;
}
int t96_main()
{
    int x;
    int *p;
    p = &x;
    t96_set2(&p, 8);
    return x;
}

// ステップ19：ポインタの加算と減算
int t97_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    int *q;
    q = p + 2;
    return *q;
}
int t98_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    int *q;
    q = p + 3;
    return *q;
}
int t99_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    int *q;
    q = p + 3;
    q = q - 2;
    return *q;
}
int t100_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    return *(p + 3);
}
int t101_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    return *(2 + p);
}
int t102_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    return (p + 3) - p;
}
int t103_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    *(p + 1) = 10;
    return *p + *(p + 1) + *(p + 2);
}
int t104_main()
{
    int x;
    x = -3;
    return x + 5;
}
int t105_main() { return sub(3, 5) < 0; }

// ステップ20：sizeof
int t106_main()
{
    int x;
    return sizeof(x);
}
int t107_main()
{
    int *y;
    return sizeof(y);
}
int t108_main()
{
    int x;
    return sizeof(x + 3);
}
int t109_main()
{
    int *y;
    return sizeof(*y);
}
int t110_main()
{
    int *y;
    return sizeof(y + 3);
}
int t111_main()
{
    int x;
    return sizeof(&x);
}
int t112_main()
{
    int **z;
    return sizeof(*z);
}
int t113_main() { return sizeof(1); }
int t114_main() { return sizeof(sizeof(1)); }
int t115_main()
{
    int x;
    return sizeof x;
}
int t116_main()
{
    int x;
    x = 1;
    sizeof(x = 3);
    return x;
}

// ステップ21：配列
int t117_main()
{
    int a[2];
    *a = 1;
    *(a + 1) = 2;
    int *p;
    p = a;
    return *p + *(p + 1);
}
int t118_main()
{
    int a[10];
    return sizeof(a);
}
int t119_main()
{
    int *b[3];
    return sizeof(b);
}
int t120_main()
{
    int a[4];
    return sizeof(*a);
}
int t121_main()
{
    int a[3];
    *a = 1;
    *(a + 1) = 2;
    *(a + 2) = 3;
    return *(a + 2) - *a;
}
int t122_main()
{
    int a[5];
    int i;
    for (i = 0; i < 5; i = i + 1)
        *(a + i) = i * i;
    return *(a + 4);
}
int t123_main()
{
    int a[4];
    return (a + 3) - a;
}
int t124_main()
{
    int a[3];
    int x;
    x = 7;
    *(a + 2) = 100;
    return x;
}
int t125_main()
{
    int x;
    int a[3];
    x = 7;
    *(a + 2) = 100;
    return x;
}
int t126_sum(int *p, int n)
{
    int s;
    int i;
    s = 0;
    for (i = 0; i < n; i = i + 1)
        s = s + *(p + i);
    return s;
}
int t126_main()
{
    int a[4];
    *a = 1;
    *(a + 1) = 2;
    *(a + 2) = 3;
    *(a + 3) = 4;
    return t126_sum(a, 4);
}

// ステップ22：配列の添字
int t127_main()
{
    int a[3];
    a[0] = 1;
    a[1] = 2;
    a[2] = 3;
    return a[0] + a[1] + a[2];
}
int t128_main()
{
    int a[5];
    int i;
    for (i = 0; i < 5; i = i + 1)
        a[i] = i * i;
    return a[4];
}
int t129_main()
{
    int a[2];
    a[1] = 7;
    return 1 [a];
}
int t130_main()
{
    int *p;
    alloc4(&p, 1, 2, 4, 8);
    return p[2];
}
int t131_main()
{
    int *b[2];
    int x;
    int y;
    x = 3;
    y = 5;
    b[0] = &x;
    b[1] = &y;
    return *b[0] + *b[1];
}
int t132_main()
{
    int a[3];
    return sizeof(a[0]);
}
int t133_main()
{
    int a[4];
    a[3] = 9;
    return *(a + 3) == a[3];
}
int t134_main()
{
    int a[4];
    a[0] = 1;
    a[1] = 3;
    a[a[0]] = 5;
    return a[1];
}
int t135_main()
{
    int a[3];
    int x;
    x = 7;
    a[2] = 100;
    return x;
}
int t136_sum(int *p, int n)
{
    int s;
    int i;
    s = 0;
    for (i = 0; i < n; i = i + 1)
        s = s + p[i];
    return s;
}
int t136_main()
{
    int a[4];
    a[0] = 1;
    a[1] = 2;
    a[2] = 3;
    a[3] = 4;
    return t136_sum(a, 4);
}

// ステップ23：グローバル変数
int t137_x;
int t137_main() { return t137_x; }
int t138_x;
int t138_main()
{
    t138_x = 3;
    return t138_x;
}
int t139_x;
int t139_y;
int t139_main()
{
    t139_x = 3;
    t139_y = 4;
    return t139_x + t139_y;
}
int t140_x[4];
int t140_main()
{
    t140_x[0] = 0;
    t140_x[1] = 1;
    t140_x[2] = 2;
    t140_x[3] = 3;
    return t140_x[3];
}
int t141_x[4];
int t141_main() { return sizeof(t141_x); }
int *t142_p;
int t142_x;
int t142_main()
{
    t142_x = 5;
    t142_p = &t142_x;
    return *t142_p;
}
int t143_g;
int t143_set(int v)
{
    t143_g = v;
    return 0;
}
int t143_main()
{
    t143_set(9);
    return t143_g;
}
int t144_x;
int t144_main()
{
    int t144_x;
    t144_x = 2;
    return t144_x;
}
int t145_count;
int t145_inc()
{
    t145_count = t145_count + 1;
    return t145_count;
}
int t145_main()
{
    t145_inc();
    t145_inc();
    return t145_inc();
}
int t146_a[3];
int t146_main()
{
    int i;
    for (i = 0; i < 3; i = i + 1)
        t146_a[i] = i + 1;
    return t146_a[0] + t146_a[1] + t146_a[2];
}

// ステップ24：char 型
int t147_main()
{
    char x[3];
    x[0] = -1;
    x[1] = 2;
    int y;
    y = 4;
    return x[0] + y;
}
int t148_main()
{
    char x[3];
    x[0] = -1;
    x[1] = 2;
    int y;
    y = 4;
    return y - x[0];
}
int t149_main()
{
    char x;
    return sizeof(x);
}
int t150_main()
{
    char x[10];
    return sizeof(x);
}
int t151_main()
{
    char *p;
    return sizeof(p);
}
int t152_main()
{
    char x;
    x = 300;
    return x;
}
int t153_main()
{
    char x;
    x = 200;
    return x < 0;
}
int t154_main()
{
    char c[2];
    int x;
    x = 100;
    c[0] = 1;
    c[1] = 2;
    return x + c[1];
}
int t155_main()
{
    char *p;
    char c[4];
    p = c;
    p[2] = 9;
    return c[2];
}
int t156_f(char a, char b, char c) { return a - b - c; }
int t156_main() { return t156_f(7, 3, 3); }
char t157_g[4];
int t157_main()
{
    t157_g[0] = 1;
    t157_g[3] = 5;
    return t157_g[0] + t157_g[3];
}
char *t158_p;
int t158_main()
{
    char c[2];
    c[1] = 3;
    t158_p = c;
    return *(t158_p + 1);
}

// ステップ25：文字列リテラル
int t159_main() { return "abc"[0]; }
int t160_main() { return "abc"[1]; }
int t161_main() { return "abc"[2]; }
int t162_main() { return "abc"[3]; }
int t163_main() { return sizeof("abc"); }
int t164_main()
{
    char *s;
    s = "hello";
    return s[4];
}
int t165_main()
{
    char *a;
    char *b;
    a = "x";
    b = "y";
    return b[0] - a[0];
}
int t166_main() { return "\n"[0]; }
int t167_main() { return "\""[0]; }
int t168_main() { return "\\"[0]; }
int t169_len(char *s)
{
    int n;
    n = 0;
    while (s[n])
        n = n + 1;
    return n;
}
int t169_main() { return t169_len("hello, world"); }
int t170_main() { return printf("hello\n"); }

// ステップ27：コメント
int t171_main()
{
    // return 5;
    return 3;
}
int t172_main() { /* return 1; */ return 2; }
int t173_main() {             /* 複数行の
              コメント */ return 4; }
int t174_main() { return 5; } // 最後の行のコメント
int t175_main() { return 2 /* 途中 */ * 3; }
int t176_main() { return 4 / 2 / 2; }
int t177_main()
{
    char *s;
    s = "/* not a comment */";
    return s[1];
}
int t178_main()
{
    char *s;
    s = "//";
    return s[0];
}

// ステップ28：終了コード（0〜255）では確かめられなかった値
int big() { return 1000; }
int neg() { return -3; }

int main()
{

    // ステップ1〜4
    assert(0, t1_main(), "int main() { 0; }");
    assert(42, t2_main(), "int main() { 42; }");
    assert(21, t3_main(), "int main() { 5+20-4; }");
    assert(41, t4_main(), "int main() { 12 + 34 - 5 ; }");

    // ステップ5：四則演算とかっこ
    assert(47, t5_main(), "int main() { 5+6*7; }");
    assert(15, t6_main(), "int main() { 5*(9-6); }");
    assert(4, t7_main(), "int main() { (3+5)/2; }");

    // ステップ6：単項 + と -
    assert(10, t8_main(), "int main() { -10+20; }");
    assert(10, t9_main(), "int main() { - -10; }");
    assert(10, t10_main(), "int main() { - - +10; }");

    // ステップ7：比較演算子
    assert(0, t11_main(), "int main() { 0==1; }");
    assert(1, t12_main(), "int main() { 42==42; }");
    assert(1, t13_main(), "int main() { 0!=1; }");
    assert(0, t14_main(), "int main() { 42!=42; }");
    assert(1, t15_main(), "int main() { 0<1; }");
    assert(0, t16_main(), "int main() { 1<1; }");
    assert(0, t17_main(), "int main() { 2<1; }");
    assert(1, t18_main(), "int main() { 0<=1; }");
    assert(1, t19_main(), "int main() { 1<=1; }");
    assert(0, t20_main(), "int main() { 2<=1; }");
    assert(1, t21_main(), "int main() { 1>0; }");
    assert(0, t22_main(), "int main() { 1>1; }");
    assert(0, t23_main(), "int main() { 1>2; }");
    assert(1, t24_main(), "int main() { 1>=0; }");
    assert(1, t25_main(), "int main() { 1>=1; }");
    assert(0, t26_main(), "int main() { 1>=2; }");

    // ステップ9：1文字のローカル変数
    assert(3, t27_main(), "int main() { int a; a=3; a; }");
    assert(8, t28_main(), "int main() { int a; int z; a=3; z=5; a+z; }");
    assert(6, t29_main(), "int main() { int a; int b; a=b=3; a+b; }");
    assert(14, t30_main(), "int main() { int a; int b; a=3; b=5*6-8; a+b/2; }");

    // ステップ10：複数文字のローカル変数
    assert(3, t31_main(), "int main() { int foo; foo=3; foo; }");
    assert(6, t32_main(), "int main() { int foo; int bar; foo=1; bar=2+3; foo+bar; }");
    assert(8, t33_main(), "int main() { int foo123; int bar; foo123=3; bar=5; foo123+bar; }");
    assert(7, t34_main(), "int main() { int _x; int Hello_World; _x=3; Hello_World=4; _x+Hello_World; }");
    assert(30, t35_main(), "int main() { int a; int b; int c; int d; int e; int f; int g; int h; int i; int j; int k; int l; int m; int n; int o; int p; int q; int r; int s; int t; int u; int v; int w; int x; int y; int z; int aa; int ab; int ac; int ad; a=1;b=2;c=3;d=4;e=5;f=6;g=7;h=8;i=9;j=10;k=11;l=12;m=13;n=14;o=15;p=16;q=17;r=18;s=19;t=20;u=21;v=22;w=23;x=24;y=25;z=26;aa=27;ab=28;ac=29;ad=30; ad; }");

    // ステップ11：return
    assert(1, t36_main(), "int main() { return 1; 2; 3; }");
    assert(2, t37_main(), "int main() { 1; return 2; 3; }");
    assert(3, t38_main(), "int main() { 1; 2; return 3; }");
    assert(5, t39_main(), "int main() { int a; a=5; return a; }");
    assert(3, t40_main(), "int main() { int returnx; returnx=3; return returnx; }");

    // ステップ12：if / while / for
    assert(3, t41_main(), "int main() { if (0) return 2; return 3; }");
    assert(3, t42_main(), "int main() { if (1-1) return 2; return 3; }");
    assert(2, t43_main(), "int main() { if (1) return 2; return 3; }");
    assert(2, t44_main(), "int main() { if (2-1) return 2; return 3; }");
    assert(4, t45_main(), "int main() { if (0) return 3; else return 4; }");
    assert(3, t46_main(), "int main() { if (1) return 3; else return 4; }");
    assert(7, t47_main(), "int main() { int a; a=0; if (a==0) if (a!=0) return 5; else return 7; return 9; }");
    assert(10, t48_main(), "int main() { int i; i=0; while(i<10) i=i+1; return i; }");
    assert(55, t49_main(), "int main() { int i; int j; i=0; j=0; for (i=0; i<=10; i=i+1) j=i+j; return j; }");
    assert(3, t50_main(), "int main() { for (;;) return 3; return 5; }");
    assert(11, t51_main(), "int main() { int i; i=0; for (; i<=10;) i=i+1; return i; }");
    assert(5, t52_main(), "int main() { int x; x=0; while (x<5) if (x<10) x=x+1; return x; }");

    // ステップ13：ブロック
    assert(3, t53_main(), "int main() { {1; {2;} return 3;} }");
    assert(5, t54_main(), "int main() { { {} return 5; } }");
    assert(10, t55_main(), "int main() { int i; i=0; while(i<10) { i=i+1; } return i; }");
    assert(55, t56_main(), "int main() { int i; int j; i=0; j=0; while(i<=10) { j=i+j; i=i+1; } return j; }");
    assert(6, t57_main(), "int main() { int j; int i; j=0; for (i=0; i<=3; i=i+1) { j=j+i; } return j; }");
    assert(7, t58_main(), "int main() { int a; a=1; if (a) { a=a+2; a=a*2; } else { a=100; } return a+1; }");
    assert(100, t59_main(), "int main() { int a; a=0; if (a) { a=a+2; a=a*2; } else { a=100; } return a; }");

    // ステップ14：関数呼び出し
    assert(3, t60_main(), "int main() { return ret3(); }");
    assert(5, t61_main(), "int main() { return ret5(); }");
    assert(8, t62_main(), "int main() { return add(3, 5); }");
    assert(2, t63_main(), "int main() { return sub(5, 3); }");
    assert(21, t64_main(), "int main() { return add6(1, 2, 3, 4, 5, 6); }");
    assert(66, t65_main(), "int main() { return add6(1, 2, add6(3, 4, 5, 6, 7, 8), 9, 10, 11); }");
    assert(136, t66_main(), "int main() { return add6(1, 2, add6(3, add6(4, 5, 6, 7, 8, 9), 10, 11, 12, 13), 14, 15, 16); }");
    assert(7, t67_main(), "int main() { int a; int b; a=3; b=add(a, 4); return b; }");
    assert(1, t68_main(), "int main() { return aligned(); }");
    assert(2, t69_main(), "int main() { return (1 + aligned()) * 1; }");
    assert(3, t70_main(), "int main() { return ((1 + (1 + aligned())) * 1) * 1; }");

    // ステップ15：関数の定義
    assert(32, t71_main(), "int ret32() { return 32; } int main() { return ret32(); }");
    assert(7, t72_main(), "int add2(int x, int y) { return x + y; } int main() { return add2(3, 4); }");
    assert(1, t73_main(), "int sub2(int x, int y) { return x - y; } int main() { return sub2(4, 3); }");
    assert(21, t74_main(), "int sum6(int a, int b, int c, int d, int e, int f) { return a + b + c + d + e + f; } int main() { return sum6(1, 2, 3, 4, 5, 6); }");
    assert(55, t75_main(), "int fib(int n) { if (n <= 1) return n; return fib(n - 1) + fib(n - 2); } int main() { return fib(10); }");
    assert(11, t76_main(), "int f(int x) { int a; a = x * 2; return a; } int main() { int a; int b; a = 1; b = f(5); return a + b; }");
    assert(5, t77_main(), "int g() { return (1 + aligned()) * 1; } int main() { return g() + (1 + g()) * 1; }");
    assert(3, t78_main(), "int main() { return later(); } int later() { return 3; }");

    // ステップ16：単項 & と単項 *
    assert(3, t79_main(), "int main() { int x; int *y; x=3; y=&x; return *y; }");
    assert(3, t80_main(), "int main() { int x; int y; int *z; x=3; y=5; z=&y+2; return *z; }");
    assert(5, t81_main(), "int main() { int x; int y; int *z; x=3; y=5; z=&x-2; return *z; }");
    assert(7, t82_main(), "int main() { int x; int *y; x=3; y=&x; *y=7; return x; }");
    assert(9, t83_main(), "int main() { int x; int *y; int **z; x=3; y=&x; z=&y; **z=9; return x; }");
    assert(3, t84_main(), "int main() { int x; x=3; return *&x; }");
    assert(6, t85_main(), "int set(int *p, int v) { *p = v; return 0; } int main() { int x; x=1; set(&x, 6); return x; }");
    assert(53, t86_main(), "int swap(int *a, int *b) { int t; t=*a; *a=*b; *b=t; return 0; } int main() { int x; int y; x=3; y=5; swap(&x, &y); return x*10+y; }");

    // ステップ17：int による宣言
    assert(3, t87_main(), "int main() { int x; x=3; return x; }");
    assert(7, t88_main(), "int main() { int x; int y; x=3; y=4; return x+y; }");
    assert(5, t89_main(), "int main() { int x; x=5; { int y; y=x; } return x; }");
    assert(8, t90_main(), "int add2(int a, int b) { return a+b; } int main() { return add2(3, 5); }");
    assert(6, t91_main(), "int main() { int foo_bar1; foo_bar1=6; return foo_bar1; }");

    // ステップ18：ポインタ型
    assert(3, t92_main(), "int main() { int x; int *y; y=&x; *y=3; return x; }");
    assert(3, t93_main(), "int main() { int x; int *y; int **z; x=3; y=&x; z=&y; return **z; }");
    assert(11, t94_main(), "int main() { int a; int *p; int **pp; a=1; p=&a; pp=&p; *p=5; **pp=**pp+6; return a; }");
    assert(4, t95_main(), "int deref(int *p) { return *p; } int main() { int x; x=4; return deref(&x); }");
    assert(8, t96_main(), "int set2(int **pp, int v) { **pp = v; return 0; } int main() { int x; int *p; p=&x; set2(&p, 8); return x; }");

    // ステップ19：ポインタの加算と減算
    assert(4, t97_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); int *q; q = p + 2; return *q; }");
    assert(8, t98_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); int *q; q = p + 3; return *q; }");
    assert(2, t99_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); int *q; q = p + 3; q = q - 2; return *q; }");
    assert(8, t100_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); return *(p + 3); }");
    assert(4, t101_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); return *(2 + p); }");
    assert(3, t102_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); return (p + 3) - p; }");
    assert(15, t103_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); *(p + 1) = 10; return *p + *(p + 1) + *(p + 2); }");
    assert(2, t104_main(), "int main() { int x; x = -3; return x + 5; }");
    assert(1, t105_main(), "int main() { return sub(3, 5) < 0; }");

    // ステップ20：sizeof
    assert(4, t106_main(), "int main() { int x; return sizeof(x); }");
    assert(8, t107_main(), "int main() { int *y; return sizeof(y); }");
    assert(4, t108_main(), "int main() { int x; return sizeof(x + 3); }");
    assert(4, t109_main(), "int main() { int *y; return sizeof(*y); }");
    assert(8, t110_main(), "int main() { int *y; return sizeof(y + 3); }");
    assert(8, t111_main(), "int main() { int x; return sizeof(&x); }");
    assert(8, t112_main(), "int main() { int **z; return sizeof(*z); }");
    assert(4, t113_main(), "int main() { return sizeof(1); }");
    assert(4, t114_main(), "int main() { return sizeof(sizeof(1)); }");
    assert(4, t115_main(), "int main() { int x; return sizeof x; }");
    assert(1, t116_main(), "int main() { int x; x = 1; sizeof(x = 3); return x; }");

    // ステップ21：配列
    assert(3, t117_main(), "int main() { int a[2]; *a = 1; *(a + 1) = 2; int *p; p = a; return *p + *(p + 1); }");
    assert(40, t118_main(), "int main() { int a[10]; return sizeof(a); }");
    assert(24, t119_main(), "int main() { int *b[3]; return sizeof(b); }");
    assert(4, t120_main(), "int main() { int a[4]; return sizeof(*a); }");
    assert(2, t121_main(), "int main() { int a[3]; *a = 1; *(a + 1) = 2; *(a + 2) = 3; return *(a + 2) - *a; }");
    assert(16, t122_main(), "int main() { int a[5]; int i; for (i = 0; i < 5; i = i + 1) *(a + i) = i * i; return *(a + 4); }");
    assert(3, t123_main(), "int main() { int a[4]; return (a + 3) - a; }");
    assert(7, t124_main(), "int main() { int a[3]; int x; x = 7; *(a + 2) = 100; return x; }");
    assert(7, t125_main(), "int main() { int x; int a[3]; x = 7; *(a + 2) = 100; return x; }");
    assert(10, t126_main(), "int sum(int *p, int n) { int s; int i; s = 0; for (i = 0; i < n; i = i + 1) s = s + *(p + i); return s; } int main() { int a[4]; *a = 1; *(a + 1) = 2; *(a + 2) = 3; *(a + 3) = 4; return sum(a, 4); }");

    // ステップ22：配列の添字
    assert(6, t127_main(), "int main() { int a[3]; a[0] = 1; a[1] = 2; a[2] = 3; return a[0] + a[1] + a[2]; }");
    assert(16, t128_main(), "int main() { int a[5]; int i; for (i = 0; i < 5; i = i + 1) a[i] = i * i; return a[4]; }");
    assert(7, t129_main(), "int main() { int a[2]; a[1] = 7; return 1[a]; }");
    assert(4, t130_main(), "int main() { int *p; alloc4(&p, 1, 2, 4, 8); return p[2]; }");
    assert(8, t131_main(), "int main() { int *b[2]; int x; int y; x = 3; y = 5; b[0] = &x; b[1] = &y; return *b[0] + *b[1]; }");
    assert(4, t132_main(), "int main() { int a[3]; return sizeof(a[0]); }");
    assert(1, t133_main(), "int main() { int a[4]; a[3] = 9; return *(a + 3) == a[3]; }");
    assert(5, t134_main(), "int main() { int a[4]; a[0] = 1; a[1] = 3; a[a[0]] = 5; return a[1]; }");
    assert(7, t135_main(), "int main() { int a[3]; int x; x = 7; a[2] = 100; return x; }");
    assert(10, t136_main(), "int sum(int *p, int n) { int s; int i; s = 0; for (i = 0; i < n; i = i + 1) s = s + p[i]; return s; } int main() { int a[4]; a[0] = 1; a[1] = 2; a[2] = 3; a[3] = 4; return sum(a, 4); }");

    // ステップ23：グローバル変数
    assert(0, t137_main(), "int x; int main() { return x; }");
    assert(3, t138_main(), "int x; int main() { x = 3; return x; }");
    assert(7, t139_main(), "int x; int y; int main() { x = 3; y = 4; return x + y; }");
    assert(3, t140_main(), "int x[4]; int main() { x[0] = 0; x[1] = 1; x[2] = 2; x[3] = 3; return x[3]; }");
    assert(16, t141_main(), "int x[4]; int main() { return sizeof(x); }");
    assert(5, t142_main(), "int *p; int x; int main() { x = 5; p = &x; return *p; }");
    assert(9, t143_main(), "int g; int set(int v) { g = v; return 0; } int main() { set(9); return g; }");
    assert(2, t144_main(), "int x; int main() { int x; x = 2; return x; }");
    assert(3, t145_main(), "int count; int inc() { count = count + 1; return count; } int main() { inc(); inc(); return inc(); }");
    assert(6, t146_main(), "int a[3]; int main() { int i; for (i = 0; i < 3; i = i + 1) a[i] = i + 1; return a[0] + a[1] + a[2]; }");

    // ステップ24：char 型
    assert(3, t147_main(), "int main() { char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; return x[0] + y; }");
    assert(5, t148_main(), "int main() { char x[3]; x[0] = -1; x[1] = 2; int y; y = 4; return y - x[0]; }");
    assert(1, t149_main(), "int main() { char x; return sizeof(x); }");
    assert(10, t150_main(), "int main() { char x[10]; return sizeof(x); }");
    assert(8, t151_main(), "int main() { char *p; return sizeof(p); }");
    assert(44, t152_main(), "int main() { char x; x = 300; return x; }");
    assert(1, t153_main(), "int main() { char x; x = 200; return x < 0; }");
    assert(102, t154_main(), "int main() { char c[2]; int x; x = 100; c[0] = 1; c[1] = 2; return x + c[1]; }");
    assert(9, t155_main(), "int main() { char *p; char c[4]; p = c; p[2] = 9; return c[2]; }");
    assert(1, t156_main(), "int f(char a, char b, char c) { return a - b - c; } int main() { return f(7, 3, 3); }");
    assert(6, t157_main(), "char g[4]; int main() { g[0] = 1; g[3] = 5; return g[0] + g[3]; }");
    assert(3, t158_main(), "char *p; int main() { char c[2]; c[1] = 3; p = c; return *(p + 1); }");

    // ステップ25：文字列リテラル
    assert(97, t159_main(), "int main() { return \"abc\"[0]; }");
    assert(98, t160_main(), "int main() { return \"abc\"[1]; }");
    assert(99, t161_main(), "int main() { return \"abc\"[2]; }");
    assert(0, t162_main(), "int main() { return \"abc\"[3]; }");
    assert(4, t163_main(), "int main() { return sizeof(\"abc\"); }");
    assert(111, t164_main(), "int main() { char *s; s = \"hello\"; return s[4]; }");
    assert(1, t165_main(), "int main() { char *a; char *b; a = \"x\"; b = \"y\"; return b[0] - a[0]; }");
    assert(10, t166_main(), "int main() { return \"\\n\"[0]; }");
    assert(34, t167_main(), "int main() { return \"\\\"\"[0]; }");
    assert(92, t168_main(), "int main() { return \"\\\\\"[0]; }");
    assert(12, t169_main(), "int len(char *s) { int n; n = 0; while (s[n]) n = n + 1; return n; } int main() { return len(\"hello, world\"); }");
    assert(6, t170_main(), "int main() { return printf(\"hello\\n\"); }");

    // ステップ27：コメント
    assert(3, t171_main(), "int main() {\n  // return 5;\n  return 3;\n}");
    assert(2, t172_main(), "int main() { /* return 1; */ return 2; }");
    assert(4, t173_main(), "int main() { /* 複数行の\n  コメント */ return 4; }");
    assert(5, t174_main(), "int main() { return 5; } // 最後の行のコメント");
    assert(6, t175_main(), "int main() { return 2 /* 途中 */ * 3; }");
    assert(1, t176_main(), "int main() { return 4 / 2 / 2; }");
    assert(42, t177_main(), "int main() { char *s; s = \"/* not a comment */\"; return s[1]; }");
    assert(47, t178_main(), "int main() { char *s; s = \"//\"; return s[0]; }");

    // ステップ28：終了コード（0〜255）では確かめられなかった値
    assert(1000, big(), "big()");
    assert(-3, neg(), "neg()");
    assert(65536, 256 * 256, "256 * 256");
    assert(-3, 7 / -2, "7 / -2");
    assert(-1, sub(2, 3), "sub(2, 3)");
    assert(300, add(100, 200), "add(100, 200)");
    assert(1, -1 < 0, "-1 < 0");

    printf("OK\n");
    return 0;
}