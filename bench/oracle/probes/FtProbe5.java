package ok.pkg;

public class FtProbe5 {
    enum Color { RED, GREEN }
    void foo() {}
    void bar() {}
    int val() { return 0; }
    void m0(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /* fall through
                   and more */
            case 2:
                foo();
                break;
        }
        }
    }
    void m1(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /*
                   fall through */
            case 2:
                foo();
                break;
        }
        }
    }
    void m2(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /*
                 * fall through
                 * and so on
                 */
            case 2:
                foo();
                break;
        }
        }
    }
    void m3(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /*
                 * fall through
                 */
                // another
            case 2:
                foo();
                break;
        }
        }
    }
    void m4(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // a
                // fall through
                // b
            case 2:
                foo();
                break;
        }
        }
    }
    void m5(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                fallthrough();
            case 2:
                foo();
                break;
        }
        }
    }
    void m6(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                String s = "falls through";
            case 2:
                foo();
                break;
        }
        }
    }
    void m7(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo(); /* falls through */ foo();
            case 2:
                foo();
                break;
        }
        }
    }
    void m8(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // fall through
                ;
            case 2:
                foo();
                break;
        }
        }
    }
    void m9(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // fall through

                // (blank line above)
            case 2:
                foo();
                break;
        }
        }
    }
    void m10(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /* fall through */

            case 2:
                foo();
                break;
        }
        }
    }
    void m11(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) break; foo(); }
            case 2:
                foo();
        }
        }
    }
    void m12(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { for (;;) { break; } }
            case 2:
                foo();
        }
        }
    }
    void m13(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { switch (y) { case 1: break; } }
            case 2:
                foo();
        }
        }
    }
    void m14(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { lbl2: { break lbl2; } }
            case 2:
                foo();
        }
        }
    }
    void m15(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                lbl3: while (true) { while (b) { break lbl3; } }
            case 2:
                foo();
        }
        }
    }
    void m16(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { continue; }
            case 2:
                foo();
        }
        }
    }
    void m17(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { while (b) { break; } }
            case 2:
                foo();
        }
        }
    }
    void m18(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { break; } }
            case 2:
                foo();
        }
        }
    }
    void m19(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) break; } while (true);
            case 2:
                foo();
        }
        }
    }
    void m20(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { Runnable r = () -> { }; }
            case 2:
                foo();
        }
        }
    }
    void m21(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                int a = switch (y) { case 1: yield 1; default: yield 2; };
            case 2:
                foo();
        }
        }
    }
}
