@Deprecated
@SuppressWarnings("x")
package bench.oracle.probes;

import java.lang.annotation.Retention;
import java.lang.annotation.Target;

@Deprecated public class GsAnnLoc {
    @Deprecated
    int a;

    @Deprecated int b;

    @Deprecated @SuppressWarnings("x") int c;

    @Deprecated
      @SuppressWarnings("x")
    int d;

    @Deprecated @SuppressWarnings("x")
    int e;

    @Deprecated
    @SuppressWarnings("x") int f;

    @SuppressWarnings("x") int g;

    @SuppressWarnings({"x", "y"}) int h;

    @SuppressWarnings(value = "x") int i;

    @Deprecated @java.lang.Override int j;

    @java.lang.Deprecated int k;

    @a.B int l;

    @a.b.C int m;

    @a.b.C(x = 1) int n;

    @Retention(java.lang.annotation.RetentionPolicy.RUNTIME) int o;

    @Deprecated() int p;

    @Target({}) int q;

    @Target(@Retention(value = java.lang.annotation.RetentionPolicy.RUNTIME)) int r;

    public @Deprecated int s;

    @Deprecated public int t;

    @Deprecated
    public int u;

    @Deprecated public
    int v;

    @Deprecated final @SuppressWarnings("x") int w;

    @Deprecated
    final
    @SuppressWarnings("x") int x;

    @Deprecated   @SuppressWarnings("x")   int y, z;

    @Deprecated
    @SuppressWarnings("x")
    @SafeVarargs
    int aa;

    @Deprecated @SuppressWarnings("x") @SafeVarargs int bb;

    @Deprecated
        @SuppressWarnings("x") @SafeVarargs
    int cc;

    @Deprecated
    void m1() {
    }

    @Deprecated void m2() {
    }

    @Deprecated @SuppressWarnings("x") void m3() {
    }

    @Deprecated
      @SuppressWarnings("x")
    void m4() {
    }

    @Deprecated <T> void m5() {
    }

    @Deprecated
    <T> void m6() {
    }

    @SuppressWarnings("x") void m7() {
    }

    @SuppressWarnings("x")
    void m8() {
    }

    @Deprecated public void m9() {
    }

    public @Deprecated void m10() {
    }

    @Deprecated
    public
    @SuppressWarnings("x") void m11() {
    }

    @Override public String toString() {
        return "";
    }

    @Deprecated GsAnnLoc() {
    }

    @Deprecated
    GsAnnLoc(int x) {
    }

    @Deprecated @SuppressWarnings("x") GsAnnLoc(int x, int y) {
    }

    @SuppressWarnings("x") <T> GsAnnLoc(T t) {
    }

    void locals(@Deprecated int p1, @Deprecated
            int p2) {
        @Deprecated int l1 = 0;
        @Deprecated
        int l2 = 0;
        @Deprecated @SuppressWarnings("x") int l3 = 0;
        for (@Deprecated int i = 0; i < 1; i++) {
        }
    }

    @Deprecated class N1 {
    }

    @Deprecated
    class N2 {
    }

    @Deprecated @SuppressWarnings("x") class N3 {
    }

    @Deprecated
      @SuppressWarnings("x")
    class N4 {
    }

    @Deprecated interface I1 {
    }

    @Deprecated
    interface I2 {
    }

    @SuppressWarnings("x") interface I3 {
    }

    @Deprecated enum E1 {
        @Deprecated A,
        @Deprecated
        B,
        @Deprecated @SuppressWarnings("x") C,
        @Deprecated
          @SuppressWarnings("x")
        D,
        E,
        @SuppressWarnings("x") F
    }

    @Deprecated record R1(int x) {
    }

    @Deprecated
    record R2(int x) {
        @Deprecated R2 {
        }
    }

    record R3(int x) {
        @Deprecated
        R3 {
        }

        @Deprecated @SuppressWarnings("x") R3(int a, int b) {
            this(a);
        }

        @SuppressWarnings("x")
        R3(int a, int b, int c) {
            this(a);
        }
    }

    @Deprecated @interface A1 {
        @Deprecated int v1();

        @Deprecated
        int v2();

        @SuppressWarnings("x") int v3();

        @Deprecated
          @SuppressWarnings("x")
        int v4();

        @Deprecated
        String v5() default "";

        @Deprecated @SuppressWarnings("x") String v6() default "";
    }

    @Deprecated
    @interface A2 {
    }

    @SuppressWarnings("x") @interface A3 {
    }

    Object o = new Object() {
        @Deprecated int f1;

        @Override public String toString() {
            return "";
        }

        @Override
        public int hashCode() {
            return 0;
        }
    };

    interface Consts {
        @Deprecated int C1 = 1;

        @Deprecated
        int C2 = 2;

        @Deprecated @SuppressWarnings("x") int C3 = 3;
    }

    void more() {
        class Loc {
            @Deprecated int lf;

            @Deprecated void lm() {
            }
        }
    }
}
