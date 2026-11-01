package ok.pkg;

/** What text of a comment does EmptyCatchBlock's commentFormat see? Config: commentFormat=^ignored$ */
public class Probe6 {
    void f() {
        try { g(); } catch (Exception e) { //ignored
        }
        try { g(); } catch (Exception e) { // ignored
        }
        try { g(); } catch (Exception e) { /*ignored*/ }
        try { g(); } catch (Exception e) { /* ignored */ }
        try { g(); } catch (Exception e) { /** ignored */ }
        try {
            g();
        } catch (Exception e) {
            /*
             * ignored
             */
        }
        try {
            g();
        } catch (Exception e) {
            //ignored
            //second line
        }
        try { g(); } catch (Exception e) { //ignored
        }
        try { g(); } catch (Exception e) { /*IGNORED*/ }
        try { g(); } catch (Exception e) { /*ignored and more*/ }
    }

    void g() {}
    void h() {
        try { g(); } catch (Exception e) { //ignored  
        }
        try { g(); } catch (Exception e) { //ignored	
        }
        try { g(); } catch (Exception e) { /*ignored  */ }
    }
}
