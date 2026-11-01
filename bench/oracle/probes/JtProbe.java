package p;

/**
 * Plain.
 */
public class JtProbe<T, U> {
}

/**
 * Generic with tags.
 * @param <T> the t
 * @param <U> the u
 * @author someone
 * @version 1.0
 * @since 1.0
 * @see Other
 * @deprecated no
 */
class Tagged<T, U> { }

/**
 * Missing and unused.
 * @param <T> the t
 * @param <X> not a parameter
 * @param
 * @param <T> duplicate
 */
class Mixed<T, U> { }

/**
 * Unknown tags.
 * @apiNote note
 * @implSpec spec
 * @custom thing
 * @author a
 * @Override
 */
class Unknown { }

/**
 * Record.
 * @param a the a
 * @param c not a component
 */
record Rec(int a, int b) { }

/** One line. */
interface Iface<T> { }

/**
 * Enum.
 * @param <T> nope
 */
enum En { A }

/**
 * Anno.
 */
@interface An { }

/* not javadoc */
class NoDoc<T> { }

class NoComment<T> { }

/**
 * Generated skipped.
 * @param <Q> bad
 */
@Generated
class Gen<T> { }

/**
 * Inner.
 * @param <T> t
 */
public class Outer<T> {
    /**
     * Inner class.
     * @author inner
     * @version 2
     * @param <Z> bad
     */
    static class Inner<Z, Y> { }
    /** @author inline */
    private class Priv { }
    /**
     * Braces {@code @param <T> not a tag} and {@link Foo}
     * <pre>
     * @Override
     * </pre>
     * text @param <Y> mid-line not a tag
     */
    class Code<Y> { }
}

/**
 * {@inheritDoc}
 * @return nothing
 * @throws Exception nothing
 * @exception IOException nothing
 * @serial include
 * @since 2
 * @hidden
 * @provides X
 * @uses Y
 * @see #foo
 * @serialData data
 * @serialField f
 */
class StdTags { }

/**
 * @param <T>
 * @author
 * @version
 */
class EmptyDesc<T> { }

/**
   * Weird   indent
 *@author nospace
	 * @since tab
 */
class Weird { }

/**
 * Unclosed {@code oops
 * @author a
 */
class Unclosed { }

/** @author x @version y */
class SameLine { }
