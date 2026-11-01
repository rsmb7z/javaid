import java.util.*;
public class FpProbe {
    void a(int x) { }
    void b(final int x) { }
    void c(String s, final int x, Object... rest) { }
    void d(final String s, final int x, final Object... rest) { }
    void e(@Deprecated int x) { }
    void f(@Deprecated final int x) { }
    void g(final @Deprecated int x) { }
    void h(int[] arr, List<String> l, long n, boolean b, char c, byte y, short z, float f, double d) { }
    void i() { }
    void j(int x, int y) { }
    abstract static class Abs { abstract void k(int x); native void l(int x); }
    interface I { void m(int x); default void n(int x) { } static void o(int x) { } }
    FpProbe(int x) { }
    FpProbe(final int x, String s) { }
    FpProbe() { }
    record R(int a) { R { } R(String s) { this(1); } void p(int q) { } }
    enum E { A(1); E(int x) { } void q(int x) { } }
    void r() {
        try { } catch (Exception e) { }
        try { } catch (final Exception e) { }
        try { } catch (IllegalStateException | IllegalArgumentException e) { }
        for (String s : new ArrayList<String>()) { }
        for (final String s : new ArrayList<String>()) { }
        for (int i : new int[0]) { }
        for (var v : new int[0]) { }
        for (int i = 0; i < 1; i++) { }
        Runnable run = () -> { };
        java.util.function.IntConsumer ic = (int x) -> { };
        java.util.function.IntConsumer ic2 = (final int x) -> { };
        java.util.function.IntConsumer ic3 = x -> { };
        new Object() { void s(int x) { } };
    }
    void t(java.lang.String s, java.util.List<?> l, Map.Entry<String, String> e) { }
    <T> void u(T t, final T u) { }
    void v(Integer boxed, int @Deprecated [] arr, int arr2[]) { }
    void w(int x, final int y, int z) { }
    void x(FpProbe this, int a) { }
    void y(var v) { }
    public static void main(String[] args) { }
}
