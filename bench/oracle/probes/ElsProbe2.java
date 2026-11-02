/*
 * License header.
 */

@Deprecated
package p;

import module java.base;
import static java.lang.Math.abs;
import java.util.*;
// c
import module java.sql;
/* c */
import java.io.*;
public enum ElsProbe2 {
    A,
    // comment about B


    B,
    /* block */

    C(1) {
        void m() { }
        void n() { }
    },


    D,
    E { int q; },
    F, G,

    H;
    int field1;
    int field2;
		int tabbed;

		int tabbed2;


		int tabbed3;
    ElsProbe2() { }
    ElsProbe2(int a) { }

    String é = "é";
    String ü = "ü";
    /**
     * Javadoc.
     */
    @Deprecated

    int docAnnotated;


    @SuppressWarnings("a")

    // comment between
    int commentBetween;


    @SuppressWarnings("a")
    // comment between
    /* another */
    int commentBetween2;

    public static final int
        multi1 = 1,
        multi2 = 2;


    private int arr[], arr2[][];
    private final java.util.Map<String,
        java.util.List<String>> generic = null;



    private final java.util.Map<String,
        java.util.List<String>> generic2 = null;


    int viaLambda() {
        Runnable r = () -> {
            int q;


            int w;
        };
        class Local {
            int a;
            int b;
            void m() { }
            void n() { }
        }
        interface LI {
            void a();
            void b();
        }
        int x = 1;
        int y = 2;
        for (int i = 0, j = 0; i < 3; i++) {
            int z;
        }
        switch (x) {
            case 1:
                int k;


                int l;
                break;
            default:
        }
        new Thread() {
            int a;
            int b;


            int c;
            public void run() { }
            public void run2() { }
        }.start();
        return 1;
    }
    static {
    }
    // comment before static init
    static {
    }


    // comment before static init
    static {
        foo();


        bar();
    }

    /* c */


    /* d */
    static {
    }
    /* c */

    /** x */
    {
    }

    sealed interface S permits S1, S2 { }
    final class S1 implements S { }
    non-sealed class S2 implements S { }
    @interface AnnoDef {
        int a();
        int b() default 1;
        String c() default "";
    }
    interface Fields {
        int A = 1;
        int B = 2;

        int C = 3;
        int D = 4;
        // c
        int E = 5;
        void m();
    }
    record Point(int x, int y) {
        Point {
            if (x < 0) { }
        }
        Point(int x) {
            this(x, 0);
        }
        static Point origin() { return null; }
        static int count;
        static int count2;
    }
    <T extends Comparable<T>> void generic(T a) { }
    <T> void generic2(T a) { }
    public void a() { } public void b() { }
    public void c() {
    } // trailing
    public void d() { }
    // after d


    public void e() { }
    void f() { /* c */ }
    void g() { } // c
    void h() { } /* c */ void i() { }
    int
    split;
    int fieldA; // trailing comment
    int fieldB; /* trailing block */
    int fieldC; // trailing

    int fieldD;
    // before method
    void method() { }
    /* before field */
    int fieldE;
}
