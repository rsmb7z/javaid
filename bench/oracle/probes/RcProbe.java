import java.util.*;
public class RcProbe {
    static { int s = 1; }
    static { int s = 1;
    }
    static { int s = 1; }  static int t;
    { int i = 1; }
    { int i = 1;
        }
    RcProbe() { }
    RcProbe(int a) { a++; }
    RcProbe(int a, int b) {
        a++; }
    RcProbe(int a, int b, int c) {
        a++;
    } int z;
    void a(int x, boolean b) {
        if (b) { x++; }
        if (b) { x++; } else { x--; }
        if (b) { x++; }
        else { x--; }
        if (b) {
            x++;
        } else {
            x--;
        }
        if (b) {
            x++;
        }
        else {
            x--;
        }
        if (b) {
            x++; }
        else if (x > 1) { x--; }
        else { x = 0; }
        if (b) { x++; } else if (x > 1) { x--; } else { x = 0; }
        if (b) {
            x++;
        } else if (x > 1) {
            x--;
        } else {
            x = 0; }
        if (b) { x++;
        }
        if (b) { x++; } x--;
        if (b) {
            x++;
        } x--;
        while (b) { x++; }
        while (b) {
            x++; }
        for (;;) { x++; }
        for (int i : new int[0]) {
            x++; }
        do { x++; } while (b);
        do {
            x++;
        } while (b);
        do {
            x++; } while (b);
        do {
            x++;
        }
        while (b);
        try { x++; } catch (Exception e) { x--; } finally { x = 0; }
        try {
            x++;
        } catch (RuntimeException e) {
            x--;
        } catch (Exception e) {
            x--;
        } finally {
            x = 0;
        }
        try {
            x++;
        }
        catch (Exception e) {
            x--;
        }
        finally {
            x = 0;
        }
        try { x++; } finally { x = 0; } x++;
        try { x++; } catch (Exception e) { x--; } x++;
        try (java.io.StringReader r = new java.io.StringReader("")) { x++; }
        try (java.io.StringReader r = new java.io.StringReader("")) {
            x++; }
        try (java.io.StringReader r = new java.io.StringReader("")) {
            x++;
        } catch (Exception e) { }
        switch (x) { case 1: x++; break; default: x--; }
        switch (x) {
            case 1:
                x++;
                break;
            default:
                x--; }
        switch (x) {
            case 1: { x++; }
            case 2: {
                x--; }
            default:
                break;
        }
        switch (x) { case 1 -> { x++; } default -> { x--; } }
        switch (x) {
            case 1 -> {
                x++; }
            default -> {
                x--;
            }
        }
        int y = switch (x) { case 1 -> { yield 1; } default -> { yield 2; } };
        int y2 = switch (x) {
            case 1 -> { yield 1; }
            default -> {
                yield 2; }
        };
        Runnable r = () -> { x2(); };
        Runnable r2 = () -> {
            x2(); };
        Object o = new Object() { int q; };
        Object o2 = new Object() {
            int q; };
        Object o3 = new Object() {
            int q;
        };
        List<Integer> l = new ArrayList<Integer>() {{ add(1); }};
        List<Integer> l2 = new ArrayList<Integer>() {{
            add(1);
        }};
        List<Integer> l3 = new ArrayList<Integer>() {{
            add(1);
        }
        };
        { x++; }
        { x++;
        }
        int[] arr = { 1, 2 };
        int[] arr2 = {
            1, 2 };
    }
    void x2() { }
    void m1() { int a; }
    void m2() { int a;
    }
    void m3() {
        int a; }
    void m4() {
        int a;
    } void m5() { }
    class In { int a; }
    class In2 {
        int a; }
    class In3 { int a; } class In4 { }
    enum E { A, B; }
    enum E2 { A, B;
    }
    enum E3 {
        A, B; }
    interface I { void f(); }
    interface I2 {
        void f(); }
    @interface Ann { int v(); }
    record R(int a) { }
    record R2(int a) { R2 { a++; } }
    record R3(int a) {
        R3 {
            a++; }
    }
    void n() { if (true) { return; } }
    int p(int x) { if (x > 0) { return 1; } else { return 2; } }
    int q(int x) {
        if (x > 0) { return 1; }
        return 2;
    }
}
