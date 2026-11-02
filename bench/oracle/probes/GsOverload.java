package bench.oracle.probes;

public class GsOverload {
    void a() {
    }

    void a(int x) {
    }

    void b() {
    }

    void a(String s, int y) {
    }

    int c;

    void a(long l, long m, long n) {
    }

    void b(int x, int y, int z) {
    }

    void b(int x) {
    }

    @Override
    public String toString() {
        return "";
    }

    void c() {
    }
    ;
    void c(int a) {
    }

    void d(int a, int b) {
    }

    void d(int a) {
    }

    void d(int... a) {
    }

    void e(int a) {
    }

    void f() {
    }

    void e() {
    }

    void e(int a, int b) {
    }

    void e(int a, int b, int c) {
    }

    void e(String s, int b, int c) {
    }

    static class Inner {
        void m(int x) {
        }

        void n() {
        }

        void m() {
        }

        void m(String s, String t) {
        }
    }

    enum E {
        A, B;

        void p() {
        }

        void q() {
        }

        void p(int x) {
        }

        void p(int x, int y) {
        }
    }

    enum E2 {
        A;
        void p() {
        }
        void p(int x) {
        }
    }

    interface I {
        void k();

        void l();

        void k(int a);

        void k(int a, int b);
    }

    record R(int x) {
        void h() {
        }

        void i() {
        }

        void h(int a) {
        }
    }

    @interface Ann {
        int v();

        int w();

        int v();
    }

    Object o = new Object() {
        void z() {
        }

        void y() {
        }

        void z(int a) {
        }
    };

    void local() {
        class L {
            void q() {
            }

            void r() {
            }

            void q(int a) {
            }
        }
    }

    enum E3 {
        A {
            void v() {
            }

            void w() {
            }

            void v(int a) {
            }
        };
    }

    public static void main(String[] args) {
    }

    void main(String a, String b) {
    }

    void a(double d) {
    }
}
