import java.math.BigInteger;
public class MagicProbe2 {
    static final double A = BigInteger.valueOf(3L).multiply(BigInteger.valueOf(4L)).doubleValue();
    static final int B = Integer.valueOf(5).hashCode();
    static final double C = -0.0;
    static final float D = -0f, E = 0f;
    int f(int x) {
        x = x & 0xffffffff;
        long l = 0xffffffffffffffffL;
        long m = 0xffffffffL;
        x = 037777777777;
        x = 0b11111111111111111111111111111111;
        x = 0x80000000;
        x = 0x7fffffff;
        double d = -0.0 + 0.0 + 1e0 + 2.0e0 + 1.0f + 0x1.0p1;
        long big = 9223372036854775807L;
        x = -2147483648;
        x = 1_0 + 0_7 + 1__0;
        return x;
    }
}
