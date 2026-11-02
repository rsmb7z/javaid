  // bad before package at top
package probe;
import java.util.*;
public class CiProbe2 {
    void sw1(int k) {
        switch (k) {
            case 1:
                foo();
              // bad after foo then case
            case 2: {
                foo();
            }
          // bad after block case
            case 3:
                if (k > 1) {
                    foo();
                }
        // bad after if then case
            case 4:
                for (;;) {
                    foo();
                }
                // ok after for
            case 5:
                foo(1,
                    2);
         // bad after multiline call
            case 6:
                int v = foo(1,
                        2);
         // bad after multiline decl
            case 7:
                foo().bar()
                    .baz();
         // bad after chain
            default:
                break;
        }
    }

    void sw2(int k) {
        switch (k) {
            case 1:
            // empty case comment ok? col same as case
            case 2:
                // inner ok
            case 3:
                  // bad deep empty case
            default:
        // bad before default
        }
    }

    void sw3(int k) {
        switch (k) {
            case 1:
            case 2:
                foo();
                break;
            // ok before default
            default:
                foo();
                // ok last
            // c2 at case level
        }
    }

    void sw4(int k) {
        switch (k) {
            case 1 -> {
                foo();
              // bad end of rule block
            }
            case 2 ->
              // bad after arrow
                foo();
            case 3 -> foo();
            // ok between rules
            default -> {
            // bad empty rule
            }
        }
    }

    void sw5(int k) {
        switch (k) {
            case 1:
                try {
                    foo();
                } finally {
                    foo();
                }
              // bad after try in case
            case 2:
                while (true) {
                    foo();
                }
              // bad after while in case
            case 3:
                do {
                    foo();
                } while (false);
              // bad after do in case
            case 4:
                switch (k) {
                    default:
                        foo();
                }
              // bad after nested switch
            case 5:
                return;
              // bad after return in case
            case 6:
                throw new RuntimeException(
                    "x");
              // bad after throw in case
            default:
        }
    }

    void bl() {
        foo(); // trailing

        // after blank ok

          // after blank bad
        foo();


        /* two blanks before ok
           more */
        foo();
        /* before block */
        /* second block */
        foo();
        /* a */ /* b */ foo();
        foo();
          /* bad
           * lines */
        foo();
    }

    void em() {
        // only comment ok
    }

    void em2() {
      // only comment bad
    }

    void em3() {

        // only comment with blanks before
    }

    void em4() {
        /* block only */
    }

    void em5() {
     /* block only bad */
    }

    void em6() {
        foo();

          // bad after blank before close

    }

    void cc() {
        if (true) {
            foo();
        }
          // bad before else if
        else if (false) {
            foo();
        }
        // ok before else
        else {
            foo();
        }
        try {
            foo();
        }
          // bad before catch
        catch (RuntimeException e) {
            foo();
        }
        // ok before finally
        finally {
            foo();
        }
    }

    void tr() {
        try {
            foo();
          // bad at end of try
        } catch (RuntimeException e) { foo(); }
        try {
            foo();
        } catch (RuntimeException e) {
            foo();
          // bad end of catch
        } finally { foo(); }
        try {
            foo();
        } catch (RuntimeException e) {
            foo();
          // bad end of catch at the closing line
        } catch (Error e) {
            foo();
        }
        try {
            foo();
            // ok try end
        } catch (RuntimeException e) { }
        if (true) {
            foo();
          // bad end of if same-line else
        } else { foo(); }
        if (true) { foo(); } else {
            foo();
          // bad end of else
        }
    }

    void ch() {
        String s = foo().bar()
            .baz()
          // bad in chain
            .qux();
        String t = foo()
                   // aligned to dot?
                   .bar();
        foo(1, 2,
            // arg
            3)
            // after args
            .bar();
        list.stream()
            // c
            .map(x -> x)
            .filter(x -> {
                // lambda body ok
                return true;
              // bad lambda end
            })
            .count();
    }

    void ex() {
        int a = 1 +
            // after plus
            2 +
          // bad after plus
            3;
        boolean b = true &&
            // after and
            false;
        int c = true
            // before question
            ? 1
          // bad before colon
            : 2;
        Object o = (Object)
            // after cast
            foo();
        foo(new int[] {
            1, 2,
            // in new array
            3
        });
        foo(new Object() {
            // anon in args
            int x;
          // bad anon in args
        });
    }

    @Override
    // between annotation and method
    public String toString() {
        return "";
    }

    // before annotated
    @SuppressWarnings("x")
          // bad between annotation and field
    int field;

    private
        // within modifiers
        int field2;

    int f3 = 1; /* trailing block */ // another trailing
    int f4 = 2; /* trailing block */
      // bad after trailing
    int f5;

    enum En {
        // ok first
        A,
        // between constants
        B,
      // bad between constants
        C(1) {
            // body ok
            void x() { }
          // bad
        };
        En() { }
        En(int a) { }
        // before end
    }

    void foo() { }
    // last in class ok
          // last in class bad
}
  // after class bad at EOF
