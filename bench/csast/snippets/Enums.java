package demo;

enum Plain { A, B, C }

enum Trailing { A, B, ; }

enum OnlySemi { ; static int x; }

enum Empty {}

enum Full implements Runnable, java.io.Serializable {
    @Deprecated A(1) {
        @Override public void run() {}
    },
    B(2, "x"),
    C,
    D { int f() { return 1; } };

    private final int v;
    private final String s;

    Full() { this(0); }
    Full(int v) { this(v, null); }
    Full(int v, String s) { this.v = v; this.s = s; }

    public void run() {}

    static { System.out.println(); }
    { v2 = 1; }
    int v2;

    enum Inner { X }
}

public enum Op {
    PLUS("+") { double apply(double a, double b) { return a + b; } },
    MINUS("-") { double apply(double a, double b) { return a - b; } };
    Op(String s) {}
    abstract double apply(double a, double b);
}
