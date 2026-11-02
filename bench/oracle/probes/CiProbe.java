// top-level ok
package probe;

 // top-level bad before import
import java.util.List;
   /* block bad before class */
// ok before class
public class CiProbe {
    // ok field comment
    int a;
  // bad field comment
    int b; // trailing ok
        // too deep
    int c;

    /* ok block */
   /* bad block */
    /**
     * javadoc ok
     */
    void m1() {
        int x = 1;
        // ok
        int y = 2;
      // bad
        int z = 3;
            // bad deep
        foo(x, y, z);
        // ok before closing
    }

    void m2() {
        foo();
          // bad before closing
    }

    void m3() {
            // bad in empty block
    }

    void m4() {
    // ok in empty block? col 4
    }

    void m5() {
        // ok in empty block
    }

    void m6(int k) {
        switch (k) {
            case 1:
                foo();
                // ok fall through
            case 2:
                foo();
              // bad fall through
            case 3:
                foo();
                break;
        // bad before default
            default:
                foo();
        }
    }

    void m7(int k) {
        switch (k) {
            // ok before first case
            case 1:
                // ok in case
                foo();
                break;
          // bad before case
            case 2:
            // bad? in empty case
            case 3:
                foo();
                // end of last case ok
        }
    }

    void m8(int k) {
        switch (k) {
            case 1:
              // bad first in case
                foo();
                break;
            case 2:
                // ok

                // ok after blank
                foo();
                // trailing at end
                // second
        }
    }

    void m9(int k) {
        switch (k) {
            case 1:
                foo();
          // bad at end of last case
        }
    }

    void m10(boolean f) {
        if (f) {
            foo();
        // bad before else
        } else {
            foo();
          // bad end of else
        }
        // ok after if
        foo();
    }

    void m11(boolean f) {
        if (f) {
            foo();
            // ok
        }
        // comment before else if
        else if (f) {
            foo();
        }
    }

    void m12() {
        try {
            foo();
        // bad before catch
        } catch (RuntimeException e) {
            foo();
          // bad before finally
        } finally {
            foo();
        }
    }

    void m13() {
        try {
            foo();
        } catch (RuntimeException e) {
            foo();
        // bad before finally same line
        } finally {
            foo();
        }
    }

    void m14() {
        foo(1,
            // ok in args
            2,
          // bad in args
            3);
        foo(1,
                // deeper in args
            2);
    }

    void m15() {
        String s = "a"
            // ok in plus
            + "b"
          // bad in plus
            + "c";
        int q = foo(2)
            // chain
            .bar()
          // bad chain
            .baz();
    }

    void m16() {
        return;
        // comment after return
    }

    int m17() {
        return foo(1,
          // bad inside return
                   2);
    }

    void m18() {
        throw new RuntimeException(
            // ok in throw
            "x");
    }

    void m19() {
        throw
            // after throw keyword
            new RuntimeException("x");
    }

    void m20() {
        int[] arr = {
            1,
            // ok in array init
            2,
          // bad in array init
            3
            // end ok
        };
        int[] arr2 = {
          // bad first in init
            1
        };
        int[] arr3 = {
        };
        int[] arr4 = {
            // empty with comment
        };
        int[] arr5 = {
                // empty bad
        };
    }

    void m21() {
        Runnable r = () -> {
            foo();
          // bad in lambda
        };
        Runnable r2 = () -> {
              // bad empty lambda
        };
        Runnable r3 = new Runnable() {
            // ok anon
            public void run() {
              // bad anon
            }
          // bad end anon
        };
    }

    void m22() {
        for (int i = 0; i < 2; i++) {
          // bad first in for
            foo();
        }
        while (true) {
            // ok
            foo();
        }
        do {
            foo();
          // bad in do
        } while (false);
    }

    void m23() {
        /* block ok */
        foo(); /* trailing block */
        /* block */ foo();
       /* bad block */ foo();
        /*
         * multi-line ok
         */
        foo();
      /*
       * multi-line bad
       */
        foo();
        foo();
    /* block at end bad */
    }

    void m24() {
        foo(); // trailing
        // normal
        foo();
        // c1
        // c2
          // c3 bad
        foo();
          // c4 bad then ok
        // c5
        foo();
    }

    // comment before method ok
  // comment before method bad
    void m25() {
    }

  // bad before closing of class
}

// after class ok
 // after class bad

class Other {
    // ok

    // ok after blank
        // bad after blank
    void m() { }

    enum E {
        A,
      // bad in enum
        B;
        // ok
    }

    interface I {
        // ok
        void f();
      // bad in interface
    }

    @interface An {
      // bad in annotation
        int v();
    }

    record R(int a) {
      // bad record
        R {
          // bad compact
        }
    }
}

class Tabs {
	// tab ok
	void m() {
		foo();
	// bad tab comment
		foo();
	  // mixed tab space
		foo();
	}
        // spaces after tabs
}

class Misc {
    void m(int k) {
        label:
          // bad after label
        foo();
        if (k > 0) // trailing on if
        {
            foo();
        }
        // comment then else
        if (k > 1) foo();
        // comment between
        else foo();
        int v =
            // value on next
            3;
        int w =
          // bad value
            3;
        for (;;)
            // body comment
            foo();
        foo(
            // first arg comment
            1);
        new Object() {
            // anon body
        };
        {
        // block bad
        }
        synchronized (this) {
        // sync bad
            foo();
        }
    }

    int m2() {
        return 1
            // after return value
            ;
    }

    int m3(int k) {
        return switch (k) {
            // in switch expr ok
            case 1 -> 1;
          // bad in switch expr
            case 2 -> {
                // ok in rule block
                yield 2;
            }
            default -> 3;
            // last ok
        };
    }

    void m4(Object o) {
        switch (o) {
            case String s when s.isEmpty() ->
                // after arrow
                foo();
            default -> {
            }
        }
    }

    void m5() {
        foo();
        // comment, then a lone brace block
        {
            foo();
        }
    }

    void m6() {
        String t = """
            text block
            // not a comment
            """;
        // after text block
        foo();
      // bad after text block
        foo();
    }

    void m7() {
        foo(); /* a */ /* b */
        /* c */ /* d */
        foo();
    }

    void m8() {
        if (true) {
            foo();
        } // trailing after brace
        // after trailing
        else {
            foo();
        }
    }

    void m9() { // trailing after open
        // ok first
        foo();
    }

    void m10() {
        foo();
    } // trailing after close
    // between methods ok
    void m11() {
    }
    void foo() {}
}
