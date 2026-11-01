public class RcSwitchProbe {
    void a(int x) {
        Runnable r1 = () -> {
            switch (x) {
                default:
                    x++; }
        };
        foo(() -> {
            switch (x) {
                default:
                    x++; }
        });
        Runnable r3;
        r3 = () -> {
            switch (x) {
                default:
                    x++; }
        };
        Object o = new Object() {
            void m() {
                switch (x) {
                    default:
                        x++; }
            }
        };
        foo(new Runnable() {
            public void run() {
                switch (x) {
                    default:
                        x++; }
            }
        });
        Supplier<Integer> s = () -> {
            switch (x) {
                default:
                    return 1; }
        };
        int y = bar(() -> { switch (x) { default: x++; }
            return 1; });
        Runnable r4 = x > 0 ? () -> {
            switch (x) {
                default:
                    x++; }
        } : null;
        switch (x) {
            default:
                x++; }
        if (x > 0) { switch (x) {
            default:
                x++; } }
    }
    Runnable f = () -> {
        switch (1) {
            default:
                f(); }
    };
    static Object g = new Object() { void m() { switch (1) {
        default:
            g(); } } };
}
