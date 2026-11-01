package ok.pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Target;

public class ModProbe2 {
    @Target(ElementType.TYPE_USE) @interface TA {}
    void g(java.util.List<String> xs, Object o) {
        try (final @TA java.io.StringReader r = null) {}
        try (@TA final java.io.StringReader r2 = null) {}
        for (final @TA String s : xs) {}
        for (@TA final String s2 : xs) {}
        try { } catch (final @TA RuntimeException e) {}
        if (o instanceof final @TA String s3) {}
        java.util.function.Function<String, String> f = (final @TA String a) -> a;
        java.util.function.Function<String, String> f2 = (@TA final String a) -> a;
    }
    record R(final @TA int a) {}
    record R2(@TA int a) {}
    record R3(int a) { public @TA R3 {} }
    record R4(int a) { @TA public R4 {} }
    enum E { @Deprecated A, B; public @TA E() {} }
    @interface An { public @TA int v(); public @Deprecated int w(); public @Deprecated void x(); }
    interface I { public @TA int c = 1; static @Deprecated void sm() {} default @TA String dm() { return null; } }
    public @Deprecated enum En { A }
    public @Deprecated record Rec() {}
    public @Deprecated interface Intf {}
    public @Deprecated @interface Ann2 {}
    static @TA Object[] arr;
    public @TA <T> void generic() {}
    public <T> @TA T generic2() { return null; }
    static @Deprecated <T> void generic3() {}
}
