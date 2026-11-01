package p;

class DescProbe2 {
    void m(java.util.List<String> list) {
        for (int i = 0; i < 1; foo(1), bar(2), baz(3)) { }
        for (int i = 0; ; this.foo(1), a.b.c(2), new Object(), ++i) { }
        for (a = b = 0, c = d = 1, e = 2; ; ) { }
        for (foo(1, 2, 3), bar(4, 5), baz(6); ; ) { }
        for (int i = 0, j = (1 + 2), k = (3); ; ) { }
        for (Runnable r = () -> { }, s = () -> { }, t = null; ; ) { }
        for (int i = 0; ; i += 1, j -= 1, k *= 2) { }
        for (int i = 0; ; --i, --j, --k) { }
        for (int i = 0; ; (i)++, j++, k++) { }
        for (int i = 0; ; i++, j++, k++) { for (int x = 0, y = 1, z = 2; ; ) { } }
        for (String s : list) { for (int x = 0, y = 1, z = 2; ; ) { } }
        for (final var a = 1, b = 2, c = 3; ; ) { }
        for (Map<String, Integer> m = null, n = null, o = null; ; ) { }
        for (int[] a = {1, 2, 3}, b = {4, 5, 6}; ; ) { }
        for (int i = x ? 1 : 2, j = 1, k = 2; ; ) { }
        for (int i = 0, j = 1, k = 2; ; ) for (int p = 0, q = 1, r = 2; ; ) { }
        Runnable r = () -> { for (int i = 0, j = 1, k = 2; ; ) { } };
    }
}
