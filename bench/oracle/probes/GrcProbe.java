package p;

import java.util.List;

public class GrcProbe {
    int f;

    GrcProbe() { }

    GrcProbe(int a) {
        f = a; }

    GrcProbe(String a) {
    }

    GrcProbe(long a) { f = 1; }

    GrcProbe(char a) {
        f = 1;
    } // comment

    void empty() { }
    void empty2() {}
    void empty3() {
    }
    void empty4() { /* c */ }
    void empty5() {  }
    void empty6() {
        // c
    }
    void empty7() {} // c
    void empty8() {} /* c */
    void empty9() { } int z;

    void nonEmpty() { f = 1; }

    void nonEmpty2() {
        f = 1; }

    void nonEmpty3() {
        f = 1;
    }

    void ifs(boolean b, boolean c) {
        if (b) { f = 1; }
        if (b) { f = 1; } else { f = 2; }
        if (b) {
            f = 1;
        } else {
            f = 2;
        }
        if (b) {
            f = 1;
        }
        else {
            f = 2;
        }
        if (b) {
            f = 1; } else {
            f = 2;
        }
        if (b) {
        } else {
        }
        if (b) { } else { }
        if (b) {} else {}
        if (b) {
        } else if (c) {
        } else {
        }
        if (b) {
            f = 1;
        } else if (c) {
            f = 2;
        }
        else if (c) {
            f = 3;
        } else {
            f = 4; }
        if (b) {
            f = 1;
        } /* c */ else {
            f = 2;
        }
        if (b) {
            f = 1;
        } // c
        else {
            f = 2;
        }
        if (b) {
            f = 1;
        }
        // c
        else {
            f = 2;
        }
        if (b) f = 1; else { f = 2; }
        if (b) { f = 1; } else f = 2;
        if (b) {
            f = 1;
        } else
        {
            f = 2;
        }
        if (b) { f = 1; } else if (c) { f = 2; } else { f = 3; }
    }

    void tries() throws Exception {
        try { f = 1; } catch (Exception e) { f = 2; } finally { f = 3; }
        try {
            f = 1;
        } catch (RuntimeException e) {
            f = 2;
        } catch (Exception e) {
        } finally {
        }
        try {
            f = 1;
        }
        catch (Exception e) {
            f = 2;
        }
        finally {
            f = 3;
        }
        try {
        } catch (Exception e) {}
        try (java.io.Reader r = null) {
            f = 1;
        } catch (Exception e) {
            f = 2; }
        try (java.io.Reader r = null) { }
        try (java.io.Reader r = null) {
            f = 1; }
        try (java.io.Reader r = null) {
            f = 1;
        }
        try {
            f = 1;
        } finally {
            f = 2; }
        try {
            f = 1;
        } catch (Exception e) {
            f = 2;
        } finally { f = 3; }
        try { } finally { }
        try {} finally {}
        try {
        } finally {
        }
    }

    void loops(boolean b, List<String> l) {
        for (int i = 0; i < 3; i++) { f = i; }
        for (String s : l) { f = 1; }
        for (String s : l) { }
        for (;;) {}
        for (;;) {
        }
        while (b) { f = 1; }
        while (b) {}
        while (b) {
        }
        while (b) {
            f = 1; }
        do { f = 1; } while (b);
        do {
            f = 1;
        } while (b);
        do {
            f = 1; } while (b);
        do { } while (b);
        do {} while (b);
        do {
        } while (b);
        do {
            f = 1;
        }
        while (b);
        do {
            f = 1;
        } /* c */ while (b);
        do f = 1; while (b);
        synchronized (this) { f = 1; }
        synchronized (this) { }
        synchronized (this) {
        }
        synchronized (this) {
            f = 1; }
        synchronized (this) {
            f = 1;
        }
    }

    void switches(int x, String s) {
        switch (x) { case 1: f = 1; break; default: f = 2; }
        switch (x) {
            case 1:
                f = 1;
                break;
            case 2: {
                f = 2;
                break; }
            case 3: {
                f = 3;
            }
            case 4: { f = 4; }
            case 5: {
            }
            case 6: {}
            case 7: { } { f = 7; }
            default: {
                f = 8;
            }
        }
        switch (x) {
        }
        switch (x) { }
        switch (x) {}
        switch (x) {
            case 1 -> { f = 1; }
            case 2 -> {
                f = 2;
            }
            case 3 -> {
                f = 3; }
            case 4 -> { }
            case 5 -> {}
            case 6 -> {
            }
            case 7 -> f = 7;
            case 8 -> throw new RuntimeException();
            default -> {
                f = 9;
            }
        }
        switch (x) {
            case 1 -> { f = 1; } default -> { f = 2; }
        }
        switch (x) {
            case 1, 2 -> {
                f = 1;
            } case 3 -> { }
            default -> { }
        }
        int y = switch (x) {
            case 1 -> { yield 1; }
            case 2 -> {
                yield 2;
            }
            default -> {
                yield 3; }
        };
        int z = switch (x) { case 1 -> 1; default -> 2; };
        int w = switch (x) {
            case 1: { yield 1; }
            default: { yield 2;
            }
        };
        switch (s) {
            case "a":
            case "b": {
                f = 1;
            }
            case "c":
                {
                    f = 2;
                }
            default:
                { f = 3; }
        }
        switch (x) {
            case 1: // c
                { f = 1; }
            default: /* c */ { f = 2; }
        }
    }

    void blocks() {
        { f = 1; }
        {
            f = 2; }
        { }
        {}
        {
        }
        label: { f = 3; }
    }

    static { f2 = 1; }
    static {
        f2 = 2; }
    static { }
    static {}
    static {
    }
    { f = 1; }
    {
        f = 1; }
    { }
    {}
    {
    }
    static int f2;

    Runnable r = () -> { f = 1; };
    Runnable r2 = () -> {
        f = 1; };
    Runnable r3 = () -> { };
    Runnable r4 = () -> {};
    Runnable r5 = () -> {
    };
    Object o = new Object() { int q; };
    Object o2 = new Object() {
        int q; };
    Object o3 = new Object() { };
    Object o4 = new Object() {};
    Object o5 = new Object() {
    };
    int[] a1 = { 1, 2 };
    int[] a2 = {
        1, 2 };
    int[] a3 = { };
    int[] a4 = {};
    int[] a5 = {
    };
    String s = foo(() -> { f = 1; });
    String s2 = foo(() -> {
        f = 1;
    });
    String s3 = foo(() -> {
        f = 1;
    }, 2);
    String s4 = foo(() -> { f = 1; }, 2);
    String s5 = foo(x -> {
        f = 1; });
    String s6 = foo(new Object() {
        int q;
    });
    String s7 = foo(new Object() {
        int q; });
    String s8 = foo(new Object() {
        int q;
    }, 2);
    String s9 = foo(1, () -> {
        f = 1;
    }
    );
    int v = foo(
        () -> {
            f = 1;
        })
        .bar();
    int v2 = foo(() -> {
        f = 1;
    }).bar();
    int v3 = foo(() -> {
        f = 1;
    })
        .bar();
    int v4 = foo(() -> { f = 1; }).bar();
    int v5 = foo(() -> {
        f = 1;
    } /* c */);
    int v6 = foo(() -> {
        f = 1;
    } // c
    );
    int v7 = foo(() -> {
    } + 1);

    enum E1 { A, B }
    enum E2 {
        A, B }
    enum E3 {
        A,
        B
    }
    enum E4 { }
    enum E5 {}
    enum E6 {
    }
    enum E7 {
        A { void m() { } },
        B {
            void m() {
            }
        },
        C {
            void m() { f2 = 1; }
        },
        D { },
        E {};
    }
    enum E8 { A { }, B { } }
    enum E9 {
        A {
        },
        B {
        }
    }
    enum E10 {
        A { int q; }, B {
            int q;
        };
        void m() { }
    }
    enum E11 {
        A,
        B;
    }
    enum E12 {
        A,
        B; void m() { } }

    class C1 { }
    class C2 {}
    class C3 {
    }
    class C4 { int q; }
    class C5 {
        int q; }
    class C6 {
        class Inner { }
    }
    class C7 { class Inner { } }
    class C8 {
        int q;
        class Inner {
            int r; } }
    interface I1 { }
    interface I2 {}
    interface I3 {
    }
    interface I4 { void m(); }
    interface I5 {
        void m(); }
    @interface A1 { }
    @interface A2 {}
    @interface A3 {
    }
    @interface A4 { int v(); }
    @interface A5 {
        int v(); }
    record R1() { }
    record R2() {}
    record R3() {
    }
    record R4(int a) { void m() { } }
    record R5(int a) {
        void m() { } }
    record R6(int a) {
        R6 { f2 = a; }
    }
    record R7(int a) {
        R7 {
            f2 = a; }
    }
    record R8(int a) {
        R8 { }
    }
    record R9(int a) {
        R9 {}
    }
    record R10(int a) {
        R10 {
        }
    }
    record R11(int a) {
        R11 {
            f2 = a;
        }
    }

    void lambdas(List<String> l) {
        l.forEach(x -> { f = 1; });
        l.forEach(x -> {
            f = 1; });
        l.forEach(x -> { });
        l.forEach(x -> {
        });
        l.forEach(x -> {
            f = 1;
        });
        Runnable r = () -> { if (f > 0) { f = 1; } };
        Runnable r2 = () -> {
            if (f > 0) { f = 1; } };
        Runnable r3 = () -> { while (true) { } };
        foo(a -> { }, b -> { });
        foo(a -> {
        }, b -> {
        });
        foo(a -> {
            f = 1;
        }, b -> {
            f = 2;
        });
        foo(a -> { f = 1; }, b -> { f = 2; });
        foo(a -> { f = 1; },
            b -> { f = 2; });
        var anon = new Object() { void m() { } };
        var anon2 = new Object() {
            void m() {
            }
        };
    }

    void tabs() {
		if (f > 0) {
			f = 1; }
		if (f > 0) { f = 1;
		}
		if (f > 0) {
			f = 1;
		} else {
			f = 2; }
		try { f = 1;
		} catch (Exception e) { }
    }

    void unicode() {
        String é = "é€"; if (f > 0) { f = 1; }
        String ü = "é€"; if (f > 0) {
            f = 1; }
    }

    void ifComments(boolean b) {
        if (b) {
            f = 1;
        } /* c */
        else {
            f = 2;
        }
        if (b) {
            f = 1;
        } /* c */ else {
            f = 2;
        } /* c */
        if (b) {
            f = 1; // c
        } else {
            // c
        }
        if (b) {
            // c
        } else {
            // c
        }
        if (b) { // c
        } else { // c
        }
        if (b) { /* c */ } else { /* c */ }
        if (b) { /* c */ } /* d */ else { /* c */ }
        if (b) {
            f = 1; } /* d */ else {
            f = 2; }
    }

    void expressionStmts() {
        Object a = new Object() { public String toString() { return "a"; } };
        Object b = new Object() {
            public String toString() {
                return "a";
            }
        };
        foo(new Object() {
            public String toString() {
                return "a"; }
        });
        foo(new Object() {
        }, new Object() {
        });
        foo(new Object() { }, new Object() { });
        foo(new Object() {
            int q;
        }, 1);
        foo(1, new Object() {
            int q;
        });
        Object c = (Runnable) () -> {
            f = 1;
        };
        Object d = (Runnable) () -> { f = 1; };
        Object e = true ? new Object() { } : new Object() {
            int q;
        };
        Object g = true ? new Object() {
            int q;
        } : null;
        f = switch (f) {
            default -> 1;
        } + 1;
        f = switch (f) { default -> 1; } + 1;
        int[][] arr = {
            {1, 2},
            {3, 4}
        };
        int[][] arr2 = {
            {1, 2}, {3, 4}};
        int[][] arr3 = { {1, 2}, {3, 4} };
    }

    void after1() {} int za;
    void after2() {} void after3() {}
    class AfterC {} int zb;
    interface AfterI {} class AfterD {}
    enum AfterE {} int zc;
    record AfterR() {} int zd;
    @interface AfterA {} int ze;
    static {} int zf;
    {} int zg;
    AfterK() {} int zh;
    void after4() {
        if (f > 0) {} f = 1;
        while (f > 0) {} f = 1;
        for (;;) {} f = 1;
        try {} finally {} f = 1;
        try {} catch (Exception e) {} f = 1;
        synchronized (this) {} f = 1;
        switch (f) {} f = 1;
        switch (f) { default -> {} } f = 1;
        switch (f) { default: {} f = 1; }
        do {} while (f > 0);
        do {} while (f > 0); f = 1;
        { } f = 1;
        {} f = 1;
        label: {} f = 1;
        Runnable r = () -> {}; f = 1;
        Runnable q = () -> {}
            ; f = 1;
        if (f > 0) {} else {} f = 1;
        if (f > 0) {
            f = 1;
        } else {} f = 1;
        if (f > 0) { f = 1; } f = 2;
        while (f > 0) { f = 1; } f = 2;
        for (;;) { f = 1; } f = 2;
        try { f = 1; } finally { f = 2; } f = 3;
        {
            f = 1;
        } f = 2;
        {
            f = 1; } f = 2;
        foo(x -> {}, y -> {}); f = 3;
        foo(new Object() {}, y -> {}); f = 3;
        switch (f) {
            case 1: {} f = 1;
            case 2: { f = 2; } f = 3;
        }
        switch (f) {
            case 3 -> {} 
            default -> { f = 1; } 
        }
    }
    int zi = foo(() -> {}) + 1;
    int zj = switch (f) { default -> { yield 1; } } + 1;
}
