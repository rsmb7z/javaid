package ok.pkg.
    sub;

import java.
    util.List;
import java.util
    .Map;
import static java.lang.Math.
    max;
import java.util.
    *;

@Deprecated
public class SwProbe<A,
    B> extends java.util.
    ArrayList<A> implements java.io.Serializable,
    Comparable<SwProbe<A, B>>
    , Runnable {
    enum E { X,
        Y
        , Z
        ;
        void m() {}
    }
    enum F { P, Q; }
    enum G {
        P,
        Q,
        ;
    }
    int[] arr = {1,
        2
        , 3};
    int[] arr2 = {1, 2, 3};
    int[] arr3 = {1, 2,
    };
    int a1, a2,
        a3
        , a4;
    int a5, a6;
    int d1[] = new int[3], d2[];
    int[] t1,
        t2;
    int
        [] t3;
    int t4
        [];
    int t5[
        ];
    int[
        ] t6;
    int[] t7
        [];
    int[]
        [] t8;
    int[][] t9;
    Object o1 = new int[
        3];
    Object o2 = new int
        [3];
    Object o3 = new int[3
        ];
    Object o4 = new int[3]
        [4];
    Object o5 = new int[3][
        4];
    Object o6 = new int[] {1,
        2};
    Object o7 = new int[]
        {1, 2};
    Object o8 = new String[]
        {"a"};
    Object o9 = arr[
        0];
    Object o10 = arr
        [0];
    Object o11 = arr[0
        ];
    Object o12 = arr[0]
        [1];
    Object o13 = int[].class;
    Object o14 = int[]
        .class;
    Object o15 = int
        [].class;
    Object o16 = java.util.
        List.class;
    Object o17 = java.util
        .List.class;
    Object o18 = this.
        a1;
    Object o19 = this
        .a1;
    Object o20 = SwProbe.
        this;
    Object o21 = SwProbe
        .this;
    Object o22 = SwProbe.super.
        toString();
    Object o23 = foo.
        new Inner();
    Object o24 = foo
        .new Inner();
    Object o25 = Foo.
        <String>bar();
    Object o26 = Foo
        .<String>bar();
    Object o27 = a.b.
        c.
        d();
    Object o28 = a
        .b
        .c()
        .d();
    Object o29 = a.b.c.d();
    Object o30 = a. // c
        b;
    Object o31 = a /* c */ .
        b;
    Object o32 = a
        /* c */ .b;
    Object o33 = a. /* c */
        b;
    Object o34 = a.
        // c
        b;
    Object o35 = a.	
        b;
    Object o36 = a
        .
        b;
    Object o37 = a.
        // c
        b.
        c;
    Object o38 = (a).
        b;
    Object o39 = (a)
        .b;
    Object o40 = foo().
        bar();
    Object o41 = foo()
        .bar();
    Object o42 = "x".
        length();
    Object o43 = "x"
        .length();
    Object o44 = new Foo().
        bar();
    Object o45 = new Foo()
        .bar();
    Object o46 = new Foo() {
    }.
        bar();
    Object o47 = new Foo() {
    }
        .bar();

    void v1(String... xs) {}
    void v2(String
        ... xs) {}
    void v3(String ...
        xs) {}
    void v4(String...
        xs) {}
    void v5(String ... xs) {}
    void v6(String /* c */ ...
        xs) {}
    void v7(String
        /* c */ ... xs) {}
    void v8(String... // c
        xs) {}
    void v9(int a,
        String... xs) {}
    void v10(int a
        , String... xs) {}
    void v11(@Anno
        String... xs) {}
    void v12(final String...
        xs) {}

    void r1(Function<String, Integer> f) {
        f = String::
            length;
        f = String
            ::length;
        f = String::length;
        f = String ::
            length;
        f = String:: // c
            length;
        f = String
            ::
            length;
        f = String /* c */
            ::length;
        f = String /* c */ ::length;
        f = String::/* c */
            length;
        f = this::
            foo;
        f = this
            ::foo;
        f = List<String>::
            size;
        f = List<String>
            ::size;
        f = int[]::
            new;
        f = int[]
            ::new;
        f = ArrayList::
            new;
        f = foo()
            ::bar;
        f = foo().bar()::
            baz;
    }

    void s1(int a, int b, int c) throws java.io.IOException,
        InterruptedException {
        foo(a,
            b);
        foo(a
            , b);
        foo(a, b);
        foo(a ,
            b);
        foo(a, // c
            b);
        foo(a /* c */ ,
            b);
        foo(a
            /* c */ , b);
        foo(a, /* c */
            b);
        foo(a,
            // c
            b);
        foo(a,	
            b);
        foo(a
            ,
            b);
        foo(a,b,
            c
            ,d);
        foo(new int[] {1,
            2}, 3);
        foo(Map.<String,
            Integer>of());
        foo((a,
            b) -> a);
        foo((a
            , b) -> a);
        foo(x -> {
            return 1;
        }, b);
        foo(x -> {
            return 1;
        }
            , b);
        for (int i = 0,
                 j = 1;
             i < 1;
             i++,
             j++) {}
        for (int i = 0
                 , j = 1
             ; i < 1
             ; i++
             , j++) {}
        for (int i = 0; i < 1; i++) {}
        for (int i = 0;
             i < 1;
             i++) {}
        for (int i = 0
             ; i < 1
             ; i++) {}
        for (;;) {}
        for (;
             ;
             ) {}
        for (
             ;;) {}
        for (int i = 0;; i++) {}
        for (int i = 0;
             ;
             i++) {}
        for (int i = 0
             ;
             ; i++) {}
        for (int x :
             xs) {}
        a++;
        a++
            ;
        a++ // c
            ;
        a++;  // c
        a++; /* c */
        a++ ;
        ;
        a++;;
        a++; ;
        a++;
        ;
        a++
        ;
        {
            ;
        }
        if (a > 0) ;
        if (a > 0)
            ;
        if (a > 0) {
            a++;
        };
        while (a > 0) ;
        label: ;
        try (Res r = open();
             Res q = open()) {}
        try (Res r = open()
             ; Res q = open()) {}
        try (Res r = open();
             Res q = open();) {}
        try (Res r = open(); Res q = open()) {}
        switch (a) {
            case 1,
                 2 -> a++;
            case 3
                , 4 -> a++;
            case 5, 6 -> a++;
            default -> a--;
        }
        switch (a) {
            case 1: a++; break;
            case 2:
                a++;
                break;
            default: ;
        }
        assert a > 0 ;
        return;
    }
    @Anno(x = {1,
        2}, y = 3,
        z = 4
        , w = 5)
    void an() {}
    @Anno(x = {1, 2}, y = 3)
    void an2() {}
    <T,
        U> void gen1() {}
    <T
        , U> void gen2() {}
    <T, U> void gen3() {}
    Map<String,
        Integer> g1;
    Map<String
        , Integer> g2;
    Map<String, Integer> g3;
    record R(int a,
        int b
        , int c) {}
    record R2(int a, int b) {}
    static {
        ;
    }
    ;
    int after;;
    int after2;
    ;
}
