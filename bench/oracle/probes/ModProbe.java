package ok.pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Target;

public class ModProbe {
    @Target(ElementType.TYPE_USE) @interface TA {}
    public static final int A = 1;
    static public final int B = 2;
    final static int C = 3;
    private static final long SERIAL = 1L;
    public final static int D = 4;
    @Deprecated public int e;
    public @Deprecated int f;
    final @Deprecated int g;
    public @TA String h;
    public synchronized void m1() {}
    synchronized public void m2() {}
    public abstract class Inner1 {}
    abstract public class Inner2 {}
    static abstract class Inner3 {}
    abstract static class Inner4 {}
    public static abstract class Inner5 {}
    final public class Inner6 {}
    public volatile int v;
    volatile public int v2;
    transient public int t;
    public transient volatile int tv;
    volatile transient int vt;
    native public void n();
    public native void n2();
    strictfp public void s() {}
    public strictfp void s2() {}
    public sealed interface SI permits A1, A2 {}
    sealed public interface SI2 permits A1 {}
    public non-sealed class NS extends SI3 {}
    non-sealed public class NS2 extends SI3 {}
    public static sealed class SC permits X {}
    sealed static class SC2 permits X {}
    static final public int sfp = 1;
    interface IF {
        default public void d() {}
        public default void d2() {}
        static public void s() {}
        public static void s2() {}
        private static void ps() {}
        abstract public void ap();
        public abstract void pa();
    }
    enum E { A; public static final int X = 1; static public int Y = 2; }
    record R(int a) { public static final int X = 1; static public int Y = 2; }
    void params(final @Deprecated int a, @Deprecated final int b) {}
    void locals() { final @Deprecated int a = 1; @Deprecated final int b = 2; }
    public static final @Deprecated int mix = 1;
    @Deprecated public @Deprecated int two = 1;
    public @Deprecated void annotatedVoid() {}
    public @TA String annotatedRet() { return null; }
    public @Deprecated class AnnClass {}
    protected static abstract final class Weird {}
    private final static transient volatile int many = 1;
    ModProbe() {}
    public @Deprecated ModProbe(int x) {}
    @Deprecated public ModProbe(long x) {}
    static @TA public int taLate;
    public static @TA int taMid;
    final @TA @Deprecated public int multiAnn;
}
