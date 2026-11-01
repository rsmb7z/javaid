package ok.pkg;

import java.util.List;

public class SpProbe {
    void f1(
        int a
    ) {}
    void f2
    (int a)
    {}
    void f3(int a) {}
    SpProbe(
        int a
    ) {}
    SpProbe
    (long a)
    {}
    SpProbe(int a, int b) {
        this(
            a
        );
    }
    SpProbe(int a, long b) {
        this
        (a, b);
    }
    SpProbe(long a, long b) {
        super(
        );
    }
    SpProbe(long a, int b) {
        super
        ();
    }
    record R1(
        int a
    ) {}
    record R2
    (int a)
    {}
    @interface An1 {
        int x(
        );
        int y
        ();
        int z();
    }
    @An1(
        x = 1
    )
    class C1 {}
    @An1
    (x = 1)
    class C2 {}
    enum E1 {
        A(
            1
        ),
        B
        (2),
        C(3);
        E1() {}
        E1(int a) {}
    }
    Object g1 = foo(
        1
    );
    Object g2 = foo
    (1);
    Object g3 = new Object(
    );
    Object g4 = new Object
    ();
    Object g5 = (
        Object
    ) g1;
    Object g6 =
    (Object) g1;
    int g7 = (
        1 + 2
    ) * 3;
    int g8 = 3 *
    (1 + 2);
    Runnable g9 = (
    ) -> {};
    Runnable g10 =
    () -> {};
    java.util.function.IntUnaryOperator g11 = (
        int a
    ) -> a;
    Object g12 = Foo.<String>bar(
        1
    );
    Object g13 = foo(
        1
    ).bar(
        2
    );

    void body(boolean a, Object o, int[] qs, Object x) throws Exception {
        if (
            a
        ) x();
        if
        (a)
        x();
        while (
            a
        ) x();
        while
        (a) x();
        for (
            int i = 0; i < 1; i++
        ) x();
        for
        (int i = 0; i < 1; i++) x();
        for (
            int q : qs
        ) x();
        switch (
            qs.length
        ) {}
        switch
        (qs.length) {}
        synchronized (
            o
        ) {}
        synchronized
        (o) {}
        try (
            Res r = open()
        ) {}
        try
        (Res r = open()) {}
        try {} catch (
            Exception e
        ) {}
        try {} catch
        (Exception e) {}
        do {} while (
            a
        );
        do {} while
        (a);
        foo(
            1
        );
        foo
        (1);
        Object o1 = new Foo(
            1
        );
        Object o2 = new Foo
        (1);
        boolean b1 = x instanceof Point(
            int p
        );
        boolean b2 = x instanceof Point
        (int p);
        boolean b3 = x instanceof Point(int p, int q);
        x = (
            a ? 1 : 2
        );
        x = (a
        ) ? 1 : 2;
        x = ((
            a
        ) && true);
        Object p1 = switch (
            qs.length
        ) { default -> 1; };
        assert (
            a
        );
        return;
    }
}
