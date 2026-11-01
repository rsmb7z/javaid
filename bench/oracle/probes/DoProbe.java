package p;

public class DoProbe {
    private int a;
    public static int s1;
    int b;
    static int s2;
    protected static int s3;
    private static int s4;
    public static int s5;
    public int c;
    private int d;
    protected int e;
    int f;
    public int g;

    DoProbe() { }
    public int h;
    static int s6;
    void m() { }
    DoProbe(int x) { }
    protected DoProbe(int x, int y) { }
    private int late = 1;
    static final int LATE_S = 2;
    @Deprecated
    int annotated;
    @Deprecated static int annotatedStatic;
    private int i1, i2;
    static int t1, t2;

    class Inner {
        int x;
        static int y;
        Inner() { }
        int z;
        void m() { }
        Inner(int q) { }
    }

    private int fwd1 = fwd2 + 1;
    public int fwd2 = 1;
    private int self = self + 1;
    public int uses = LATE_S;
    private static int fs1 = DoProbe.LATE_S;
    public static int fs2;

    enum E {
        A, B;
        private int p;
        public int q;
        static int r;
        E() { }
        void m() { }
        E(int x) { }
        int after;
    }

    interface I {
        int K = 1;
        void m();
        int L = 2;
        static int M = 3;
        public static int N = 4;
        private int O = 5;
    }

    @interface Ann {
        int v();
        int W = 1;
    }

    record R(int a) {
        private int rf;
        static int rs;
        R { }
        void m() { }
        R(int a, int b) { this(a); }
        public int rp;
    }

    Object o = new Object() {
        void m() { }
        int anon;
        Object() { }
    };

    void method() {
        int local;
        class L {
            void lm() { }
            int lf;
            L() { }
        }
    }
    public int last;
    private static int lastS;
}
