/** before package: ok */
package p;

/** before import: invalid */
import java.util.List;
/** between imports: invalid */
import java.util.Map;

/** class doc: ok */
public class IjpProbe {
    /** field ok */ int a;
    /** two */ /** three */ int b;
    @Deprecated /** between annotation and modifier */ public int c;
    public /** between modifier and type */ int d;
    int /** between type and name */ e;
    /** before annotation */ @Deprecated public int f;
    /** before type */ int g;
    int h; /** trailing doc: attaches to next */
    int i;
    /** method ok */ void m() { }
    /** before type params */ <T> void gen() { }
    public <T> /** after type params */ void gen2() { }
    void body() {
        /** inside method: invalid */
        int local = 1;
        /** local class: ok? */ class L { }
        /** local record */ record LR(int x) { }
        /** before statement: invalid */
        local++;
        Runnable r = /** in expr */ () -> { };
        foo(/** in args */ 1);
    }
    /** before ctor ok */ IjpProbe() { }
    /** before static init: invalid */
    static { }
    /** before instance init: invalid */
    { }
    /** enum doc */ enum E {
        /** constant ok */ A,
        /** constant with annotation */ @Deprecated B,
        @Deprecated /** between */ C,
        D /** after constant */,
        E1;
        /** end of enum: invalid */
    }
    /** interface */ interface I {
        /** method */ void m();
        /** field */ int K = 1;
    }
    /** annotation */ @interface Ann {
        /** element */ int v();
    }
    /** record doc */ record R(/** component: invalid */ int a, int b) {
        /** compact ctor ok */ R { }
    }
    void params(/** param doc: invalid */ int p) { }
    /** end of class: invalid */
}

/** second top-level */
class Second { }

/** end of file: invalid */
