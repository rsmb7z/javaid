package ok.pkg;

public class FtInf {
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
                break;
        }
        }
    }
    void m1(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { break; }
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
                do { break; } while (true);
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
                while (true) { foo(); break; }
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
                for (;;) { foo(); break; }
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
                do { foo(); break; } while (true);
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
                while (true) { break; foo(); }
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
                for (;;) { break; foo(); }
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
                do { break; foo(); } while (true);
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
                while (true) { if (b) break; }
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
                for (;;) { if (b) break; }
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
                do { if (b) break; } while (true);
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
                while (true) { if (b) { break; } }
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
                for (;;) { if (b) { break; } }
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
                do { if (b) { break; } } while (true);
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
                while (true) { if (b) break; foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m16(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) break; foo(); }
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
                do { if (b) break; foo(); } while (true);
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
                while (true) { if (b) { break; } foo(); }
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
                for (;;) { if (b) { break; } foo(); }
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
                do { if (b) { break; } foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m21(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); if (b) break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m22(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); if (b) break; }
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
                do { foo(); if (b) break; } while (true);
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
                while (true) { foo(); if (b) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m25(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); if (b) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m26(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); if (b) { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m27(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); } else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m28(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); } else { break; } }
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
                do { if (b) { foo(); } else { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m30(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { lbl: { break lbl; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m31(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { lbl: { break lbl; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m32(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { lbl: { break lbl; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m33(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { lbl: { break lbl; } foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m34(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { lbl: { break lbl; } foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m35(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { lbl: { break lbl; } foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m36(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m37(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m38(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m39(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { break; } foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m40(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { break; } foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m41(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { break; } foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m42(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { break; } finally { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m43(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { break; } finally { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m44(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { break; } finally { } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m45(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); try { break; } finally { } foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m46(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); try { break; } finally { } foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m47(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); try { break; } finally { } foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m48(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { synchronized (this) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m49(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { synchronized (this) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m50(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { synchronized (this) { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m51(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { switch (y) { case 1: break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m52(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { switch (y) { case 1: break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m53(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { switch (y) { case 1: break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m54(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m55(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m56(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m57(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) break; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m58(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) break; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m59(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) break; else foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m60(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m61(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m62(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m63(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m64(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m65(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m66(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) break; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m67(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) break; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m68(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) break; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m69(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { if (y > 1) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m70(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { if (y > 1) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m71(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { if (y > 1) break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m72(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { while (b) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m73(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { while (b) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m74(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { while (b) { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m75(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m76(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m77(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m78(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { foo(); } catch (RuntimeException e) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m79(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { foo(); } catch (RuntimeException e) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m80(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { foo(); } catch (RuntimeException e) { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m81(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); } else if (y > 1) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m82(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); } else if (y > 1) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m83(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); } else if (y > 1) { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m84(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m85(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m86(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { continue; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m87(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m88(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m89(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) continue; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m90(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m91(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m92(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); continue; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m93(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { { break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m94(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { { break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m95(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { { break; } } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m96(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); { { break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m97(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); { { break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m98(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); { { break; } } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m99(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { { break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m100(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { { break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m101(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { { break; } } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m102(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) ; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m103(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) ; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m104(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) ; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m105(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) foo(); else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m106(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) foo(); else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m107(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) foo(); else { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m108(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { { break; } } finally { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m109(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { { break; } } finally { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m110(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { { break; } } finally { } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m111(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { break; } catch (RuntimeException e) { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m112(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { break; } catch (RuntimeException e) { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m113(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { break; } catch (RuntimeException e) { } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m114(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { foo(); } finally { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m115(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { foo(); } finally { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m116(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { foo(); } finally { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m117(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { break; } catch (RuntimeException e) { foo(); } finally { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m118(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { break; } catch (RuntimeException e) { foo(); } finally { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m119(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { break; } catch (RuntimeException e) { foo(); } finally { foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m120(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { lbl: if (b) break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m121(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { lbl: if (b) break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m122(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { lbl: if (b) break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m123(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { lbl: lbl2: { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m124(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { lbl: lbl2: { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m125(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { lbl: lbl2: { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m126(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { lbl: { foo(); break lbl; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m127(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { lbl: { foo(); break lbl; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m128(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { lbl: { foo(); break lbl; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m129(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { synchronized (this) { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m130(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { synchronized (this) { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m131(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { synchronized (this) { foo(); break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m132(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { synchronized (this) { if (b) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m133(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { synchronized (this) { if (b) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m134(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { synchronized (this) { if (b) break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m135(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); break; } else { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m136(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); break; } else { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m137(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); break; } else { foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m138(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { break; } else if (y > 1) { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m139(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { break; } else if (y > 1) { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m140(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { break; } else if (y > 1) { foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m141(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); } else { if (y > 1) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m142(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); } else { if (y > 1) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m143(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); } else { if (y > 1) break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m144(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { switch (y) { default: break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m145(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { switch (y) { default: break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m146(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { switch (y) { default: break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m147(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { for (int i = 0; i < 1; i++) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m148(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { for (int i = 0; i < 1; i++) { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m149(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { for (int i = 0; i < 1; i++) { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m150(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { do { break; } while (b); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m151(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { do { break; } while (b); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m152(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { do { break; } while (b); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m153(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { break; break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m154(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { break; break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m155(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { break; break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m156(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); break; ; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m157(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); break; ; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m158(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); break; ; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m159(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { return; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m160(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { return; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m161(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { return; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m162(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { throw new RuntimeException(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m163(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { throw new RuntimeException(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m164(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { throw new RuntimeException(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m165(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m166(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m167(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m168(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { return; } else { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m169(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { return; } else { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m170(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { return; } else { foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m171(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m172(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m173(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m174(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) break; else return; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m175(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) break; else return; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m176(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) break; else return; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m177(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) throw new RuntimeException(); else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m178(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) throw new RuntimeException(); else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m179(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) throw new RuntimeException(); else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m180(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) continue; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m181(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) continue; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m182(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) continue; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m183(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) foo(); else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m184(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) foo(); else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m185(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) foo(); else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m186(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { return; } else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m187(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { return; } else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m188(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { return; } else { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m189(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { break; } else { return; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m190(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { break; } else { return; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m191(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { break; } else { return; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m192(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m193(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m194(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m195(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) break; else continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m196(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) break; else continue; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m197(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) break; else continue; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m198(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m199(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m200(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else { foo(); break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m201(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else if (y > 1) break; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m202(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else if (y > 1) break; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m203(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else if (y > 1) break; else foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m204(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); return; } else { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m205(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); return; } else { foo(); break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m206(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); return; } else { foo(); break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m207(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); } else { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m208(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); } else { foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m209(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); } else { foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m210(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); break; } else { return; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m211(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); break; } else { return; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m212(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); break; } else { return; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m213(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { foo(); if (b) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m214(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { foo(); if (b) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m215(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { foo(); if (b) return; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m216(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m217(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m218(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m219(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { return; } else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m220(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { return; } else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m221(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { return; } else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m222(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else if (y > 1) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m223(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else if (y > 1) return; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m224(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else if (y > 1) return; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m225(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) return; else if (y > 1) break; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m226(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) return; else if (y > 1) break; else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m227(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) return; else if (y > 1) break; else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m228(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) break; else if (y > 1) return; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m229(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) break; else if (y > 1) return; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m230(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) break; else if (y > 1) return; else foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m231(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) foo(); else if (y > 1) foo(); else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m232(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) foo(); else if (y > 1) foo(); else break; }
            case 2:
                foo();
                break;
        }
        }
    }
    void m233(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) foo(); else if (y > 1) foo(); else break; } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m234(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) foo(); else if (y > 1) break; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m235(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) foo(); else if (y > 1) break; else foo(); }
            case 2:
                foo();
                break;
        }
        }
    }
    void m236(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) foo(); else if (y > 1) break; else foo(); } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m237(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); } else if (y > 1) { foo(); } else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m238(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); } else if (y > 1) { foo(); } else { break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m239(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); } else if (y > 1) { foo(); } else { break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m240(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { if (y > 1) break; else foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m241(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { if (y > 1) break; else foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m242(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { if (y > 1) break; else foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m243(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { if (y > 1) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m244(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { if (y > 1) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m245(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { if (y > 1) foo(); else break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m246(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { if (y > 1) return; else foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m247(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { if (y > 1) return; else foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m248(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { if (y > 1) return; else foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m249(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { if (b) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m250(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { if (b) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m251(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { if (b) foo(); else break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m252(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { if (b) break; else foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m253(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { if (b) break; else foo(); } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m254(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { if (b) break; else foo(); } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m255(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { if (b) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m256(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { if (b) break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m257(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { if (b) break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m258(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { { { if (b) foo(); else break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m259(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { { { if (b) foo(); else break; } } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m260(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { { { if (b) foo(); else break; } } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m261(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { if (b) { foo(); } else { if (y > 1) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m262(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { if (b) { foo(); } else { if (y > 1) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m263(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { if (b) { foo(); } else { if (y > 1) foo(); else break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m264(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { try { if (b) foo(); else break; } finally { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m265(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { try { if (b) foo(); else break; } finally { } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m266(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { try { if (b) foo(); else break; } finally { } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
    void m267(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                while (true) { synchronized (this) { if (b) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m268(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                for (;;) { synchronized (this) { if (b) foo(); else break; } }
            case 2:
                foo();
                break;
        }
        }
    }
    void m269(int x, int y, Color c, boolean b) {
        loop: for (;;) {
        switch (x) {
            case 1:
                do { synchronized (this) { if (b) foo(); else break; } } while (true);
            case 2:
                foo();
                break;
        }
        }
    }
}
