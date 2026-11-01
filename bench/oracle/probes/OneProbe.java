package ok.pkg;

public class OneProbe {
    int fa; int fb;
    int fc = 1; int fd = 2; int fe = 3;
    static { foo(); bar(); }
    { foo(); bar(); baz(); }
    OneProbe() { foo(); bar(); }
    void f(int a, int b) {
        int c = 1; int d = 2;
        a = 1; b = 2;
        foo(); bar();
        foo(); // c
        foo(); /* c */ bar();
        if (a == 1) { foo(); } else { bar(); }
        if (a == 1) foo(); else bar();
        if (a == 1) foo();
        if (a == 1) { foo(); bar(); }
        for (int i = 0; i < 1; i++) foo();
        for (int i = 0; i < 1; i++) { foo(); bar(); }
        for (int i = 0, j = 0; i < 1; i++, j++) {}
        for (;;) { break; }
        for (int i : new int[0]) { foo(); }
        for (int i : new int[0]) { foo(); bar(); }
        while (a < 1) { a++; b++; }
        while (a < 1) a++;
        do { a++; } while (a < 1);
        do { a++; b++; } while (a < 1);
        switch (a) { case 1: foo(); break; default: bar(); }
        switch (a) { case 1: foo(); bar(); break; }
        switch (a) { case 1 -> foo(); default -> bar(); }
        switch (a) { case 1 -> { foo(); bar(); } default -> bar(); }
        try { foo(); } catch (Exception e) { bar(); } finally { baz(); }
        try { foo(); bar(); } catch (Exception e) { }
        try (var r = open(); var r2 = open()) { foo(); }
        try (var r = open()) { foo(); bar(); }
        try (var r = open(); var r2 = open();) { }
        Runnable r = () -> { foo(); bar(); };
        Runnable r2 = () -> foo();
        Runnable r3 = () -> { foo(); }; bar();
        new Thread(() -> { foo(); }).start(); bar();
        synchronized (this) { foo(); bar(); }
        { foo(); bar(); }
        label: foo(); bar();
        int e1 = 0; { foo(); }
        assert a == 1; assert b == 2;
        throw new RuntimeException(); 
    }
    int g() { return 1; } int h() { return 2; }
    void k() { foo(); } void l() { bar(); }
    class In { void m() { foo(); bar(); } }
    Object anon = new Object() { void m() { foo(); bar(); } };
    void n() { int x = 1; int y = 2; foo(); }
    void o() { if (true) { foo(); }
        bar(); }
    void p() {
        foo();
        bar();
    }
    void q() { ; ; }
    void r() { foo();; }
    void s() { yield1(); }
    void foo() {} void bar() {} void baz() {}
    java.io.StringReader open() { return null; }
}
