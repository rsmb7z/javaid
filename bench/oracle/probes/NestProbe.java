public class NestProbe {
    static { { int s = 1; } }
    { { } }
    NestProbe() { { } }
    void a(int x) {
        { int y = 1; }
        {
            {
                int z = 2;
            }
        }
        {}
        if (x > 0) { { } }
        if (x > 0) { } else { { } }
        for (;;) { { break; } }
        while (x > 0) { x--; }
        do { { } } while (x > 0);
        try { { } } catch (Exception e) { { } } finally { { } }
        synchronized (this) { { } }
        label: { x++; }
        label2: { { x++; } }
        switch (x) {
            case 1: { x++; break; }
            case 2: x++; { x--; } break;
            case 3: { x++; } { x--; }
            case 4:
            default: { break; }
        }
        switch (x) {
            case 1 -> { x++; }
            case 2 -> { { x++; } }
            default -> { }
        }
        Runnable r = () -> { { } };
        Runnable r2 = () -> { };
        int k = switch (x) { case 1: { yield 1; } default: yield 2; };
        int m = switch (x) { case 1 -> { yield 1; } default -> { { yield 2; } } };
        new Object() { void q() { { } } };
        class Local { void l() { { } } }
        { class L2 { } }
    }
    void b(int x) {
        switch (x) {
            case 1: { { x++; } break; }
            case 2: { }
        }
    }
}
