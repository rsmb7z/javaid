package p;

import java.io.IOException;

public class JmProbe<T> {
    /** No tags at all. */
    public int noTags(int a, String b) { return 1; }

    /**
     * Complete.
     * @param a the a
     * @param b the b
     * @return one
     */
    public int complete(int a, String b) { return 1; }

    /**
     * Missing param b.
     * @param a the a
     * @return one
     */
    public int missingParam(int a, String b) { return 1; }

    /**
     * Unused param.
     * @param a the a
     * @param z not a parameter
     * @return one
     */
    public int unusedParam(int a) { return 1; }

    /**
     * Duplicate param.
     * @param a the a
     * @param a again
     * @return one
     */
    public int dupParam(int a) { return 1; }

    /**
     * Missing return.
     * @param a the a
     */
    public int missingReturn(int a) { return 1; }

    /**
     * Return on void.
     * @param a the a
     * @return nothing
     */
    public void returnOnVoid(int a) { }

    /**
     * Duplicate return.
     * @param a the a
     * @return one
     * @return two
     */
    public int dupReturn(int a) { return 1; }

    /**
     * Bare return.
     * @param a the a
     * @return
     */
    public int bareReturn(int a) { return 1; }

    /**
     * Generic.
     * @param <U> the u
     * @param a the a
     * @return t
     */
    public <U> T generic(U a) { return null; }

    /**
     * Generic missing.
     * @param a the a
     * @return t
     */
    public <U, V> T genericMissing(U a) { return null; }

    /**
     * Generic unused.
     * @param <Q> the q
     * @param a the a
     * @return t
     */
    public <U> T genericUnused(U a) { return null; }

    /**
     * Throws documented.
     * @param a the a
     * @return one
     * @throws IOException on io
     */
    public int throwsDoc(int a) throws IOException { return 1; }

    /**
     * Throws missing.
     * @param a the a
     * @return one
     */
    public int throwsMissing(int a) throws IOException, IllegalStateException { return 1; }

    /**
     * Throws unmatched tag.
     * @param a the a
     * @return one
     * @throws IOException not thrown
     * @exception Foo.Bar also not
     */
    public int throwsExtra(int a) { return 1; }

    /**
     * Throw statement.
     * @param a the a
     * @return one
     */
    public int throwStatement(int a) {
        if (a < 0) { throw new IllegalArgumentException("x"); }
        try { return 1; } catch (RuntimeException e) { throw new IllegalStateException(e); }
    }

    /**
     * Inherit doc.
     * {@inheritDoc}
     */
    public int inherit(int a) { return 1; }

    /**
     * {@inheritDoc}
     * @param a the a
     */
    public int inheritPlusParam(int a, int b) { return 1; }

    /** {@inheritDoc} */
    private int inheritPrivate(int a) { return 1; }

    /** {@inheritDoc} */
    public static int inheritStatic(int a) { return 1; }

    /** {@inheritDoc} */
    public JmProbe() { }

    /**
     * Override annotated.
     */
    @Override
    public int overridden(int a) { return 1; }

    /**
     * Override annotated with partial.
     * @param z unmatched
     */
    @Override
    public int overriddenPartial(int a) { return 1; }

    /**
     * Ctor.
     * @param a the a
     * @return nothing
     */
    public JmProbe(int a) { }

    /**
     * Ctor missing.
     */
    public JmProbe(int a, int b) { }

    /**
     * Varargs.
     * @param a the a
     */
    public void varargs(int... a) { }

    /**
     * Inline return.
     * {@return the value}
     */
    public int inlineReturn() { return 1; }

    /**
     * Multi line param
     * @param a
     *        the a
     * @param b the b
     */
    public void multiline(int a, int b) { }

    /**
     * Bare param.
     * @param
     */
    public void bareParam(int a) { }

    /**
     * Void ok.
     */
    public void voidOk() { }

    /**
     * Other tags.
     * @see Foo
     * @since 1
     * @deprecated no
     * @author me
     * @custom thing
     */
    public int otherTags() { return 1; }

    /** no space*/
    public int tight(int a) { return 1; }

    /* not javadoc */
    public int notJavadoc(int a) { return 1; }

    public int noJavadoc(int a) { return 1; }

    /**
     * Private.
     */
    private int priv(int a) { return 1; }

    /**
     * Pkg.
     */
    int pkg(int a) { return 1; }

    /**
     * Prot.
     */
    protected int prot(int a) { return 1; }

    /**
     * Abstract.
     */
    public abstract int abs(int a);

    /**
     * Native.
     */
    public native int nat(int a);

    interface I {
        /**
         * Iface.
         */
        int im(int a);
        /** Default. */
        default int dm(int a) { return 1; }
    }

    @interface Ann {
        /** Elem. */
        int v();
        /**
         * Elem with params.
         * @param x bad
         * @return ok
         */
        String w();
        /**
         * Elem return missing.
         */
        String z() default "";
    }

    record R(int a, int b) {
        /**
         * Compact.
         * @param a the a
         */
        public R { }
        /**
         * Canonical.
         */
        public R(int a, int b, int c) { this(a, b); }
    }

    private static class Hidden {
        /** Doc. */
        public int hid(int a) { return 1; }
    }
    public static class Shown {
        /** Doc. */
        public int shown(int a) { return 1; }
        /** Doc. */
        void shownPkg(int a) { }
    }
    Object o = new Object() {
        /** Doc. */
        public int anon(int a) { return 1; }
    };
    enum E {
        A;
        /** Doc. */
        public int ev(int a) { return 1; }
    }
}
