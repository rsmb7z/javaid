package ok.pkg;

public class FtProbe4 {
    enum Color { RED, GREEN }
    void foo() {}
    void bar() {}
    int val() { return 0; }
    void m0(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { break; }
            case 2:
                foo();
        }
        }
    }
    void m1(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) foo();
            case 2:
                foo();
        }
        }
    }
    void m2(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (int i = 0; ; i++) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m3(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;true;) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m4(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (1 < 2) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m5(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while ((true)) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m6(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (!false) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m7(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); } while ((true));
            case 2:
                foo();
        }
        }
    }
    void m8(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); } while (!true);
            case 2:
                foo();
        }
        }
    }
    void m9(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) foo();
            case 2:
                foo();
        }
        }
    }
    void m10(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
            case 2:
                foo();
                // fall through
            case 3:
                foo();
                break;
        }
        }
    }
    void m11(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // fall through
                foo();
            case 2:
                foo();
                break;
        }
        }
    }
    void m12(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // fall through
                // another comment
            case 2:
                foo();
                break;
        }
        }
    }
    void m13(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // another comment
                // fall through
            case 2:
                foo();
                break;
        }
        }
    }
    void m14(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                // fall through
                foo();
            case 2:
                foo();
                break;
        }
        }
    }
    void m15(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
            case 2: // fall through
                foo();
                break;
        }
        }
    }
    void m16(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /*
                 * fall through
                 */
            case 2:
                foo();
                break;
        }
        }
    }
    void m17(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /** falls through */
            case 2:
                foo();
                break;
        }
        }
    }
    void m18(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // this falls through to the next
            case 2:
                foo();
                break;
        }
        }
    }
    void m19(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // nothing to see
                /* falls thru */
            case 2:
                foo();
                break;
        }
        }
    }
    void m20(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                /* falls thru */ // other
            case 2:
                foo();
                break;
        }
        }
    }
    void m21(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1: {
                foo();
                // fall through
            }
            case 2:
                foo();
                break;
        }
        }
    }
    void m22(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1: {
                foo();
            }
            // fall through
            case 2:
                foo();
                break;
        }
        }
    }
    void m23(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) {
                    foo();
                    // fall through
                }
            case 2:
                foo();
                break;
        }
        }
    }
    void m24(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // fall through
            default:
                foo();
        }
        }
    }
    void m25(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
            default:
                foo();
        }
        }
    }
    void m26(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
            case 2: case 3:
                foo();
                break;
        }
        }
    }
    void m27(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
            case 2:
            case 3:
                foo();
                break;
        }
        }
    }
    void m28(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                break;
            default:
                foo();
            case 2:
                foo();
                break;
        }
        }
    }
    void m29(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                break;
            case 2:
                foo();
        }
        }
    }
    void m30(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                break;
            default:
                foo();
        }
        }
    }
    void m31(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                break;
            case 2:
                foo();
                // fall through
        }
        }
    }
    void m32(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                break;
            case 2:
                foo();
                return;
        }
        }
    }
}
