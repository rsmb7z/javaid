/** no period at file start */
package probe;

import java.util.List;

/**
 * lowercase class summary
 *
 * Text without paragraph tag.
 * <p>
 * Paragraph.
 *
 * @param <T> the type
 * not indented
 */
public class JtPos<T> {

	/** tab-indented field without period */
	int field;

	/**
	 * Tab-indented method.
	 * <p>Paragraph without blank line.
	 *
	 * @param x the x
	 * not indented
	 * @return the thing
	 *     fine
	 */
	int tabbed(int x) {
		return x;
	}

  /** two-space indented lowercase */
  void twoSpaces() {}

	/** Javadoc inside a method body is not a Javadoc position. */
	void body() {
		/** not attached lowercase */
		int local = 0;
		/** another one
		 *
		 * text without p
		 */
		for (int i = 0; i < 1; i++) {}
	}

	/**
	 * Constructor summary without period
	 * @param x the x
	 *   two spaces
	 */
	JtPos(int x) {}

	/** Enum. */
	enum E {
		/** lowercase constant */
		ONE,
		/**
		 * Constant.
		 *
		 * Text without paragraph.
		 */
		TWO;

		/** enum method without period */
		void m() {}
	}

	/** interface lowercase */
	interface I {
		/** method {@inheritDoc} */
		void m();

		/** {@inheritDoc} */
		void n();
	}

	/** record lowercase */
	record R(int a) {
		/** compact */
		R {}
	}

	/** annotation lowercase */
	@interface A {
		/** member lowercase */
		int value() default 1;
	}

	/** Multiple
	 * annotations.
	 */
	@Deprecated
	@SuppressWarnings("unused")
	void annotated() {}

	@Deprecated
	/** javadoc after annotation lowercase */
	void afterAnnotation() {}

	/** first */ /** second lowercase */
	void twoComments() {}

	/* not a javadoc lowercase */
	void blockComment() {}

	// line comment lowercase
	void lineComment() {}

	/**
	 * Generic method.
	 *
	 * @param <U> the u
	 *     ok
	 * @param u the u
	 * not ok
	 */
	<U> void generic(U u) {}

	/** Static init. */
	static {
	}

	/** instance init lowercase */
	{
	}
}

/** Second top-level lowercase. */
class Second {}

/** dangling at the end lowercase */
