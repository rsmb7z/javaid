public class LenProbe {
    void a() { }
    void b() {
    }
    void c() {

    }
    void d() {
        int x = 1;
    }
    void e() {
        // comment
        int x = 1;
    }
    void f() {
        int x = 1;

        /* block
           comment */
        int y = 2; // trailing
    }
    void g() { int x = 1;
        int y = 2; }
    abstract static class Abs { abstract void h(); abstract void i(int a, int b, int c); }
    LenProbe() { }
    LenProbe(int a) {
        int x = a;
        int y = x;
    }
    LenProbe(int a, int b, int c) {
    }
    record R(int a, int b, int c) {
        R {
            int x = a;
            int y = b;
        }
        void m() {
            Runnable r = () -> {
                int z = 1;
                int w = 2;
            };
        }
    }
    void j(
        int a,
        int b) {
        int x = a;
    }
    @Deprecated
    void k() {
        int x = 1;
        int y = 2;
    }
    void l() {

        int x = 1;

        int y = 2;

    }
    interface I { default void m() { int x = 1; int y = 2; } int n(); }
    static { int s = 1; }
    { int t = 1; }
    @Foo void o(int a, int b, int c) { }
    @Bar(1) void p(int a, int b, int c) { }
    @Baz void q(int a, int b, int c) { }
    @Override public String toString() { return null; }
    @Override public boolean equals(Object a) { return false; }
    void r(int a, int b) { }
    void s(int a, int b, int c) { }
    void t(int a, int... b) { }
    void u(int a, int b, int... c) { }
    void v(final int a, final int b, final int c) { }
    void w(@Foo int a, @Foo int b, @Foo int c) { }
    <T> void x(T a, T b, T c) { }
    enum E { A(1, 2, 3); E(int a, int b, int c) { } }
    void y() { new Object() { void z(int a, int b, int c) { } }; }
    int z0(int a, int b, int c) { return 0; }
}
