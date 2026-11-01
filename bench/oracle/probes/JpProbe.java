package probe;

/** Probe for JpProbe. */
public class JpProbe {

    /**
     * Summary.
     *
     * <p>Paragraph.
     */
    void m1() {}

    /**
     * Summary.
     * <p>Paragraph without blank line.
     */
    void m2() {}

    /**
     * Summary.
     *
     * Paragraph without p tag.
     */
    void m3() {}

    /**
     * Summary.
     *
     * <p> Paragraph with space after p.
     */
    void m4() {}

    /**
     * Summary.
     *
     * <p>
     * Paragraph with newline after p.
     */
    void m5() {}

    /**
     * <p>First paragraph is redundant.
     */
    void m6() {}

    /**
     * <p>
     * First paragraph with newline.
     */
    void m7() {}

    /**
     * Summary.
     *
     * <p>One.
     *
     * <p>Two.
     *
     * Three without p.
     */
    void m8() {}

    /**
     * Summary.
     *
     *
     * <p>Two blank lines before.
     */
    void m9() {}

    /**
     * Summary.
     *
     * <p><ul>
     *   <li>list
     * </ul>
     */
    void m10() {}

    /**
     * Summary.
     *
     * <p>
     * <ul>
     *   <li>list
     * </ul>
     */
    void m11() {}

    /**
     * Summary.
     *
     * <p><pre>
     * code
     * </pre>
     */
    void m12() {}

    /**
     * Summary.
     *
     * <p><table><tr><td>x</td></tr></table>
     */
    void m13() {}

    /**
     * Summary.
     *
     * <p><h2>Head</h2>
     */
    void m14() {}

    /**
     * Summary.
     *
     * <p><div>block</div>
     */
    void m15() {}

    /**
     * Summary.
     *
     * <p><blockquote>quote</blockquote>
     */
    void m16() {}

    /**
     * Summary.
     *
     * <p><dl><dt>a<dd>b</dl>
     */
    void m17() {}

    /**
     * Summary.
     *
     * <p><ol><li>a</ol>
     */
    void m18() {}

    /**
     * Summary.
     *
     * <p><hr>
     */
    void m19() {}

    /**
     * Summary.
     *
     * <p><address>addr</address>
     */
    void m20() {}

    /**
     * Summary.
     *
     * <p><b>bold</b> is not a block tag.
     */
    void m21() {}

    /**
     * Summary.
     *
     * <p><p>Double p.
     */
    void m22() {}

    /**
     * Summary.
     *
     * <p>Text <ul><li>inside</ul>
     */
    void m23() {}

    /**
     * Summary.
     *
     * <p><!-- comment --><ul><li>x</ul>
     */
    void m24() {}

    /**
     * Summary.
     *
     * <p><!-- comment -->Text
     */
    void m25() {}

    /**
     * Summary.
     *
     * <p><!-- comment -->
     * Text
     */
    void m26() {}

    /**
     * Summary.
     * @param x <p>text in param
     */
    void m27() {}

    /**
     * Summary.
     *
     * @param x the x
     *
     * <p>after block tag
     */
    void m28() {}

    /**
     * Summary.
     *
     * @param x the x.
     *     <p>more
     */
    void m29() {}

    /**
     * Summary.
     *
     * <div>
     *   <p>nested paragraph
     *
     *   <p>another nested
     * </div>
     */
    void m30() {}

    /**
     * Summary.
     *
     * <ul>
     *   <li>one
     *
     *   <li>two
     * </ul>
     */
    void m31() {}

    /**
     * Summary.
     *
     * <ul>
     *   <li>one
     *
     *   text after blank in list
     * </ul>
     */
    void m32() {}

    /**
     * Summary.
     *
     * <pre>
     * code
     *
     * more code
     * </pre>
     */
    void m33() {}

    /**
     * Summary.
     *
     * <b>bold start of paragraph</b> without p.
     */
    void m34() {}

    /**
     * Summary.
     *
     * {@code code start} without p.
     */
    void m35() {}

    /**
     * Summary.
     *
     * {@link Foo} start without p.
     */
    void m36() {}

    /**
     * Summary.
     *
     *    indented text without p.
     */
    void m37() {}

    /**
     * Summary.
     *
     * @return x
     */
    void m38() {}

    /**
     * Summary.
     *
     *
     * @return x
     */
    void m39() {}

    /**
     * Summary.
     *
     * <p/>
     * After self closing.
     */
    void m40() {}

    /**
     * Summary.
     *
     * <p></p>
     * After empty p.
     */
    void m41() {}

    /**
     * Summary.
     *
     * <p></p>After empty p same line.
     */
    void m42() {}

    /**
     * Summary.
     *
     * <P>Upper case p.
     */
    void m43() {}

    /**
     * Summary.
     *
     * <P><UL><LI>x</UL>
     */
    void m44() {}

    /**
     * Summary.
     *
     * <p>Para one.
     * <p>Para two without blank.
     */
    void m45() {}

    /**
     * Summary.
     *
     * <p>Para one.</p>
     *
     * <p>Para two.</p>
     */
    void m46() {}

    /**
     * Summary.
     *
     * <p>Para one.</p>
     * <p>Para two without blank.</p>
     */
    void m47() {}

    /** Single line. */
    void m48() {}

    /** <p>Single line p. */
    void m49() {}

    /** Summary. <p>Inline p. */
    void m50() {}

    /**
     * Summary. <p>Inline p on first line.
     */
    void m51() {}

    /**
     * Summary.
     *
     * Text one.
     *
     * Text two.
     */
    void m52() {}

    /**
     * Summary.
     *
     * Text with trailing blank
     *
     */
    void m53() {}

    /**
     * Summary.
     *
     */
    void m54() {}

    /**
     *
     * Starts with blank.
     */
    void m55() {}

    /**
     *
     * <p>Starts with blank then p.
     */
    void m56() {}

    /**
     * Summary.
     *
     *
     *
     * Several blanks.
     */
    void m57() {}

    /**
     * Summary.
     *
     * Tab-only blank line.
     */
    void m58() {}

    /**
     * Summary.
     *
     * <p>Spaces-only blank line.
     */
    void m59() {}

    /**
     * Summary.
     *
     * <p>Text
     * continues
     *
     * <p>Next
     */
    void m60() {}

    /**
     * Summary.
     * <ul>
     *   <li>item
     * </ul>
     *
     * Text after list.
     */
    void m61() {}

    /**
     * Summary.
     * <ul>
     *   <li>item
     * </ul>
     *
     * <p>Text after list.
     */
    void m62() {}

    /**
     * Summary.
     *
     * <p>Text with {@code code
     * spanning} lines.
     */
    void m63() {}

    /**
     * Summary.
     *
     * <p>{@code startcode} para.
     */
    void m64() {}

    /**
     * Summary.
     *
     * <p>{@code startcode}
     */
    void m65() {}

    /**
     * Summary.
     *
     * <p> {@code startcode with space}
     */
    void m66() {}

    /**
     * Summary.
     *
     * <p>
     */
    void m67() {}

    /**
     * Summary.
     *
     * <p>
     *
     * Text after blank after p.
     */
    void m68() {}

    /**
     * Summary.
     *
     * <p>
     * Trailing spaces after p.
     */
    void m69() {}

    /**
     * Summary.
     *
     * <p>Text</p> trailing
     */
    void m70() {}

    /**
     * Summary.
     *
     * <p>Text
     * <ul>
     *   <li>x
     * </ul>
     */
    void m71() {}

    /**
     * Summary.
     *
     * <p>Text
     * <ul><li>x</ul>
     *
     * <p>Next
     */
    void m72() {}

    /**
     * Summary.
     *
     * <p>&lt;escaped&gt; text
     */
    void m73() {}

    /**
     * Summary.
     *
     * <p>
     * <b>bold after newline</b>
     */
    void m74() {}

    /**
     * Summary.
     *
     * <a href="x">link</a> start.
     */
    void m75() {}

    /**
     * Summary.
     *
     * <p><a href="x">link</a> start.
     */
    void m76() {}

    /**
     * @param x <p>Redundant? no, in block tag.
     *
     * <p>Out of block tag.
     */
    void m77() {}

    /**
     * Summary.
     * @since 1
     *
     * <p>After since
     */
    void m78() {}

    /**
     * Summary.
     *
     * <p>Line one
     *
     *
     * <p>After two blanks
     */
    void m79() {}

    /**
     * Summary.
     *
     * <p>
     * <p>Two p
     */
    void m80() {}

    /**
     * <p>
     * <p>Two p redundant
     */
    void m81() {}

    /**
     * <ul>
     * <li>
     * <p>Nested in list
     * </ul>
     */
    void m82() {}

    /**
     * Summary.
     *
     * <blockquote>
     * <p>nested in blockquote
     * </blockquote>
     */
    void m83() {}

    /**
     * Summary.
     *
     * <p>Text
     * <br>
     * <p>After br
     */
    void m84() {}

    /**
     * Summary.
     *
     * <p><br>
     */
    void m85() {}

    /**
     * Summary.
     *
     * <p><img src="x.png">
     */
    void m86() {}

    /**
     * Summary.
     *
     * <p><span>inline</span>
     */
    void m87() {}

    /**
     * Summary.
     *
     * <p><pre>{@code
     * code
     * }</pre>
     */
    void m88() {}
}
