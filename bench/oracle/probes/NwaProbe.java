import java.util.*;
public class NwaProbe {
    @interface Ann { int v() default 0; }
    @ Ann int f1;
    @Ann int f2;
    @
    Ann int f3;
    int[] a1 = { 1, 2 };
    int[] a2 = {1, 2};
    int[] a3 = {
        1, 2 };
    int[][] a4 = { { 1 }, {2} };
    void m(int x, int[] arr, Object o) {
        int b1 = - x;
        int b2 = -x;
        int b3 = + x;
        int b4 = +x;
        int b5 = ~ x;
        int b6 = ~x;
        boolean c1 = ! true;
        boolean c2 = !true;
        int d1 = x ++;
        x ++;
        ++ x;
        ++x;
        -- x;
        x --;
        x--;
        String s1 = o. toString();
        String s2 = o.toString();
        String s3 = o.
            toString();
        String s4 = o
            .toString();
        int e1 = arr [0];
        int e2 = arr[ 0];
        int e3 = arr[0] [0 == 0 ? 0 : 0];
        int[] g1 [] = null;
        int g2 [] = null;
        int[] g3 = new int [3];
        int[] g4 = new int[3];
        int[][] g5 = new int[3] [3];
        int[] g6 = new int [] { 1 };
        int[] g7 = new int[] { 1 };
        Object h1 = (Object) - x;
        Object h2 = - - x;
        int i1 = x - - x;
        int i2 = x - -x;
        int i3 = -   x;
        int i4 = - /* c */ x;
        int i5 = -
            x;
        int i6 = ~
            x;
        List<int[]> l = null;
        Object j1 = java.util. List.class;
        Object j2 = NwaProbe. class;
        Runnable r = NwaProbe :: new;
        String k1 = int[]. class.getName();
        @ Deprecated int k2 = 1;
        @Ann(v = 1) int k3 = 1;
        @Ann ( v = 1) int k4 = 1;
    }
    int[] n1() [] { return null; }
    int n2(int ... xs) { return 0; }
    int n3(int[] ... xs) { return 0; }
    @ SuppressWarnings("x") void n4() { }
    void p(Object o) {
        if (o instanceof String s) { }
        o = ( - 1 );
        o = (- 1);
        int q = o.hashCode( ) - 1;
        long t = - 1L;
        double u = - 1.0;
        Object v = - (1);
    }
}
