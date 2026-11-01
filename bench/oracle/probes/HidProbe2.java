package p;

import java.util.function.Function;
import java.util.function.Supplier;

public class HidProbe2<T> {
    private int a;
    private static int s;
    private int xValue;
    private T t;
    private final Function<Integer, Integer> fld = a -> a + 1;
    private static final Function<Integer, Integer> sfld = a -> a;
    private static Supplier<Object> sup = () -> new Object() {
        void m(int a, int s) { }
    };
    private Supplier<Object> isup = () -> new Object() {
        void m(int a, int s) { }
    };

    static void staticAnon() {
        Object o = new Object() {
            void m(int a, int s) { }
        };
        new Object() {
            void m(int a, int s) { }
        };
        Runnable r = () -> {
            Object x = new Object() {
                void m(int a) { }
            };
        };
    }

    static void staticLocalClass() {
        class L {
            L(int a, int s) { }
            void m(int a, int s) { }
            static void sm(int a, int s) { }
        }
        int a;
        Function<Integer, Integer> f = a2 -> a2;
        Function<Integer, Integer> g = s -> s;
        Function<Integer, Integer> h = a -> a;
    }

    void instLocalClass() {
        class L {
            L(int a, int s) { }
            void m(int a, int s) { }
            static void sm(int a, int s) { }
        }
    }

    static int sret;
    static Function<Integer, Integer> sf = a -> a;
    Function<Integer, Integer> nf = a -> a;
    static { Function<Integer, Integer> f = a -> a; }
    { Function<Integer, Integer> f = a -> a; }

    HidProbe2<T> setT(T t) { this.t = t; return this; }
    HidProbe2<T> setA(int a) { return this; }
    HidProbe2 setXValue(int xValue) { return this; }
    void setxValue(int xValue) { }
    void setXvalue(int xValue) { }
    p.HidProbe2<T> setS(int s) { return null; }
    HidProbe2<T>[] setA2(int a) { return null; }
    Object setA3(int a) { return this; }
    static HidProbe2<?> setS2(int s) { return null; }
    void setT2(T t) { }
    void set(@Deprecated int a) { }
    @Deprecated void setA9(@Deprecated final int a) { }

    enum E {
        X { int bodyField; void m(int bodyField, int a) { } },
        Y(1) { void n(int ev, int s) { } };
        int ev;
        static int sev;
        E() { }
        E(int ev) { }
        void m(int ev, int sev, int a, int s) { }
    }

    static class SN {
        int snf;
        void m(int a, int snf, int s) { }
        class SNI {
            void m(int a, int snf, int s) { }
        }
    }

    class IN {
        int inf;
        void m(int a, int inf, int s) { }
        class INI {
            void m(int a, int inf, int s) { }
        }
        static class INS {
            void m(int a, int inf, int s) { }
        }
    }

    interface I {
        int ifield = 0;
        default void d(int a, int ifield) {
            Function<Integer, Integer> f = a -> a;
            Function<Integer, Integer> g = (a) -> a;
            Function<Integer, Integer> h = (Integer a) -> a;
            int a2;
            Object o = new Object() { int a; void m(int a, int s) { } };
        }
        class C { int cf; void m(int a, int cf, int s) { } }
    }

    @interface Ann {
        class AC { int acf; void m(int acf, int a) { } }
    }

    void switches(Object o) {
        switch (o) {
            case HidProbe2<?> a -> { }
            case String s when s.isEmpty() -> { }
            case Integer t -> { }
            default -> { }
        }
        switch (1) {
            case 1:
                int a = 0;
                break;
            default:
                int s;
        }
        for (int a = 0, s = 1; ; ) { }
    }

    void tryResources() throws Exception {
        try (java.io.Reader a = null; java.io.Reader s = null) { }
        try (java.io.Reader t = null) { } catch (RuntimeException | Error a) { }
    }

    void receiver(HidProbe2<T> this, int a) { }

    record Rec(int a, int s) {
        Rec {
            int t;
        }
        Rec(int a) { this(a, 1); }
        void m(int a, int s) { }
    }

    abstract class Abs {
        abstract void m(int a);
        abstract void setA(int a);
        void n(int a) { }
        Abs(int a) { }
    }

    void generic() {
        this.<String>gen("x");
    }
    <U> void gen(U a) { }
    void genericParam(java.util.List<String> a, String... s) { }
    void arrays(int a[], int[] s[]) { }
    int T;
    void shadow(int T, int t) { }
}
