package p;

// bare_param_then_tag
/**
 * Text.
 * @param
 * @return x
 */
class C0 { }

// bare_param_then_end
/**
 * Text.
 * @param
 */
class C1 { }

// bare_param_then_text
/**
 * Text.
 * @param
 * more text
 */
class C2 { }

// bare_return
/**
 * Text.
 * @return
 * @param <T> t
 */
class C3 { }

// bare_return_end
/**
 * Text.
 * @return
 */
class C4 { }

// bare_see
/**
 * Text.
 * @see
 * @since 1
 */
class C5 { }

// bare_since_end
/**
 * Text.
 * @since
 */
class C6 { }

// bare_throws
/**
 * Text.
 * @throws
 * @since 1
 */
class C7 { }

// serialfield_end
/**
 * Text.
 * @serialField f
 */
class C8 { }

// serialfield_full
/**
 * Text.
 * @serialField f int desc
 */
class C9 { }

// pre_override
/**
 * Text.
 * <pre>
 * @Override
 * </pre>
 */
class C10 { }

// pre_override_noclose
/**
 * Text.
 * @Override
 */
class C11 { }

// pre_override_html
/**
 * <pre>
 * @Override
 * void m() {}
 * </pre>
 */
class C12 { }

// unclosed_inline_mid
/**
 * Text {@link Foo
 * more
 */
class C13 { }

// unclosed_brace_text
/**
 * Text { not inline
 */
class C14 { }

// html_comma
/**
 * Use <A, B> here.
 */
class C15 { }

// html_unclosed
/**
 * <b>bold
 */
class C16 { }

// html_stray_close
/**
 * </b> stray
 */
class C17 { }

// html_lt_space
/**
 * a < b and c > d
 */
class C18 { }

// html_amp
/**
 * a && b &amp; c
 */
class C19 { }

// html_attr
/**
 * <a href="x">y</a>
 */
class C20 { }

// html_p_unclosed
/**
 * <p>one
 * <p>two
 */
class C21 { }

// html_li
/**
 * <ul><li>a<li>b</ul>
 */
class C22 { }

// at_midline
/**
 * mail me@example.com
 */
class C23 { }

// at_start_text
/**
 * @ alone
 */
class C24 { }

// firstline_tag
/** @author a */
class C25 { }

// empty_tag
/**
 * @
 */
class C26 { }

// gen_brackets
/**
 * List<String> and Map<K,V>
 */
class C27 { }

// gen_brackets2
/**
 * {@code List<String>} and List<String>
 */
class C28 { }

// code_pre
/**
 * <pre>{@code
 * @Override
 * }</pre>
 */
class C29 { }
