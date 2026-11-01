/**
 * Package doc.
 * @since 1
 * @deprecated no
 */
package p;

/** @author a @version b */
class SingleTags { }

/** {@inheritDoc} */
class SingleInline { }

/** Just text. */
class SingleText { }

/** @deprecated */
class SingleEmptyDep { }

/**
 * Multi.
 * @deprecated gone
 * @param <T> t
 * @return x
 * @throws E e
 */
class Misordered<T> {
    /**
     * Method.
     * @return x
     * @param a the a
     * @throws IOException io
     * @param b the b
     * @deprecated old
     */
    int m(int a, int b) { return 1; }

    /**
     * Empty descriptions.
     * @param a
     * @param b the b
     * @return
     * @throws IOException
     * @exception Foo
     * @deprecated
     * @since
     * @see Other
     * @author
     */
    int e(int a, int b) { return 1; }

    /**
     * Description on next line.
     * @param a
     *        the a
     * @param
     *        b
     * @return
     *        x
     */
    int n(int a, int b) { return 1; }

    /** @return x */
    int single() { return 1; }

    /** {@code x} and {@link Foo} */
    int singleInline() { return 1; }

    /**
     * No asterisks here
       this line has none
     * but this one does

     * after a blank line
     <!-- html
     comment lines
     -->
     * done
       */
    int noAsterisks() { return 1; }

    /**
     * Last line with text */
    int lastLineText() { return 1; }

    /**
     * Fine.
     */
    int fine() { return 1; }

    /** Field doc.
     * @deprecated old
     * @since 1
     */
    int field;

    /**
     * @return x
     * @param a the a
     */
    enum E {
        /**
         * @return bad order but not a target
         * @param a x
         */
        A;
    }

    /**
     * @return x
     * @param a x
     */
    @interface Ann {
        /**
         * @return x
         * @param a x
         */
        int v();
    }

    void body() {
        /**
         * @return x
         * @param a x
         */
        int local = 1;
    }

    /**
     * @return x
     * @param a x
     */
    record R(int a) {
        /**
         * @return x
         * @param a x
         */
        R { }
    }
}
