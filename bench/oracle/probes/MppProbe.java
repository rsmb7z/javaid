package ok.pkg;

import java.util.List;

public class MppProbe {
    enum E { X (1), Y(2), Z
        (3); E (int a) {} }
    record R (int a) {}
    record R2(int a) { R2 (int a, int b) { this(a); } }
    @interface A { int v () default 1; }
    MppProbe (int a) { this (a, 1); }
    MppProbe(int a, int b) { super (); }
    MppProbe(int a, int b, int c)
    { super
        (); }
    void m (int a) {}
    void n(int a) {}
    void o(int a)
    {}
    void p
        (int a) {}
    void body(Object o, List<String> xs) {
        foo (1);
        foo(2);
        foo
            (3);
        this.foo (4);
        this.foo
            (5);
        o.hashCode ();
        o.hashCode()
            .toString ();
        xs.<String>get (0);
        xs.stream().map (x -> x);
        new Object ();
        new Object();
        new Object
            ();
        new Object () {};
        new MppProbe (1);
        new int [2];
        new int[] {1};
        Runnable r = () -> {};
        Runnable q = this::foo;
        String t = ("a" + "b") .trim ();
        new MppProbe(1) .foo (2);
        foo (
            6);
        foo 
            (7);
        foo	(8);
        if (o instanceof R (int z)) {}
        if (o instanceof R
            (int z)) {}
        if (o instanceof R(int z)) {}
    }
    void foo(int a) {}
}
