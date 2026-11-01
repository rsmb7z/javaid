package probes;

public class ScProbe {
    // CHECKSTYLE:OFF
    long a1 = 1l;
    // CHECKSTYLE:ON
    long a2 = 1l;
    // CHECKSTYLE.OFF: UpperEll
    long b1 = 1l; void Bad_B() {}
    // CHECKSTYLE.ON: UpperEll
    long b2 = 1l;
    // CHECKSTYLE.OFF: UpperEll|MethodName
    long b3 = 1l; void Bad_C() {}
    // CHECKSTYLE.ON: MethodName
    long b4 = 1l; void Bad_D() {}
    // CHECKSTYLE.ON: UpperEll
    // OFF UpperEll
    long c1 = 1l; int x = 1;; // TODO trailing
    // ON UpperEll
    /* OFF Todo */
    // TODO again
    /* ON Todo */
    // TODO after on
    // IDOFF
    static final int bad_const = 1;
    // IDON
    static final int bad_const2 = 1;
    /* OFF MethodName
       more text
       ON MethodName */
    void Bad_E() {}
    long d1 = 1l; /* CHECKSTYLE:OFF */ long d2 = 1l;
    long d3 = 1l;
    long d4 = 1l; // CHECKSTYLE:ON
    long d5 = 1l;
	long e1 = 1l;  // CHECKSTYLE:OFF
	long e2 = 1l;
    // CHECKSTYLE:ON
    // CHECKSTYLE.SUPPRESS: UpperEll
    long f1 = 1l;
    long f2 = 1l;
    long f3 = 1l; // CHECKSTYLE.SUPPRESS: UpperEll
    long f4 = 1l;
    // NEAR UpperEll -2
    long g1 = 1l;
    long g2 = 1l;
    long g3 = 1l;
    // SUPPRESS CHECKSTYLE UpperEll
    long h1 = 1l;
    long h2 = 1l; // SUPPRESS CHECKSTYLE UpperEll
    // MSG UpperEll Upper
    long i1 = 1l;
    long i2 = 1l;
    long i3 = 1l;
    long i4 = 1l;
    /* MSG MethodName Name */
    void Bad_F() {}
    void Bad_G() {}
    void Bad_H() {}
    // NID
    static final int bad_const3 = 1;
    static final int bad_const4 = 1;
    /* NID */
    static final int bad_const5 = 1;
    static final int bad_const6 = 1;
    static final int bad_const7 = 1;
    static final int bad_const8 = 1;
    // NEAR UpperEll 0
    long j1 = 1l; // NEAR UpperEll 1
    long j2 = 1l;
    // CHECKSTYLE:OFF
    // CHECKSTYLE:OFF
    long k1 = 1l;
    // CHECKSTYLE:ON
    long k2 = 1l; String s = "// CHECKSTYLE:OFF";
    long k3 = 1l;
}
