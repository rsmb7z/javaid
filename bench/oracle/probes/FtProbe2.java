package ok.pkg;

public class FtProbe2 {
    enum Color { RED, GREEN }
    void foo() {}
    void bar() {}
    int val() { return 0; }
    void m0(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                { foo(); break; }
            case 2:
                foo();
        }
        }
    }
    void m1(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) { return; } else { break; }
            case 2:
                foo();
        }
        }
    }
    void m2(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) { return; }
            case 2:
                foo();
        }
        }
    }
    void m3(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) return; else break;
            case 2:
                foo();
        }
        }
    }
    void m4(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) { foo(); } else if (y > 1) { return; } else { return; }
            case 2:
                foo();
        }
        }
    }
    void m5(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) { foo(); } else if (y > 1) { return; }
            case 2:
                foo();
        }
        }
    }
    void m6(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { return; } finally { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m7(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { foo(); } finally { return; }
            case 2:
                foo();
        }
        }
    }
    void m8(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { return; } catch (RuntimeException e) { return; }
            case 2:
                foo();
        }
        }
    }
    void m9(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { return; } catch (RuntimeException e) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m10(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { foo(); } catch (RuntimeException e) { return; }
            case 2:
                foo();
        }
        }
    }
    void m11(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { return; } catch (RuntimeException e) { return; } finally { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m12(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                synchronized (this) { return; }
            case 2:
                foo();
        }
        }
    }
    void m13(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                synchronized (this) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m14(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: return; default: return; }
            case 2:
                foo();
        }
        }
    }
    void m15(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: return; default: foo(); }
            case 2:
                foo();
        }
        }
    }
    void m16(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: return; }
            case 2:
                foo();
        }
        }
    }
    void m17(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: break; default: return; }
            case 2:
                foo();
        }
        }
    }
    void m18(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { return; }
            case 2:
                foo();
        }
        }
    }
    void m19(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (b) { return; }
            case 2:
                foo();
        }
        }
    }
    void m20(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { return; }
            case 2:
                foo();
        }
        }
    }
    void m21(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { return; } while (b);
            case 2:
                foo();
        }
        }
    }
    void m22(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); } while (true);
            case 2:
                foo();
        }
        }
    }
    void m23(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (int i = 0; i < 1; i++) { return; }
            case 2:
                foo();
        }
        }
    }
    void m24(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (int i : new int[0]) { return; }
            case 2:
                foo();
        }
        }
    }
    void m25(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                lbl: { break lbl; }
            case 2:
                foo();
        }
        }
    }
    void m26(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                lbl: return;
            case 2:
                foo();
        }
        }
    }
    void m27(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                System.exit(0);
            case 2:
                foo();
        }
        }
    }
    void m28(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                assert b;
            case 2:
                foo();
        }
        }
    }
    void m29(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                ;
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
                ;
            case 2:
                foo();
        }
        }
    }
    void m31(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                return;
                foo();
            case 2:
                foo();
        }
        }
    }
    void m32(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                break;
                // fall through
            case 2:
                foo();
        }
        }
    }
    void m33(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                break;
                foo();
            case 2:
                foo();
        }
        }
    }
    void m34(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                int a = 1;
            case 2:
                foo();
        }
        }
    }
    void m35(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                class L {}
            case 2:
                foo();
        }
        }
    }
    void m36(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                Runnable r = () -> { return; };
            case 2:
                foo();
        }
        }
    }
    void m37(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                return; // x
            case 2:
                foo();
        }
        }
    }
}
