package probes;

public class JlProbe2 {
	/**
	 * Tabs.
	 * @return x
	 */
	int a() { return 0; }

	/** Tabs open
	   * mixed
	 * ok
	 */
	int b() { return 0; }

	/**
	* flush
		 * deep
	 */
	int c() { return 0; }

    /**
	 * mixed
     * ok
     */
    int d() { return 0; }

    /**
     * Not attached.
     * @return x
     */

    /**
     * Another.
    * x
     */
    int e() { return 0; }

    /** Generic. */
    <T> /**
     * odd.
    * x
     */ int f() { return 0; }

    /**
     * Trailing.
     * @return x
     * @param a b */
    int g(int a) { return 0; }

    /**
     * Unclosed {@code
     * foo.
     */
    int h() { return 0; }

    /***
     * Triple star open.
     */
    int i() { return 0; }

    /***
      * Triple star, off.
     */
    int j() { return 0; }
}
