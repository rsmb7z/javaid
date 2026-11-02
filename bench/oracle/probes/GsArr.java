package bench.oracle.probes;

import java.util.List;
import java.util.function.IntFunction;

public class GsArr {
    int[] a1 = {1, 2};
    int [] a2 = {1, 2};
    int[ ] a3 = {1, 2};
    int [ ] a4 = {1, 2};
    int a5[] = {1};
    int a6 [] = {1};
    int a7[ ] = {1};
    int[]a8 = {1};
    int[] a9[] = {{1}};
    int[][] b1 = new int[2][3];
    int[][] b2 = new int [2] [3];
    int[][] b3 = new int[ 2 ][ 3 ];
    int[][] b4 = new int[2] [3];
    int[][] b5 = {{1}, {2}};
    int[][] b6 = new int[][] {{1}};
    int[][] b7 = new int[][]{{1}};
    int[] b8 = new int[]{1};
    int[] b9 = new int[] {1};
    String[] c1 = new String[0];
    String[]  c2 = null;
    String [] c3 = null;
    List<int[]> c4 = null;
    List<int []> c5 = null;
    List<List<int[]>> c6 = null;
    List<int[] > c7 = null;
    List<int[]>[] c8 = null;
    Class<?> c9 = int[].class;
    Class<?> d1 = int [].class;
    Class<?> d2 = String[][].class;
    IntFunction<int[]> d3 = int[]::new;
    IntFunction<int[]> d4 = int []::new;
    Object d5 = (int[]) null;
    Object d6 = (int []) null;
    Object d7 = (int[])null;
    Object d8 = (int[]) (Object) null;

    int g1(int[] p, int[]... q) {
        int[] l = p;
        int x = l[0];
        int y = l [0];
        int z = l[ 0];
        int w = l[0 ];
        int v = l[0]+1;
        int u = l[0] + 1;
        int t = l[0]  + 1;
        int s = l[0]++;
        int r = l[0] ++;
        l[0]++;
        l [0]++;
        l[0] = 1;
        l[0]= 1;
        l[0] =1;
        l[0]=1;
        l[l[0]] = 2;
        l[l [0]] = 3;
        l[ l[0] ] = 4;
        foo(l[0], l[1]);
        foo(l[0],l[1]);
        foo(l[0] , l[1]);
        foo(l[0]);
        foo( l[0] );
        foo(l [0] );
        foo(l[0] /* c */, l[1]);
        foo(l[0]/* c */, l[1]);
        foo(l[0] // c
        );
        foo(l[0]
        );
        x = l[0] > 1 ? l[1] : l[2];
        x = l[0]>1 ? l[1]:l[2];
        x = l[0] >> 1;
        x = l[0]>>1;
        boolean bb = l[0] > 1 && l[1] < 2;
        boolean b2 = l[0]>1&&l[1]<2;
        boolean b3 = l[0] instanceof Object;
        int[][] m = new int[2][2];
        m[0][1] = m[1][0];
        m [0] [1] = 1;
        m[0] [1] = 1;
        m[0][ 1] = 1;
        x = m[0][1]+m[1][0];
        String ss = foo(l)[0] + foo(l) [1];
        l = foo(l)[0].clone();
        Object o = l[0].getClass();
        Object oo = l[0] .getClass();
        o = new int[3].length;
        o = new int[]{1, 2}[0];
        o = new int[] {1, 2} [0];
        for (int e : l) {
        }
        for (int i = 0; i < l[0]; i++) {
        }
        for (int i = 0; i<l[0]; i++) {
        }
        for (int i = l[0];i<2;i++) {
        }
        return l[0];
    }

    int[] g2() { return new int[0]; }
    int g3()[] { return null; }
    int [] g4() { return null; }
    int g5 () [] { return null; }

    @interface Ann {
        int[] v1();
        int [] v2() default {};
        String[] v3() default {"a", "b"};
    }

    void g6(@Deprecated int @Deprecated [] p, int @Deprecated [] @Deprecated [] q) {
    }

    int @Deprecated [] an1;
    int@Deprecated [] an2;
    int @Deprecated[] an3;
    int @Deprecated [] @Deprecated [] an4;
    @Deprecated int[] an5;

    void g7(int... r) {
    }

    void g8() {
        int[] arr = {1};
        int n = arr[0]; int k = arr [0];
        int j = arr
            [0];
        int i = arr[
            0];
        int h = arr[0
            ];
        int g = arr[0]
            + 1;
        int f = arr[0] +
            arr[0];
        int e = arr[0]; int[] d = null;
        String s = "[ ] [x]";
        char c = '[';
        /* [ ] */ // [ ]
        int x = arr[0];int y = arr [0];
    }
}
