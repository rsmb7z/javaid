package p;

import java.io.IOException;

public class JmProbe2 {
    /**
     * Enum ctors.
     */
    public enum E {
        A;
        /** Doc. */
        E() { }
        /** Doc. */
        public E(int x) { }
        /** Doc. */
        private E(int x, int y) { }
    }

    /**
     * Throws variants.
     * @param a the a
     * @return one
     * @throws java.io.IOException qualified
     * @exception IllegalStateException simple
     */
    public int qualified(int a) throws IOException, java.lang.IllegalStateException {
        throw new java.lang.IllegalArgumentException("x");
    }

    /**
     * Throws in try.
     * @param a the a
     * @return one
     */
    public int tries(int a) {
        try {
            throw new IllegalStateException();
        } catch (RuntimeException e) {
            throw new UnsupportedOperationException();
        } finally {
            throw new ArithmeticException();
        }
    }

    /**
     * Try without catch.
     * @param a the a
     * @return one
     */
    public int tryNoCatch(int a) {
        try (java.io.Reader r = null) {
            throw new IllegalStateException();
        }
    }

    /**
     * Lambda and anonymous.
     * @param a the a
     * @return one
     */
    public int lambdas(int a) {
        Runnable r = () -> { throw new IllegalStateException(); };
        Object o = new Object() { void m() { throw new IllegalStateException(); } };
        class Local { void m() { throw new IllegalStateException(); } }
        throw new Error();
    }

    /**
     * Rethrow.
     * @param a the a
     * @return one
     */
    public int rethrow(int a, Exception e) throws Exception {
        throw e;
    }

    /**
     * Multi line.
     * @param a
     *        the a
     * @param
     *        b
     * @return
     *        one
     */
    public int multi(int a, int b) { return 1; }

    /**
     * Bounded.
     * @param <T> the t
     * @param a the a
     * @return a
     */
    public <T extends Comparable<T>> T bounded(T a) { return a; }

    /**
     * Annotated params.
     * @param a the a
     * @param b the b
     * @return x
     */
    public int annotated(@Deprecated final int a, @Deprecated int... b) { return 1; }

    /**
     * Receiver.
     * @param a the a
     * @return x
     */
    public int receiver(JmProbe2 this, int a) { return 1; }

    /**
     * Pre block.
     * <pre>
     * @param ignored? no, a custom tag
     * </pre>
     * @param a the a
     * @return x
     */
    public int pre(int a) { return 1; }

    /**
     * Code block.
     * {@code
     * @param a not a tag
     * }
     * @param a the a
     * @return x
     */
    public int code(int a) { return 1; }

    /**
     * Inline tags {@link #code(int)} and {@literal @param} here.
     * @param a the a
     * @return x
     */
    public int inline(int a) { return 1; }

    /**
     * Text mentioning @return mid line and @param too.
     */
    public int midline(int a) { return 1; }

    /**
     * Static inherit.
     * {@inheritDoc}
     * @return x
     */
    public static int staticInheritPlus() { return 1; }

    /**
     * Value.
     * @return {@code x}
     */
    public int valueReturn() { return 1; }

    /**
     * Tab	indented
     * @param	a	tabbed
     * @return	one
     */
    public int tabs(int a) { return 1; }

    /**
     * @param a the a
     * @return x
     */
    @SuppressWarnings("x") @Deprecated
    public int annotatedMethod(int a) { return 1; }

    /** One line @param a and @return both mid-line. */
    public int oneLine(int a) { return 1; }

    /**
     * @return x
     */
    public final int fin() { return 1; }

    /**
     * Interface in class.
     */
    public interface I {
        /**
         * Doc.
         */
        int m(int a);
        /**
         * Doc.
         * @param a the a
         * @return x
         */
        private int p(int a) { return a; }
    }

    /**
     * Unclosed html.
     * <b>bold
     * @param a the a
     * @return x
     */
    public int unclosed(int a) { return 1; }

    /**
     * Generic text <T> here.
     * @param a the a
     * @return x
     */
    public int genericText(int a) { return 1; }

    /**
     * Bad comma <A, B> here.
     * @param a the a
     */
    public int badComma(int a) { return 1; }
}

/**
 * Top-level record.
 */
public record JmTopRecord(int a) {
    /** Doc. */
    public JmTopRecord { }
    /** Doc. */
    public int twice(int x) { return a * x; }
}

class JmThrowTwice {
    /**
     * Same exception thrice.
     */
    public void thrice(int a) throws java.io.IOException {
        if (a == 1) { throw new IllegalStateException("a"); }
        if (a == 2) { throw new IllegalStateException("b"); }
        if (a == 3) { throw new java.io.IOException("c"); }
        throw new IllegalStateException("d");
    }
}

class JmPackageOuter {
    public static class PublicInner {
        /** Doc. */
        public int pub(int a) { return a; }
        /** Doc. */
        int pkg(int a) { return a; }
        public static class Deeper {
            /** Doc. */
            public int deep(int a) { return a; }
        }
    }
    private static class PrivateInner {
        /** Doc. */
        public int priv(int a) { return a; }
        public class PublicInPrivate {
            /** Doc. */
            public int pip(int a) { return a; }
        }
    }
    /** Doc. */
    public int outer(int a) { return a; }
    /**
     * Throws in catches.
     */
    public void catches() {
        try {
            run();
        } catch (Exception e) {
            throw new AssertionError("first", e);
        } catch (Error e) {
            throw new AssertionError("second", e);
        }
    }
}
