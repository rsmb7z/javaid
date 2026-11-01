import java.lang.annotation.*;
public class MagicProbe {
    static final int A = 5;
    static final int B = 5 + 6;
    static final int C = -7;
    static final int[] D = { 3, 4 };
    static final int[] E = new int[8];
    static final long F = 1000L * 60 * 60;
    static final double G = 3.14;
    static final float H = 2.5f;
    static final int I = (int) 9.9;
    static final int J = Integer.parseInt("11") + 12;
    static final int K = true ? 13 : 14;
    static final int L = 15 << 2;
    static final String M = String.valueOf(16);
    static final Object N = new Object[] { 17 };
    static final int O = foo(18);
    final int P = 19;
    static int Q = 20;
    private final static int R = 21, S = 22;
    int T = 23;
    int[] U = { 24, 25 };
    int V = 0, W = 1, X = 2, Y = 3, Z = -1, AA = -2;
    long BB = 0L, CC = 1L, DD = 2L, EE = 3L;
    double FF = 0.0, GG = 1.0, HH = 2.0, II = 3.0;
    float JJ = 1f, KK = 2f, LL = 0.5f;
    int hex = 0x10, oct = 010, bin = 0b101, big = 1_000;
    char ch = 65;
    interface Consts { int CONST = 26; int[] ARR = { 27 }; }
    enum Level { LOW(28), HIGH(29); Level(int x) { } }
    @interface Ann { int v() default 30; int w() default 0; }
    @Retention(RetentionPolicy.RUNTIME) @interface Ann2 { int[] value(); }
    @Ann2({ 31, 32 }) int annotated;
    @Ann(v = 33) int annotated2;
    @SuppressWarnings("x") int annotated3 = 34;
    int foo(int x) { return x; }
    @Override public int hashCode() {
        int h = 17;
        h = 31 * h + 37;
        return 41 * h;
    }
    int hashCode2() { return 43; }
    void m(int x) {
        int a = 44;
        a = 45;
        a += 46;
        foo(47);
        foo(-48);
        foo(-1);
        foo(2);
        foo(3);
        if (x > 49) { }
        for (int i = 0; i < 50; i++) { }
        while (x != 51) { x--; }
        switch (x) { case 52: break; case 0: break; }
        int[] arr = new int[53];
        arr[54] = 55;
        arr[0] = arr[1];
        double d = x * 0.5;
        long l = x * 1000L;
        String s = "56" + 57;
        char c = 'a' + 58;
        boolean b = x == 59 || x < 60;
        x = x > 0 ? 61 : 62;
        x = (int) 63.5;
        x <<= 64;
        x = x >> 1;
        Runnable r = () -> foo(65);
        java.util.function.IntSupplier is = () -> 66;
        new Thread(() -> { foo(67); });
        int[][] grid = { { 68 }, { 69, 70 } };
        double e = 2.718, pi = 3.14159;
        float f1 = 1.5f;
        x = x * 100 / 3;
        x = 0x7FFFFFFF;
        x = (x + 71) * 72;
        x = Math.max(73, 0);
        x = -(74);
        x = - 75;
        x = +76;
        x = ~77;
        x = !(x > 78) ? 79 : 80;
        assert x > 81 : 82;
        return2(83);
        throw new RuntimeException("" + 84);
    }
    int m2() { return 85; }
    int m3() { return -86; }
    int m4() { return 0; }
    Object m5() { return new Object[] { 87 }; }
    int m6(int a) { return a == 88 ? 89 : 90; }
    class Inner {
        static final int IN = 91;
        int in2 = 92;
        void n() { int z = 93; }
        @Override public int hashCode() { return 94; }
    }
    static { int s = 95; }
    { int t = 96; }
    MagicProbe() { int u = 97; }
    void gen() { java.util.List<Integer> l = java.util.Arrays.asList(98, 99); }
    int ternaryConst = true ? 100 : 101;
    static final int FINAL_WITH_CALL = Math.max(102, 103);
    static final int NESTED_ARR = new int[] { 104 }[0];
    final static String STR = "x".repeat(105);
    static final java.util.Map<String, Integer> MAP = new java.util.HashMap<>(106);
    private static final int[][] TABLE = { { 107, 108 }, { 109 } };
    public static final long BIG = 0xFFFFFFFFL;
    public static final double RATIO = 1.0 / 3.0;
}
