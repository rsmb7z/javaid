package p;

public class DescProbe {
    void m(int n) {
        for (int i = 0; i < n; i++) { }
        for (int i = 0, j = 1; i < n; i++, j++) { }
        for (int i = 0, j = 1, k = 2; i < n; i++, j++, k++) { }
        for (int i = 0, j = 1, k = 2, l = 3; ; ) { }
        for (; ; i++, j++, k++, l++) { }
        for (i = 0, j = 1, k = 2; ; ) { }
        for (i = 0, j = 1, k = 2, l = 3; i < 1; i++, j++, k++) { }
        for (;;) { }
        for (int i = f(1, 2, 3, 4), j = g(1, 2, 3); ; ) { }
        for (int[] a = {1, 2, 3, 4}; ; ) { }
        for (int i = 0, j = 1, k = 2; ; i = h(1, 2, 3, 4)) { }
        for (int a = 0,
                 b = 1,
                 c = 2; ; ) { }
        for (;
             ;
             i++,
             j++,
             k++) { }
        for (Object o : list) { }
        for (final int i = 0, j = 1, k = 2; ; ) { }
        for (@SuppressWarnings("x") int i = 0, j = 1, k = 2; ; ) { }
    }
}
