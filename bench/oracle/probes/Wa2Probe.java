@Deprecated package ok.pkg;
//comment at the top
// spaced comment
import java.util.List;
import java.util.Map;

//no space after the slashes
/// three slashes, spaced
///three slashes, tight
//// four slashes, spaced
////four slashes, tight
//
///
////
//	tab after the slashes
//#hash
//- dash
//:colon
//*star
// 	space and tab
//   many spaces
//é accented
// é accented

@interface Anno { int value() default 0; }

@Deprecated class K1 {}
@Deprecated(since = "1")class K2 {}
@Deprecated(since = "1") class K3 {}
@Deprecated
class K4 {}
@Deprecated @SuppressWarnings("x") class K5 {}
@Deprecated@SuppressWarnings("x") class K6 {}
@java.lang.Deprecated class K7 {}
@java.lang.Deprecated(since = "1")class K8 {}
@a.b.Anno class K9 {}
@a.b.Anno(1)class K10 {}
@Anno(value = {@Anno(1), @Anno(2)}) class Q1 {}
@Anno(@Anno(1)) class Q2 {}
@Anno(value = {@Anno(1),@Anno(2)},other = @Anno(3), more = @Anno(4)) class Q3 {}

public class Wa2Probe<@Anno T, @Anno(1)U, @Anno
        V> extends @Anno Object implements @Anno(1)Runnable, @Anno Comparable<Wa2Probe<T, U, V>> {
    enum E {
        @Anno A,
        @Anno(1)B,
        @Anno
        C,
        @Deprecated @Anno D,
        @Deprecated@Anno F;
    }
    List<@Anno String> a1;
    List<@Anno(1)String> a2;
    List<@Anno @Anno(1) String> a3;
    List<@Anno@Anno(1) String> a4;
    Map<@Anno String,@Anno(1)Integer> a5;
    List<@Anno ? extends Number> a6;
    List<@Anno(1)? extends Number> a7;
    String @Anno [] b1;
    String @Anno(1)[] b2;
    int @Anno [] @Anno(1)[] b3;
    int @Anno [][] b4;
    String[] @Anno [] b5;
    Outer.@Anno Inner c1;
    Outer.@Anno(1)Inner c2;
    java.util.@Anno List<String> c3;
    java.util.@Anno(1)List<String> c4;
    @Anno String d1;
    @Anno(1)String d2;
    @Anno
    String d3;
    @Anno public String d4;
    public @Anno String d5;
    public @Anno(1)String d6;
    public @Anno @Anno(1) String d7;
    public @Anno@Anno(1) String d8;
    final @Anno int d9 = 1;
    @a.b.Anno int d10;
    @a.b.Anno(1)int d11;
    @a.b.Anno
    int d12;

    void m1(String @Anno ... xs) {}
    void m2(String @Anno(1)... xs) {}
    void m3(@Anno String... xs) {}
    void m4(@Anno(1)String... xs) {}
    void m5(@Anno int p, final @Anno(1) int q) {}
    void m6(@Anno Wa2Probe this) {}
    void m7(@Anno(1)Wa2Probe this, int x) {}
    @Anno String m8() { return null; }
    @Anno(1)String m9() { return null; }
    <@Anno X> void m10() {}
    <@Anno(1)X> void m11() {}
    void m12() throws @Anno Exception {}
    void m13() throws @Anno(1)Exception {}
    void m14() throws @Anno RuntimeException, @Anno(1)Error {}
    @Anno <X> void m15() {}
    @Anno(1)<X> void m16() {}

    void body(Object x) {
        Object n1 = new @Anno Object();
        Object n2 = new @Anno(1)Object();
        Object n3 = new @Anno String @Anno [3];
        Object n4 = new @Anno(1)String @Anno(1)[3];
        Object n5 = new String @Anno [] {};
        Object n6 = (@Anno String) x;
        Object n7 = (@Anno(1)String) x;
        Object n8 = (@Anno String & @Anno(1)Runnable) x;
        boolean n9 = x instanceof @Anno String;
        boolean n10 = x instanceof @Anno(1)String;
        boolean n11 = x instanceof @Anno String s;
        boolean n12 = x instanceof final @Anno String s2;
        boolean n13 = x instanceof @Anno(1)String s3;
        boolean n14 = x instanceof final @Anno(1)String s4;
        boolean n15 = x instanceof @Anno @Anno(1)String;
        @Anno int l1 = 1;
        @Anno(1)int l2 = 1;
        @Anno
        int l3 = 1;
        final @Anno int l4 = 1;
        @Anno final int l5 = 1;
        @Anno@Anno(1) int l6 = 1;
        for (@Anno String s : new String[0]) {}
        for (@Anno(1)String s : new String[0]) {}
        try (@Anno Res r = open()) {} catch (final @Anno Exception e) {}
        try {} catch (@Anno(1)Exception e) {}
        Runnable r = (@Anno String q) -> {};
        Function<String, String> f = (@Anno(1)String q) -> q;
        Object o = List.<@Anno String>of();
        Object p = List.<@Anno(1)String>of();
        Object q = Foo::<@Anno String>bar;
    }

    @Anno
    // comment between
    void m17() {}
    @Anno /* c */ void m18() {}
    @Anno(1)/* c */void m19() {}
    @Anno// c
    void m20() {}

    void commentsInCode() {
        int a = 1; //trailing tight
        int b = 2; // trailing spaced
        int c = 3;//trailing tight no gap
        String s = "//not a comment";
        char ch = '/';
        /*//not a comment either*/
        /* c */ //tight after block
        //
        foo(a, //tight in args
            b);
        foo(//tight first
            a);
        else_();
    }

    void elseAndDo(boolean a) {
        if (a) x(); else;
        if (a) x(); else ;
        if (a) x(); else x();
        if (a) {} else {}
        if (a) {}else{}
        do;while (a);
        do ; while (a);
        do {} while (a);
        do{}while(a);
        while (a);
        while(a);
        for (;;);
        for(;;);
        return;
    }
}
