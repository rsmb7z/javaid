package ok.pkg;

public class OneProbe2 {
    void c(boolean x, int i, int[] xs) {
        foo(); if (x) {}
        foo(); if (x) bar();
        foo(); while (x) {}
        foo(); for (;;) {}
        foo(); for (int k : xs) {}
        foo(); do {} while (x);
        foo(); switch (1) {}
        foo(); try {} finally {}
        foo(); synchronized (this) {}
        foo(); return;
        foo(); throw new RuntimeException();
        foo(); label: for (;;) {}
        foo(); assert x;
        foo(); int v;
        foo(); ;
        ; foo();
        foo(); class Loc {}
        foo(); interface LI {}
        foo(); break;
        foo(); continue;
        foo(); var q = 1;
        foo(); final int w = 1;
        foo(); x = !x;
        foo(); new Object();
        foo(); i++;
        foo(); --i;
        foo(); int[] arr = {1, 2};
        foo(); String s = "a;b;c";
        foo(); // ; ;
        foo(); /* ; */ bar();
        foo(); { }
        foo(); ; bar();
    }
    void d(int[] xs) {
        for (int i = 0; i < 1; i++) { foo(); } foo();
        for (int i = 0;
             i < 1; i++) { foo(); }
        for (int i = 0; i < 1; i++)
            foo(); bar();
        foo(); for (int i = 0; i < 1; i++) {}
        int a = 1; for (int i = 0, j = 0; i < 1; i++) {}
        for (int i = 0; i < 1; i++) {} foo();
        if (true) { foo(); } foo();
        if (true) {} else {} foo();
        while (true) { break; } foo();
        for (int k : xs) { foo(); } bar();
        foo(); for (int k : xs) foo();
    }
    void e() { try (java.io.StringReader a = null; java.io.StringReader b = null; java.io.StringReader c = null) { foo(); } }
    void e2() {
        try (java.io.StringReader a = null;
             java.io.StringReader b = null) { foo(); }
    }
    void e3(java.io.StringReader r, java.io.StringReader r2) { try (r; r2) { foo(); } }
    void e4() { foo(); try (java.io.StringReader a = null) { bar(); } }
    void e5() { try (java.io.StringReader a = null) { bar(); } foo(); }
    void e6() { try (java.io.StringReader a = null; java.io.StringReader b = null) { bar(); bar(); } }
    void e7() { foo(); try (java.io.StringReader a = null; java.io.StringReader b = null) { bar(); } foo(); }
    void foo() {}
    void bar() {}
}
