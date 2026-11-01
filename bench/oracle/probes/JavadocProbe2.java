/** Package doc {@link Jp01} */
package probe2;

import a.Jp01;
import a.Jp02;
import a.Jp03;
import a.Jp04;
import a.Jp05;
import a.Jp06;
import a.Jp07;
import a.Jp08;
import a.Jp09;
import a.Jp10;
import a.Jp11;
import a.Jp12;
import a.Jp13;
import a.Jp14;
import a.Jp15;
import a.Jp16;
import a.Jp17;
import a.Jp18;
import a.Jp19;
import a.Jp20;
import a.Jp21;
import a.Jp22;
import a.Jp23;
import a.Jp24;
import a.Jp25;
import a.Jp26;
import a.Jp27;
import a.Jp28;
import a.Jp29;
import a.Jp30;

public class JavadocProbe2 {
    void body() {
        /** local class doc {@link Jp02} */
        class Local {}
        /** local record doc {@link Jp03} */
        record R(int a) {}
        new Object() {
            /** anon field {@link Jp04} */
            int f;

            /** anon method {@link Jp05} */
            void m() {}
        };
        Runnable r = () -> {
            /** in lambda {@link Jp06} */
            int z = 0;
        };
    }

    int a; /** trailing on the line before the next member {@link Jp07} */
    int b;

    /** stacked first {@link Jp08} */
    /** stacked second {@link Jp09} */
    void stacked() {}

    /** @return the {@link Jp10} */
    int ret() { return 0; }

    /** double  space {@link   Jp11   label} and tab {@link	Jp12} */
    void spaces() {}

    /** @see "Jp13" */
    void quoted() {}

    /** @throws java.io.Jp14 qualified */
    void thr() {}

    /** {@link #foo(List<Jp15>)} generic arg */
    void gen() {}

    /** {@link #foo(Jp16...)} varargs and {@link #bar(Jp17[])} array */
    void va() {}

    /** static block doc {@link Jp18} */
    static { }

    /** instance init doc {@link Jp19} */
    { }

    /** record doc {@link Jp20} */
    record Rec(/** component doc {@link Jp21} */ int c) {}

    /** {@link Jp22 } trailing space inside braces */
    void ws() {}

    /** @link Jp23 not a real tag */
    void fake() {}

    /** {@link Jp24}{@link Jp25} adjacent */
    void adj() {}

    /** pre: <pre>{@link Jp26}</pre> and <code>{@link Jp27}</code> */
    void html() {}

    /** @see Jp28 Jp29 more words */
    void seeWords() {}

    /** @param <T> the {@link Jp30} param type */
    <T> void typeParam() {}
}
