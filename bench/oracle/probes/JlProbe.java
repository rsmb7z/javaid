package probes;

/** One line. */
public class JlProbe {
    /** Starts on first line
     * and goes on.
     */
    void a() {}

    /**
     * Starts on second line.
     */
    void b() {}

    /**

     * Blank then content.
     */
    void c() {}

    /**
     *
     * Two lines down.
     */
    void d() {}

    /**
     */
    void e() {}

    /**
     * @return x
     */
    int f() { return 0; }

    /**
     * Text.
     * @return x
     */
    int g() { return 0; }

    /**
     * Text.
     *
     * @return x
     */
    int h() { return 0; }

    /** @return x */
    int i() { return 0; }

    /** Text. @return x */
    int j() { return 0; }

    /**
     * Text.
     * @param a one
     *
     * @return x
     */
    int k(int a) { return 0; }

    /**
     *
     * @return x
     */
    int l() { return 0; }

    /** Text.
     * @return x
     */
    int m() { return 0; }

    /**
     * Text.
     *
     *
     * @return x
     */
    int n() { return 0; }

    /**
     * Misaligned.
      * One right.
    * One left.
       * Far right.
     */
    void o() {}

      /**
       * Indented comment fine.
        * off by one right.
       */
      void p() {}

    /**
     * Closing off.
   */
    void q() {}

    /**
     * Closing right.
       */
    void r() {}

    /** Opening text
        * aligned with opening text?
     * normal
     */
    void s() {}

    /**  * star on opening
     * normal
        * other
     */
    void t() {}

    /**
     ** double
     * single
    */
    void u() {}

    /* not javadoc
       * text
     */
    void v() {}

    /**
       text without star
     * star
     */
    void w() {}

    /** Closing on text line
     * foo */
    void x() {}

    /**
     * Inside {@code
     *   code
     * } here.
     */
    void y() {}

    /**
	 * Tab indented.
	 */
    void z() {}
}
