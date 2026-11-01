package probe;

/** Probe for JcProbe. */
public class JcProbe {

    /**
     * Summary.
     *
     * @param x the x
     *     continued ok
     */
    void m1() {}

    /**
     * Summary.
     *
     * @param x the x
     *    three spaces only
     */
    void m2() {}

    /**
     * Summary.
     *
     * @param x the x
     *   two spaces
     */
    void m3() {}

    /**
     * Summary.
     *
     * @param x the x
     * no indentation
     */
    void m4() {}

    /**
     * Summary.
     *
     * @param x the x
     *        eight spaces
     */
    void m5() {}

    /**
     * Summary.
     *
     * @param x the x
     *     five spaces total
     *     second continuation
     *   bad third
     */
    void m6() {}

    /**
     * Summary.
     *
     * @return the thing
     *     continued ok
     */
    void m7() {}

    /**
     * Summary.
     *
     * @return the thing
     * not indented
     */
    void m8() {}

    /**
     * Summary.
     *
     * @throws IOException if
     *     continued
     */
    void m9() {}

    /**
     * Summary.
     *
     * @throws IOException if
     * not indented
     */
    void m10() {}

    /**
     * Summary.
     *
     * @param x
     *     description on next line
     */
    void m11() {}

    /**
     * Summary.
     *
     * @param x
     * description on next line no indent
     */
    void m12() {}

    /**
     * Summary.
     *
     * @param x
     *   description two spaces
     */
    void m13() {}

    /**
     * Summary.
     *
     * @return
     *     description on next line
     */
    void m14() {}

    /**
     * Summary.
     *
     * @return
     * description no indent
     */
    void m15() {}

    /**
     * Summary.
     *
     * @param x the x
     *
     *     after blank star line
     */
    void m16() {}

    /**
     * Summary.
     *
     * @param x the x
     *
     * after blank star line no indent
     */
    void m17() {}

    /**
     * Summary.
     *
     * @param x {@code code
     *     spanning} lines
     */
    void m18() {}

    /**
     * Summary.
     *
     * @param x {@code code
     * spanning no indent} lines
     */
    void m19() {}

    /**
     * Summary.
     *
     * @param x the x {@link Foo
     *     label}
     */
    void m20() {}

    /**
     * Summary.
     *
     * @param x the x
     *     {@code startcode} on continuation
     */
    void m21() {}

    /**
     * Summary.
     *
     * @param x the x
     * {@code startcode} on continuation no indent
     */
    void m22() {}

    /**
     * Summary.
     *
     * @param x
     *     {@code startcode} on next line
     */
    void m23() {}

    /**
     * Summary.
     *
     * @param x
     * {@code startcode} on next line no indent
     */
    void m24() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <b>bold start</b> continuation
     */
    void m25() {}

    /**
     * Summary.
     *
     * @param x the x
     * <b>bold start no indent</b> continuation
     */
    void m26() {}

    /**
     * Summary.
     *
     * @param x
     *     <b>bold start</b> on next line
     */
    void m27() {}

    /**
     * Summary.
     *
     * @param x
     * <b>bold start no indent</b> on next line
     */
    void m28() {}

    /**
     * Summary.
     *
     * @param x the x
     *     text <b>bold
     *     spanning</b> lines
     */
    void m29() {}

    /**
     * Summary.
     *
     * @param x the x
     *     text <b>bold
     * spanning no indent</b> lines
     */
    void m30() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <ul>
     *       <li>item
     *     </ul>
     */
    void m31() {}

    /**
     * Summary.
     *
     * @param x the x
     * <ul>
     *   <li>item
     * </ul>
     */
    void m32() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <pre>
     * code in pre
     *   more code
     *     </pre>
     */
    void m33() {}

    /**
     * Summary.
     *
     * @param x the x
     * <pre>
     * code in pre no indent
     * </pre>
     */
    void m34() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <pre>{@code
     * code in pre
     * }</pre>
     */
    void m35() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <a href="x"
     *        target="y">link</a>
     */
    void m36() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <a href="x"
     * target="y">link</a>
     */
    void m37() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <a href=
     * "x">link</a>
     */
    void m38() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <a
     *     href="x">link</a>
     */
    void m39() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <p>paragraph
     * not indented in paragraph
     */
    void m40() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <p>paragraph
     *     indented in paragraph
     */
    void m41() {}

    /**
     * Summary.
     *
     * @see Foo
     *     continued see
     */
    void m42() {}

    /**
     * Summary.
     *
     * @see Foo
     * not indented see
     */
    void m43() {}

    /**
     * Summary.
     *
     * @see <a href="x">link</a>
     *     continued
     */
    void m44() {}

    /**
     * Summary.
     *
     * @see <a href="x">link</a>
     * not indented
     */
    void m45() {}

    /**
     * Summary.
     *
     * @since 1.0
     *     continued
     */
    void m46() {}

    /**
     * Summary.
     *
     * @since 1.0
     * not indented
     */
    void m47() {}

    /**
     * Summary.
     *
     * @deprecated use y
     *     continued
     */
    void m48() {}

    /**
     * Summary.
     *
     * @deprecated use y
     * not indented
     */
    void m49() {}

    /**
     * Summary.
     *
     * @author me
     * not indented
     */
    void m50() {}

    /**
     * Summary.
     *
     * @custom tag
     * not indented custom
     */
    void m51() {}

    /**
     * Summary.
     *
     * @custom tag
     *     indented custom
     */
    void m52() {}

    /**
     * Summary.
     *
     * @param x the x
     *	tab indented
     */
    void m53() {}

    /**
     * Summary.
     *
     * @param x the x
     *	 tab then space
     */
    void m54() {}

    /**
     * Summary.
     *
     * @param x the x
     * 	tab after star
     */
    void m55() {}

    /**
     * Summary.
     *
     * @param x the x
     *     space ok
     * @param y the y
     * not indented
     */
    void m56() {}

    /**
     * Summary.
     *
     * @param x the x
     *     ok
     *
     * @param y the y
     *     ok
     */
    void m57() {}

    /**
     * Summary.
     * continuation text before tag is not checked
     *
     * @param x the x
     */
    void m58() {}

    /**
     * Summary text on lines
     * not indented
     *   random
     */
    void m59() {}

    /**
     * Summary.
     *
     * <ul>
     *   <li>not in block tag
     * </ul>
     */
    void m60() {}

    /**
     * Summary.
     *
     * @param x the x
     *     text {@link Foo#bar(int,
     *     String) label} more
     */
    void m61() {}

    /**
     * Summary.
     *
     * @param x the x
     *     text {@link Foo#bar(int,
     * String) label} more
     */
    void m62() {}

    /**
     * Summary.
     *
     * @param x the x
     *     text {@code a
     *     b
     * c}
     */
    void m63() {}

    /**
     * Summary.
     *
     * @param x the x
     *     text
     *
     *     para
     */
    void m64() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <p>
     *     para
     */
    void m65() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <p>
     * para no indent
     */
    void m66() {}

    /**
     * Summary.
     *
     * @param x the x
     *      six spaces
     *       seven
     *    three
     */
    void m67() {}

    /**
     * Summary.
     *
     * @return the x
     *     <table>
     *       <tr><td>a</td></tr>
     *     </table>
     */
    void m68() {}

    /**
     * Summary.
     *
     * @return the x
     * <table>
     *   <tr><td>a</td></tr>
     * </table>
     */
    void m69() {}

    /**
     * Summary.
     *
     * @return <ul>
     *     <li>a
     *     <li>b
     *     </ul>
     */
    void m70() {}

    /**
     * Summary.
     *
     * @return <ul>
     * <li>a
     * <li>b
     * </ul>
     */
    void m71() {}

    /**
     * Summary.
     *
     * @param <T>
     *     generic param
     */
    void m72() {}

    /**
     * Summary.
     *
     * @param <T> generic param
     * no indent
     */
    void m73() {}

    /**
     * Summary.
     *
     * @throws IOException {@code if
     *     bad}
     */
    void m74() {}

    /**
     * Summary.
     *
     * @throws IOException
     *
     * @param x the x
     * no indent
     */
    void m75() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <img src="a.png"
     * alt="x">
     */
    void m76() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <br>
     * <br>
     */
    void m77() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <!-- comment
     * continues -->
     */
    void m78() {}

    /**
     * Summary.
     *
     * @param x the x
     * <!-- comment -->
     */
    void m79() {}

    /** @param x the x
     *     continued ok
     */
    void m80() {}

    /** @param x the x
     * not indented
     */
    void m81() {}

    /** @return x
     */
    void m82() {}

    /**
     * Summary.
     *
     * @param x the x
     *     ok
     *     <b>bold</b>
     *   <i>italic</i>
     */
    void m83() {}

    /**
     * Summary.
     *
     * @param x the x
     *     <ul><li>a
     *   <li>b</ul>
     */
    void m84() {}
}
