package p;

import java.util.function.Supplier;

public class JvProbe2 {
    interface I {
        enum En { Z, /** d */ Y }
        class C { int x; }
        int fld = 1;
    }
    public enum Pub { A, B }
    private enum Priv { A, /** d */ B }
    protected enum Prot { A }
    /** doc */ enum Doc { A, B }
    Supplier<Object> s = () -> new Object() { int inLambdaInit; };
    Object o = new Object() {
        int f1;
        class Deep { int f2; }
        void m() { int local; class L { int f3; } }
    };
    static { int local; class L { int f4; } }
    { Object o = new Object() { int f5; }; }
    JvProbe2() { Object o = new Object() { int f6; }; }
    void m() { Supplier<Object> s = () -> new Object() { int f7; }; Object a = new Object() { int f8; }; }
    /** doc */
    // line comment between
    /* block comment between */
    int afterComments;
    int a1, b1 = 2, c1[];
    /** doc */ int a2, b2, c2;
    /** doc */
    private final
    int wrapped;
    /** multi
     *  line */
    @Deprecated
    @SuppressWarnings("x")
    protected static volatile int ann;
    /***/ int starsOnly;
    /***/
    int stars2;
    /** */ int blank;
    /**
     */
    int emptyDoc;
    private static final long serialVersionUID = 1L;
    static final long serialPersistentFields = 1L;
    int ignoreMe;
}

record Rec(int a) {
    int notAllowed;
    /** d */ static int ok;
    static int missing;
}

interface Top { int K = 1; /** d */ int L = 1; }
@interface Anno { int V = 1; String name() default ""; }
