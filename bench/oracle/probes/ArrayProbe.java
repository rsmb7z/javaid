package ok.pkg;

import java.lang.annotation.ElementType;
import java.lang.annotation.Target;
import java.util.List;

/** C-style array brackets after a name (ArrayTypeStyle). One distinctive case per line. */
public class ArrayProbe {
    @Target(ElementType.TYPE_USE) @interface TA {}
    // ---- from ArrayProbe
    int a[];
    int[] b;
    int c[], d;
    int[] e[];
    int[][] f;
    int g[][];
    int[] h, i[];
    String[] j = new String[0];
    String k[] = {"a"};
    int[] m() { return null; }
    int n()[] { return null; }
    void p(int a[]) {}
    void q(int[] a) {}
    void r(int... a) {}
    void s(int a[], int[] b) {}
    void t() {
        int a[] = new int[1];
        int[] b = new int[1];
        for (int x[] : new int[1][1]) {}
        for (int y[] = new int[1], z[] = null;;) {}
        List<String> list[];
        String[] arr[] = null;
        int[] cc, dd[], ee;
        int multi
            [] = null;
        int[]
            split[] = null;
    }
    record R(int[] a) {}
    enum E { A; int z[]; }
    interface I { int X[] = {1}; int m(int a[]); }
    @interface Ann { int v()[] default {}; }
    Runnable lam = () -> { int a[] = null; };
    java.util.function.Function<int[], int[]> fn = (int a[]) -> a;
    java.util.function.Function<int[], int[]> fn2 = (int[] a) -> a;
    void gen(List<String> x[], List<String>[] y) {}
    abstract class Ab { abstract int[] q(); abstract int r()[]; }
    void pattern(Object o) { if (o instanceof int[] ia) {} }
    // ---- from ArrayProbe2

    int multiline(
        int x)[] { return null; }
    int splitBracket[
    ];
    int annotated @TA [];
    int annotated2 @TA [] @TA [];
    void res() throws Exception { try (java.io.InputStream in[] = null) {} }
    Object anon = new Object() { int q[]; };
    void generic(java.util.Map<String, int[]> m[]) {}
    int twoLines[]
        [];
    int sameLineName[] /* c */ [];
    int[] create = new int[] {1};
    int[][] create2 = new int[2][];
    Object o = new int[0].length;
    void sp(int... args) {}
    void spc(String... args[]) {}
    int fieldA, fieldB[], fieldC[][];
    // ---- from ArrayProbe3

    int v1
        @TA [];
    int n2(int x)
        [] { return null; }
    int n3
        (int x)[] { return null; }
    int n4(int x)[]
        [] { return null; }
    int n5(int x) [] /* c */
        [] { return null; }
    int n6(
        int x
    )[] { return null; }
    void p1(int a
        [], int b[]) {}
    void p2(int a[],
            int b[]) {}
    int f1 = 1, f2
        [] = null;
    int n7(int x)
        @TA [] { return null; }
    // ---- from ArrayProbe4
    int a1(int x)[] { return null; }
    int a2(
        int x)[] { return null; }
    int a3(int x
        )[] { return null; }
    int a4(
        )[] { return null; }
    int a5(
        int x
        )[] { return null; }
    int a6()[] { return null; }
    int a7(
    )[]
        [] { return null; }
    int a8(int x, int y
        )[] { return null; }
    int a9(int x,
           int y)[] { return null; }
    int b1(int x[]
        )[] { return null; }
    int b2(
        int x) [] { return null; }
    int b3(int x
    ) throws Exception { return 0; }
    int[] b4(int x
        ) { return null; }
    // ---- from ArrayProbe6
    int x1(
        int a
      )[] { return null; }
    int x2(
        int a
       )[] { return null; }
    int x3(
        int a
        )[] { return null; }
    int x4(
        int a
         )[] { return null; }
    int longerName(
        int a
              )[] { return null; }
    int y(
        int a
     )[] { return null; }
    int g1,
        g2[];
    int h1,
  h2[];
    int k1 = 0,
        k2
        [] = null;
}
