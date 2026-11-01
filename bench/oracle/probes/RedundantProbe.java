package p;

import java.io.IOException;
import java.io.Reader;

public class RedundantProbe {
    public interface I {
        public int A = 1;
        static int B = 2;
        final int C = 3;
        public static final int D = 4;
        public abstract void m();
        abstract void n();
        public void o();
        static void s() { }
        public static void ps() { }
        default void d() { }
        public default void pd() { }
        void withFinal(final int x);
        public static class SC { }
        static class SC2 { }
        public abstract static class AC { }
        final class FC { }
        public static interface NI { }
        static interface NI2 { }
        abstract interface NI3 { }
        public static enum NE { X }
        static enum NE2 { X }
        public static record NR(int a) { }
        static record NR2(int a) { }
        public @interface NA { }
        static @interface NA2 { }
        private static void ps2() { }
    }

    @interface Ann {
        public int v();
        public abstract int w();
        int X = 1;
        public static final int Y = 2;
        static class AC { }
    }

    abstract static class Abs {
        abstract void m(final int a);
        void c(final int a) { }
        native void nat(final int a);
        public abstract void pm();
    }

    static final class FinalClass {
        final void m() { }
        static final void sm() { }
        private final void pm() { }
        public FinalClass() { }
        final int f = 1;
        @SafeVarargs final void sv(java.util.List<String>... l) { }
        @java.lang.SafeVarargs final void sv2(java.util.List<String>... l) { }
        class Inner {
            final void im() { }
        }
        interface InnerI {
            default void d() { }
        }
        record InnerR(int a) {
            final void rm() { }
        }
    }

    class NonFinal {
        final void m() { }
        private final void pm() { }
        static final void sm() { }
        private static final void psm() { }
        public NonFinal() { }
        protected NonFinal(int a) { }
        private NonFinal(int a, int b) { }
        NonFinal(int a, int b, int c) { }
        @SafeVarargs private final void psv(java.util.List<String>... l) { }
        Object o = new Object() {
            final void am() { }
            public final String toString() { return ""; }
        };
    }

    public static class PublicNested {
        public PublicNested() { }
        public static class Deeper {
            public Deeper() { }
        }
        static class PkgDeeper {
            public PkgDeeper() { }
        }
        protected static class ProtDeeper {
            public ProtDeeper() { }
        }
        public class Pi {
            public Pi() { }
        }
    }

    static class PkgNested {
        public PkgNested() { }
        public static class Deeper {
            public Deeper() { }
        }
        protected static class Prot {
            public Prot() { }
        }
    }

    protected static class Prot {
        public Prot() { }
    }

    private static class Priv {
        public Priv() { }
    }

    enum E {
        X, Y { final void m() { } void n() { } };
        public E() { }
        private E(int a) { }
        E(int a, int b, int c) { }
        @Deprecated private E(String s) { }
        final void f() { }
        static final void sf() { }
        private final void pf() { }
        abstract static class AbsIn { }
    }

    enum E2 {
        X;
        E2() { }
        protected E2(int a) { }
    }

    public static enum PubEnum { A }
    static enum StatEnum { A }
    static record StatRec(int a) { }
    final record FinRec(int a) { }
    static final record SFRec(int a) { public SFRec { } public SFRec(String s) { this(1); } }
    record PlainRec(int a) { final void m() { } public PlainRec(int a, int b) { this(a); } }
    public record PubRec(int a) { public PubRec(int a, int b) { this(a); } }
    static strictfp class SfpClass { strictfp void m() { } }
    strictfp interface SfpI { }
    static strictfp enum SfpE { A }

    void resources() throws IOException {
        try (final Reader r = null; Reader q = null) { }
        try (final Reader r = null) { } finally { }
        final int local = 1;
        final Object o = null;
    }

    void locals(final int p, Object o) {
        final int a = 1;
        for (final int i : new int[0]) { }
        try { } catch (final RuntimeException e) { }
        java.util.function.IntUnaryOperator f = (final int x) -> x;
        if (o instanceof final String s) { }
    }
}

class PkgTop {
    public PkgTop() { }
    public static class N {
        public N() { }
    }
    void m() {
        class Local {
            public Local() { }
        }
        Object o = new Object() { final void m() { } };
    }
}

final class FinalTop {
    final void m() { }
    static final class N { final void m() { } }
    class N2 { final void m() { } }
}

public class PubTop2 {
    public PubTop2() { }
}

abstract class AbsTop {
    public abstract void m();
    public AbsTop() { }
}

interface TopI {
    public void m();
    abstract void a();
    public interface Nested { }
    class Cls { public Cls() { } }
    public static class PCls { public PCls() { } }
}

enum TopE {
    A;
    public static final int X = 1;
    final void m() { }
}
