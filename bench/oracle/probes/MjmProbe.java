package p;

public class MjmProbe {
    public void plain() { }
    protected void prot() { }
    void pkg() { }
    private void priv() { }
    /** doc */ public void documented() { }
    @Override public String toString() { return ""; }
    @Override
    public int hashCode() { return 1; }
    @Deprecated public void deprecated() { }
    @Test public void tested() { }
    public MjmProbe() { }
    protected MjmProbe(int a) { }
    MjmProbe(int a, int b) { }
    public int getFoo() { return 1; }
    public int isFoo() { return 1; }
    public boolean isBar() { return true; }
    public void setFoo(int foo) { this.foo = foo; }
    public void setBar(int bar) { }
    public void setBaz(int a, int b) { this.a = a; }
    public int getThrows() throws Exception { return 1; }
    public int getNoReturn() { int x = 1; return x; }
    public int getLate() { int x = 1; return x; }
    public int getEmpty() { }
    public int get() { return 1; }
    public int getfoo() { return 1; }
    public void setX(int x) { x = 1; foo(); }
    public void setY(int y) { }
    public abstract void abs();
    public native void nat();
    public void oneLine() { int x; }
    public void twoLines() {
        int x;
    }
    public void emptyBlock() {
    }
    public void multi() {
        int a;
        int b;
        int c;
    }

    public static class PubNested {
        public void pn() { }
        void pkgn() { }
        public PubNested() { }
    }
    static class PkgNested {
        public void pkn() { }
    }
    protected static class ProtNested {
        public void pn() { }
        protected void prn() { }
    }
    private static class PrivNested {
        public void pvn() { }
    }
    public interface I {
        void im();
        default void dm() { }
        static void sm() { }
        private void pm() { }
    }
    interface PkgI {
        void im();
    }
    public @interface Ann {
        int v();
        String s() default "";
    }
    @interface PkgAnn {
        int v();
    }
    public enum E {
        A;
        public void em() { }
        void pkg() { }
        E() { }
        public E(int x) { }
    }
    public record R(int a) {
        public R { }
        public void rm() { }
        void pkg() { }
        public int a() { return a; }
    }
    record PkgR(int a) {
        public PkgR { }
        public void rm() { }
    }

    public void anon() {
        Object o = new Object() {
            public void am() { }
            public String toString() { return ""; }
        };
        class Local {
            public void lm() { }
        }
        Runnable r = () -> { };
    }
    Object field = new Object() {
        public void fm() { }
    };
}

class PkgTop {
    public void m() { }
    public PkgTop() { }
}

public interface PubTop {
    void m();
}

interface PkgTopI {
    void m();
}

public enum PubEnum {
    A;
    public void m() { }
}

public @interface PubAnn {
    int v();
}
