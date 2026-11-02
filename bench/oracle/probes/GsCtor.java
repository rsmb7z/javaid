package bench.oracle.probes;

import java.util.List;

public class GsCtor {
    int a;

    GsCtor() {
    }

    GsCtor(int a, int b) {
    }

    GsCtor(int a) {
    }

    void m() {
    }

    @Deprecated
    GsCtor(String s) {
    }

    private <T> GsCtor(T t, int... xs) {
    }

    int b;

    protected
    GsCtor(long l) {
    }

    static class Grouped {
        Grouped(int a, int b, int c) {
        }

        /* comment between */
        Grouped(int a) {
        }

        // another comment
        Grouped() {
        }

        void x() {
        }
    }

    static class Semi {
        Semi(int a) {
        }
        ;
        Semi(int a, int b) {
        }

        Semi() {
        }
    }

    static class Init {
        Init() {
        }

        static {
        }

        Init(int a) {
        }

        {
        }

        Init(int a, int b) {
        }
    }

    static class NoCtor {
        void a() {
        }
    }

    enum E {
        A(1), B(2, 3);

        E(int a, int b) {
        }

        E(int a) {
        }

        void f() {
        }

        E() {
        }
    }

    enum E2 {
        A, B;
        E2(int a) {
        }
        E2() {
        }
    }

    record R(int x, int y) {
        R {
        }

        R(int x) {
            this(x, 0);
        }

        void m() {
        }

        R() {
            this(0, 0);
        }
    }

    record R2(int x) {
        R2(String s) {
            this(1);
        }

        R2 {
        }

        R2(String s, String t) {
            this(1);
        }
    }

    interface I {
        void m();
    }

    void local() {
        class Loc {
            Loc(int a, int b) {
            }

            Loc(int a) {
            }

            void f() {
            }

            Loc() {
            }
        }
        Object o = new Object() {
            void f() {
            }
        };
    }

    static class Inner {
        Inner() {
        }

        class Deep {
            Deep(int a) {
            }

            int f;

            Deep() {
            }
        }

        Inner(int a) {
        }
    }

    public static void main(String[] args) {
    }
}
