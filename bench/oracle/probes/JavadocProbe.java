/** Package-level doc: {@link Imp29} */
package probe;

import a.Imp01;
import a.Imp02;
import a.Imp03;
import a.Imp04;
import a.Imp05;
import a.Imp06;
import a.Imp07;
import a.Imp08;
import a.Imp09;
import a.Imp10;
import a.Imp11;
import a.Imp12;
import a.Imp13;
import a.Imp14;
import a.Imp15;
import a.Imp16;
import a.Imp17;
import a.Imp18;
import a.Imp19;
import a.Imp20;
import a.Imp21;
import a.Imp22;
import a.Imp23;
import a.Imp24;
import a.Imp25;
import a.Imp26;
import a.Imp27;
import a.Imp28;
import a.Imp29;
import a.Imp30;
import a.Imp31;
import a.Imp32;
import a.Imp33;

/** class doc: {@link Imp01}. */
public class JavadocProbe {

    /** field doc {@link Imp02} */
    int f;

    /** method doc {@link Imp03} */
    void m() {}

    /** ctor doc {@link Imp04} */
    JavadocProbe() {}

    /** nested class {@link Imp05} */
    static class N {}

    /**
     * see tag
     *
     * @see Imp06
     */
    void see() {}

    /**
     * @throws Imp07 when bad
     */
    void thr() {}

    /** {@linkplain Imp08 text} */
    void lp() {}

    /** {@value Imp09#X} */
    void val() {}

    /** {@code Imp10} */
    void code() {}

    /** plain text Imp11 */
    void plain() {}

    /**
     * @param x the {@link Imp12} to use
     */
    void param(int x) {}

    /** {@link #foo(Imp13)} */
    void args() {}

    /** {@link Other#bar(java.lang.String, Imp14)} */
    void args2() {}

    void body() {
        /** local doc {@link Imp15} */
        int local = 0;
        /** statement doc {@link Imp16} */
        foo();
    }

    /** blank line follows {@link Imp17} */

    void blank() {}

    /** line comment follows {@link Imp18} */
    // some line comment
    void lineComment() {}

    /** block comment follows {@link Imp19} */
    /* a block comment */
    void blockComment() {}

    /**
     * multi-line inline {@link
     * Imp20} here
     */
    void multi() {}

    /** @see <a href="x">Imp21</a> */
    void anchor() {}

    /** @see Imp22#get(Object) */
    void seeMethod() {}

    /** {@link java.util.Imp23} */
    void qualified() {}

    /** annotated {@link Imp24} */
    @Deprecated
    void annotated() {}

    /** nested ref {@link Imp25.Inner} */
    void nestedRef() {}

    /** @exception Imp26 */
    void exc() {}

    enum E {
        /** const doc {@link Imp27} */
        A;
    }

    /** {@link Imp28} */
    @interface Ann {
        /** elem doc {@link Imp30} */
        int x();
    }

    /** two refs {@link Imp31} and {@link Imp32} */
    void two() {}

    /** @see Imp33 */
    int trailing; /** after */
}
