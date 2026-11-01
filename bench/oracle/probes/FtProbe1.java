package ok.pkg;

public class FtProbe1 {
    enum Color { RED, GREEN }
    void foo() {}
    void bar() {}
    int val() { return 0; }
    void m0(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
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
                break;
            case 2:
                foo();
                return;
            case 3:
                foo();
                throw new RuntimeException();
            case 4:
                foo();
                continue;
            case 5:
                foo();
                continue loop;
            case 6:
                foo();
                break loop;
            default:
                foo();
        }
        }
    }
    void m2(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
            case 2:
                foo();
                break;
            case 3:
                // just a comment
            case 4:
                foo();
        }
        }
    }
    void m3(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                // fall through
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
                foo(); // falls through
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

                // fall through

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
                /* fall through */
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
                foo();
                /* fall through */ case 2:
                foo();
                break;
        }
        }
    }
    void m8(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo(); /* fallthrough */
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
                // $FALL-THROUGH$
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
                // Fall Through
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
                foo();
                // FALL THROUGH
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
                // falls-thru
            case 2:
                foo();
                // fallthru
            case 3:
                foo();
                // fall-thru
            case 4:
                foo();
                // falls thru
            case 5:
                foo();
                // fall thru
            case 6:
                foo();
                // falls through
            case 7:
                foo();
                // fallsthrough
            case 8:
                foo();
                // fallsthru
            case 9:
                foo();
                break;
        }
        }
    }
}
