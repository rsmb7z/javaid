package p;

public class DfeProbe {
    public void plain() { int x = 1; }
    protected void prot() { int x = 1; }
    void pkg() { int x = 1; }
    private void priv() { int x = 1; }
    public final void fin() { int x = 1; }
    public static void stat() { int x = 1; }
    public abstract void abs();
    public native void nat();
    public void empty() { }
    public void emptyComment() { /* nothing */ }
    public void emptyLine() {
        // nothing
    }
    public void emptySemi() { ; }
    /** documented */
    public void documented() { int x = 1; }
    /* plain block comment */
    public void blockComment() { int x = 1; }
    // line comment
    public void lineComment() { int x = 1; }
    /**/
    public void tiny() { int x = 1; }
    /** */
    public void blank() { int x = 1; }
    /** first */ /* second */
    public void two() { int x = 1; }
    /* first */ /** second */
    public void twoRev() { int x = 1; }
    public /** inside */ void inside() { int x = 1; }
    public void /** afterType */ afterType() { int x = 1; }
    public <T> /** afterTypeParams */ void generic() { int x = 1; }
    /** beforeTypeParams */ public <T> void generic2() { int x = 1; }
    @Deprecated /** afterAnnotation */ public void annotated() { int x = 1; }
    /** beforeAnnotation */ @Deprecated public void annotated2() { int x = 1; }
    @Test public void test() { int x = 1; }
    @Before public void before() { int x = 1; }
    @After public void after() { int x = 1; }
    @BeforeClass public void bc() { int x = 1; }
    @AfterClass public void ac() { int x = 1; }
    @org.junit.Test public void qualified() { int x = 1; }
    @Override public String toString() { return "x"; }
    @Override public int hashCode() { return 1; }
    @Override public boolean equals(Object o) { }
    @Override public void emptyOverride() { }
    @Other public void other() { int x = 1; }
    @Test(expected = Exception.class) public void withArgs() { int x = 1; }
    public synchronized void sync() { int x = 1; }
    public int returns() { return 1; }
    public DfeProbe self() { return this; }

    public DfeProbe() { }

    public static class Nested {
        public void nm() { int x = 1; }
    }
    static class PkgNested {
        public void pm() { int x = 1; }
    }
    private static class PrivNested {
        public void pm() { int x = 1; }
    }
    protected static class ProtNested {
        public void pm() { int x = 1; }
        public static class Deep { public void dm() { int x = 1; } }
    }
    public final class FinalInner {
        public void fm() { int x = 1; }
    }
    public enum E {
        A;
        public void em() { int x = 1; }
    }
    public record R(int a) {
        public void rm() { int x = 1; }
    }
    public interface I {
        void im();
        default void dm() { int x = 1; }
    }
    public @interface Ann { int v(); }

    public void anon() {
        Object o = new Object() {
            public void am() { int x = 1; }
        };
        class Local {
            public void lm() { int x = 1; }
        }
    }
}

public final class DfeFinal {
    public void m() { int x = 1; }
}

class DfePackage {
    public void m() { int x = 1; }
}

public class DfePrivateCtor {
    private DfePrivateCtor() { }
    public void m() { int x = 1; }
}

public class DfeMixedCtor {
    private DfeMixedCtor() { }
    DfeMixedCtor(int a) { }
    public void m() { int x = 1; }
}

public class DfeProtCtor {
    protected DfeProtCtor() { }
    public void m() { int x = 1; }
}

public class DfePrivCtorsInner {
    public static class Inner {
        private Inner() { }
        public void m() { int x = 1; }
    }
    public void outer() { int x = 1; }
}

public enum DfeEnum {
    A;
    public void m() { int x = 1; }
}

public sealed class DfeSealed permits DfeSub {
    public void m() { int x = 1; }
}

public non-sealed class DfeSub extends DfeSealed {
    public void m2() { int x = 1; }
}

public record DfeRecord(int a) {
    public void m() { int x = 1; }
}

public interface DfeIface {
    default void m() { int x = 1; }
}

public class DfeGeneric<T> {
    public <U> void m(U u) { int x = 1; }
    public T get() { return null; }
}

public class DfeImplicit {
    /** doc */
    <C> DfeImplicit doNoMods(C c) { return null; }
    /** doc */
    public <C> DfeImplicit doMods(C c) { return null; }
    /** doc */
    @Deprecated <C> DfeImplicit doAnn(C c) { return null; }
    /** doc */
    <C> void withVoid(C c) { int x = 1; }
}

public interface DfeOuter {
    class Implicit {
        public void m() { int x = 1; }
    }
    interface Nested {
        class Deep { public void d() { int x = 1; } }
    }
    enum En { A; public void e() { int x = 1; } }
    class Priv { public void p() { int x = 1; } }
}

interface DfePkgIface {
    class Implicit {
        public void m() { int x = 1; }
    }
}
