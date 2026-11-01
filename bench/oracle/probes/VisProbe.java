package p;

import org.junit.Rule;
import com.google.common.annotations.VisibleForTesting;

public class VisProbe {
    int pkg;
    protected int prot;
    public int pub;
    private int priv;
    static int spkg;
    final int fpkg = 1;
    static final int SF = 1;
    public static final int PSF = 1;
    public final int pf = 1;
    protected static int ps;
    public static int pubstatic;
    public int serialVersionUID;
    public static final long serialVersionUID2 = 1L;
    int a, b[], c;
    public int x, y;
    @Rule public int ruled;
    @org.junit.Rule public int ruled2;
    @org.junit.ClassRule int ruled3;
    @VisibleForTesting int vft;
    @Deprecated int dep;
    @SuppressWarnings("x") public int sw;
    @Rule
    int ruled4;
    transient int tr;
    volatile int vol;
    static volatile int svol;
    final static int FS = 2;
    private static final int PRIV = 3;
    public String s = "a";
    public int[] arr;

    void m() {
        int local;
        Object o = new Object() {
            int anonField;
            public int anonPub;
        };
        class Local {
            int localField;
        }
    }

    interface I {
        int IF = 1;
        class Inner {
            int innerOfInterface;
        }
    }

    @interface Ann {
        int v();
        class AnnInner {
            int ai;
        }
    }

    enum E {
        A, B;
        int enumField;
        public int enumPub;
        private int enumPriv;
    }

    record R(int a, int b) {
        static int rs;
        public static final int RSF = 1;
        static final int rsf2 = 1;
    }

    static class Nested {
        int nestedField;
        class Deep {
            public int deep;
        }
    }

    interface J {
        enum Inside {
            X;
            int insideEnum;
        }
    }

    class Gen<T> {
        T t;
        public T pt;
    }
}
