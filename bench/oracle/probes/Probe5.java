package ok.pkg;

/** EmptyCatchBlock: which empty catch blocks are allowed by commentFormat / exceptionVariableName. */
public class Probe5 {
    void f() {
        try { g(); } catch (Exception e) {}
        try { g(); } catch (Exception e) { /* */ }
        try { g(); } catch (Exception e) { /* word */ }
        try {
            g();
        } catch (Exception e) {
            // good reason
        }
        try {
            g();
        } catch (Exception e) {
            // ---
        }
        try { g(); } catch (Exception ignored) {}
        try { g(); } catch (Exception ignored) { /* x */ }
        try { g(); } catch (Exception expected) {}
        try { g(); } catch (Exception other) {}
        try {
            g();
        } catch (Exception e) {
            /* first */
            // second
        }
        try {
            g();
        } catch (Exception e) {
            // ---
            /* real words */
        }
        try { g(); } catch (Exception e) { ; }
        try { g(); } catch (Exception | Error e) {}
        try {
            g();
        } catch (Exception e) {
            /* ignored */
        }
        try {
            g();
        } catch (Exception e) {
            // TODO
        }
    }

    void g() {}
}
