package p;

public class JvProbe {
    /** doc */ @Deprecated public int a;
    @Deprecated /** doc */ public int b;
    @Deprecated public /** doc */ int c;
    public /** doc */ int d;
    /** doc */ public int e;
    public int /** doc */ f;
    /** one */ /** two */ public int g;
    /** one */ /* x */ public int h;
    /** one */ // x
    public int i;
    /**/ public int j;
    public /** doc */ static int k;
    /** doc */ int l;
    int /** doc */ l2;
    /** doc */ int m, n;
    int noDoc;
    private int priv;
    /* plain */ public int plain;
    // line
    public int line;
    public int serialVersionUID;
    /** doc */
    @Deprecated
    public int multi;
    @Deprecated
    /** doc */
    public int afterAnn;
    /**
     * doc
     */
    private static final java.util.List<String> generic = null;
    /** doc */ int[] arr[];
    /** doc */
    Object o = new Object() { int inAnon; /** d */ int docAnon; };

    void m() {
        int local;
        /** doc */ int localDoc;
        class L { int inLocal; /** doc */ int localDoc2; }
    }

    enum E {
        /** doc */ A,
        B,
        @Deprecated C,
        /** doc */ @Deprecated D,
        @Deprecated /** doc */ E1;
        int ef;
        /** doc */ int efd;
    }

    interface I {
        int K = 1;
        /** doc */ int L = 2;
        class In { int innerInIface; }
    }

    @interface Ann {
        int v();
        int W = 1;
        /** doc */ int X = 2;
    }

    record R(int a, int b) {
        static int rs;
        /** doc */ static int rsd;
    }

    private static class PrivNested { int pn; /** d */ int pnd; }
    protected static class ProtNested { int pn; public int pub; }
    public static class PubNested { int pn; public int pub; protected int prot; private int priv; }
}

class PkgTop { int pt; public int ptPub; }
