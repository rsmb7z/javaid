public class FpProbe2 {
    void a(int... v) { }
    void b(String... v) { }
    void c(int x[]) { }
    void d(int[] x[]) { }
    void e(int[]... x) { }
    void f(final int... v) { }
    void g(@SuppressWarnings("x") final int x, @SuppressWarnings("y") int y) { }
    void h() {
        try { } catch (final RuntimeException | Error e) { }
        for (int[] row : new int[0][0]) { }
        for (int i[] : new int[0][0][0]) { }
        for (@SuppressWarnings("x") int i : new int[0]) { }
        for (@SuppressWarnings("x") final int i : new int[0]) { }
        for (final var i : new int[0]) { }
        try (java.io.StringReader r = new java.io.StringReader("")) { } catch (Exception e) { }
    }
    FpProbe2(int... v) { }
    FpProbe2(final long l, int[] a) { }
}
