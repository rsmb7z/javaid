package ok.pkg;

public class FtProbe3 {
    enum Color { RED, GREEN }
    void foo() {}
    void bar() {}
    int val() { return 0; }
    void m0(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (b) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m1(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m2(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m3(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (int i = 0; i < 1; i++) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m4(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (int i : new int[0]) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m5(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); } while (b);
            case 2:
                foo();
        }
        }
    }
    void m6(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); } while (true);
            case 2:
                foo();
        }
        }
    }
    void m7(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (b) { break; }
            case 2:
                foo();
        }
        }
    }
    void m8(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (b) { continue; }
            case 2:
                foo();
        }
        }
    }
    void m9(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (b) foo();
            case 2:
                foo();
        }
        }
    }
    void m10(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (b) return;
            case 2:
                foo();
        }
        }
    }
    void m11(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                lbl: foo();
            case 2:
                foo();
        }
        }
    }
    void m12(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                lbl: { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m13(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                lbl: while (b) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m14(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: return; case 2: foo(); }
            case 2:
                foo();
        }
        }
    }
    void m15(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: foo(); }
            case 2:
                foo();
        }
        }
    }
    void m16(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { default: return; }
            case 2:
                foo();
        }
        }
    }
    void m17(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { default: foo(); }
            case 2:
                foo();
        }
        }
    }
    void m18(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { }
            case 2:
                foo();
        }
        }
    }
    void m19(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: continue; default: continue; }
            case 2:
                foo();
        }
        }
    }
    void m20(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: continue; default: break; }
            case 2:
                foo();
        }
        }
    }
    void m21(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: return; case 2: return; }
            case 2:
                foo();
        }
        }
    }
    void m22(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: case 2: return; }
            case 2:
                foo();
        }
        }
    }
    void m23(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1: return; case 2: case 3: }
            case 2:
                foo();
        }
        }
    }
    void m24(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1 -> { return; } default -> { return; } }
            case 2:
                foo();
        }
        }
    }
    void m25(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                switch (y) { case 1 -> { return; } default -> { foo(); } }
            case 2:
                foo();
        }
        }
    }
    void m26(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) break; else if (y > 1) break; else foo();
            case 2:
                foo();
        }
        }
    }
    void m27(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) { return; } else { continue; }
            case 2:
                foo();
        }
        }
    }
    void m28(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                if (b) { throw new RuntimeException(); } else { return; }
            case 2:
                foo();
        }
        }
    }
    void m29(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { return; } catch (RuntimeException e) { break; }
            case 2:
                foo();
        }
        }
    }
    void m30(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try (java.io.StringReader r = null) { return; }
            case 2:
                foo();
        }
        }
    }
    void m31(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try (java.io.StringReader r = null) { foo(); }
            case 2:
                foo();
        }
        }
    }
    void m32(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { foo(); } catch (RuntimeException e) { return; } finally { return; }
            case 2:
                foo();
        }
        }
    }
    void m33(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { foo(); } catch (RuntimeException e) { foo(); } finally { return; }
            case 2:
                foo();
        }
        }
    }
    void m34(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                try { return; } finally { }
            case 2:
                foo();
        }
        }
    }
    void m35(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                { { return; } }
            case 2:
                foo();
        }
        }
    }
    void m36(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                { return; foo(); }
            case 2:
                foo();
        }
        }
    }
    void m37(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                {}
            case 2:
                foo();
        }
        }
    }
    void m38(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                foo();
                {}
            case 2:
                foo();
        }
        }
    }
}
