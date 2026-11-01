package p;

import java.util.function.Function;

public class HidProbe {
    private int a;
    private static int s;
    private int b, c;
    int[] arr;

    HidProbe(int a, int s) {
        int b = 1;
    }

    void m(int a, int s, String q) {
        int b;
        int c = 0, d = 1;
        for (int a = 0; a < 1; a++) { }
        for (int s : arr) { }
        try { } catch (Exception a) { }
        try (java.io.Reader b = null) { } catch (Exception e) { }
        Function<Integer, Integer> f = a -> a;
        Function<Integer, Integer> g = (a) -> a;
        Function<Integer, Integer> h = (Integer a) -> a;
        java.util.function.BiFunction<Integer, Integer, Integer> k = (c, s) -> c;
        Object o = null;
        if (o instanceof String a) { }
        if (o instanceof Integer q) { }
        { int a = 2; }
        switch (o) { case String b -> { } default -> { } }
    }

    static void sm(int a, int s) {
        int b;
        int s2;
    }

    static {
        int a;
        int s;
    }

    {
        int a;
        int s;
    }

    void setA(int a) { this.a = a; }
    HidProbe setB(int b) { this.b = b; return this; }
    HidProbe setC(int c) { this.c = c; return null; }
    void setS(int s) { }
    static void setSs(int s) { HidProbe.s = s; }
    void seta(int a) { }
    void set(int a) { }
    int setD(int a) { return a; }
    void setA2(int a, int b) { }
    void setA3(final int a) { }
    void setA4(int... a) { }
    void setA5(int a) throws Exception { }
    <T> void setA6(int a) { }
    void setA7(int a) { int x = a; }

    abstract static class Abs {
        int a;
        abstract void m(int a);
        void n(int a) { }
    }

    interface I {
        void m(int a);
        default void d(int a) { }
        static void st(int a) { }
    }

    static class Nested {
        int nf;
        static int nsf;
        void m(int a, int s, int nf, int nsf) { }
        static void sm(int a, int s, int nf, int nsf) { }
    }

    class Inner {
        int inf;
        void m(int a, int s, int inf, int b) { }
        static void sm(int a, int s, int inf) { }
    }

    void local() {
        class L {
            int lf;
            void m(int a, int lf, int s) { }
        }
        Object o = new Object() {
            int af;
            void m(int a, int af, int s) { }
        };
    }

    enum E {
        X(1), Y(2);
        private final int ev;
        E(int ev) { this.ev = ev; }
        void m(int ev, int X) { }
        static void sm(int ev, int X) { }
    }

    record R(int rc, String rd) {
        static int rs;
        R {
            int rc2;
        }
        R(int rc, String rd, int extra) {
            this(rc, rd);
        }
        void m(int rc, int rs, int rd) { }
        static void sm(int rc, int rs) { }
    }

    record R2(int x) {
        R2(int x) { this.x = x; }
    }

    void patterns(Object o) {
        if (o instanceof R(int rc, String rd)) { }
        if (o instanceof R(var a, var s)) { }
        if (!(o instanceof String a)) { return; }
    }

    int field2 = 5;
    void sh() {
        int a = a;
        Runnable r = () -> { int field2 = 1; };
        Function<Integer, Integer> fn = field2 -> 1;
    }
    private int Z;
    void caseSens(int z, int Z) { }
    int x1;
    void outer() {
        int x1;
        {
            int x1;
        }
    }
}
