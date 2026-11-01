public class EbProbe {
    static { }
    static { /* c */ }
    static { ; }
    { }
    { // c
    }
    { int x; }
    void a(int x, boolean b) {
        while (b) { }
        while (b) { /* c */ }
        while (b) { ; }
        while (b) ;
        do { } while (b);
        do { /* c */ } while (b);
        for (;;) { }
        for (;;) { // c
        }
        for (int i : new int[0]) { }
        if (b) { }
        if (b) { /* c */ }
        if (b) { } else { }
        if (b) { x++; } else { }
        if (b) { x++; } else { // c
        }
        if (b) { } else if (!b) { }
        try { } finally { }
        try { x++; } finally { }
        try { x++; } finally { /* c */ }
        try { } catch (Exception e) { }
        try { x++; } catch (Exception e) { }
        try { x++; } catch (Exception e) { /* c */ }
        try (java.io.StringReader r = new java.io.StringReader("")) { }
        synchronized (this) { }
        synchronized (this) { /* c */ }
        switch (x) { }
        switch (x) { case 1: }
        switch (x) { case 1: { } }
        switch (x) { case 1: break; default: }
        switch (x) { case 1: { } break; }
        switch (x) { case 1: /* c */ default: }
        switch (x) { case 1 -> { } default -> { /* c */ } }
        switch (x) { case 1 -> { } }
        switch (x) { case 1 -> x++; }
        int[] arr = { };
        int[] arr2 = { /* c */ };
        int[][] arr3 = { { }, { 1 } };
        Runnable r = () -> { };
        Runnable r2 = () -> { /* c */ };
        { }
        { /* c */ }
        label: { }
        new Object() { };
        class L { }
        int y = switch (x) { case 1 -> { yield 1; } default -> { yield 2; } };
        if (b) {
        }
        if (b) {
            // comment
        }
        if (b) {

        }
        if (b) { /* a */ /* b */ }
        if (b) { // c1
            // c2
        }
    }
    void m() { }
    EbProbe() { }
    @interface A { String[] v() default { }; }
    @A(v = { }) int f;
    enum E { A { }, B; }
}
