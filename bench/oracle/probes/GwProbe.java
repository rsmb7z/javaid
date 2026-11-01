package ok.pkg;

import java.util.List;
import java.util.Map;
import java.util.function.Supplier;

public class GwProbe<T> {
    List<String> d1;
    List <String> d2;
    List< String> d3;
    List<String > d4;
    List<String>  d5;
    List<String>	d6;
    List<	String> d7;
    List<String	> d8;
    Map<String,List<String>> m1;
    Map<String, List<String> > m2;
    Map<String, List<String >> m3;
    Map<String, List<String>>  m4;
    Map<String, Map<String, List<String>>> m5;
    Map<String, Map<String, List<String>> > m6;
    Map<String, Map<String, List<String> >> m7;
    Map <String, List<String>> m8;
    Map<String, List <String>> m9;
    Map<String,
        List<String>> m10;
    List<
        String> l1;
    List<String
        > l2;
    List<String
    > l3;
    List<?> w1;
    List<? extends Number> w2;
    List<?extends Number> w3;
    List< ?> w4;
    List<? super Integer> w5;
    List<? > w6;
    List<String>[] arr1;
    List<String> [] arr2;
    List<String>[][] arr3;
    List</*c*/String> c1;
    List<String/*c*/> c2;
    List<String> /*c*/ c3;
    List<String>/*c*/ c4;
    Outer<String>.Inner o1;
    Outer<String> .Inner o2;
    Outer<String>. Inner o3;

    public <A> void g1() {}
    public<A> void g2() {}
    public  <A> void g3() {}
    public <A>void g4() {}
    public < A> void g5() {}
    public <A > void g6() {}
    public <A>  void g7() {}
    public static <A extends Comparable<A>> A g8(A a) { return a; }
    public static <A extends Comparable<A> > A g9(A a) { return a; }
    public static <A extends Comparable<A >> A g10(A a) { return a; }
    public static <A extends Comparable<A>>A g11(A a) { return a; }
    public <A extends Number & Comparable<A>> void a1() {}
    public <A extends Number &Comparable<A>> void a2() {}
    public <A extends Number& Comparable<A>> void a3() {}
    public <A extends Number&Comparable<A>> void a4() {}
    public <A extends Number  &  Comparable<A>> void a5() {}
    public <A extends Number &  Comparable<A>> void a6() {}
    public <A extends Number  & Comparable<A>> void a7() {}
    public <A extends Number & Comparable<A>
        & Runnable> void a8() {}
    public <A extends Number
        & Comparable<A>> void a9() {}
    public <A extends Number &
        Comparable<A>> void a10() {}
    public <A extends Number> void a11() {}
    public <A, B> void a12() {}
    public <A,B> void a13() {}
    public <A , B> void a14() {}
    public
        <A> void n1() {}
    public
    <A> void n2() {}
    public <A>
        void n3() {}
    @Deprecated <A> void n4() {}
    @Deprecated<A> void n5() {}
    <A> GwProbe(A a) {}
    <A>GwProbe(A a, int b) {}
    private <A> GwProbe(A a, int b, int c) {}
    private<A> GwProbe(A a, int b, int c, int d) {}

    static class C1<A> {}
    static class C2 <A> {}
    static class C3< A> {}
    static class C4<A > {}
    static class C5<A>{}
    static class C6<A extends List<String>> {}
    static class C7<A extends List<String> > {}
    static class C8<A extends List<String>>{}
    static class C9<A, B> {}
    static class C10<A,B> {}
    interface I1<A> {}
    interface I2 <A> {}
    interface I3<A extends Object> {}
    record R1<A>(A a) {}
    record R2 <A>(A a) {}
    record R3<A> (A a) {}
    record R4<A>(List<A> a) {}
    static class D1 extends ArrayList<String> {}
    static class D2 extends ArrayList <String> {}
    static class D3 extends ArrayList<String>{}
    static class D4 implements Comparable<D4> {
        public int compareTo(D4 o) { return 0; }
    }
    static class D5 implements Comparable<D5>,Runnable {
        public void run() {}
        public int compareTo(D5 o) { return 0; }
    }
    static class D6 implements Comparable<D6> , Runnable {
        public void run() {}
        public int compareTo(D6 o) { return 0; }
    }

    void body(Object x, List<String> p1, List <String> p2, List< String> p3, List<String > p4) {
        Object e1 = new ArrayList<String>();
        Object e2 = new ArrayList <String>();
        Object e3 = new ArrayList< String>();
        Object e4 = new ArrayList<String >();
        Object e5 = new ArrayList<String> ();
        Object e6 = new ArrayList<>();
        Object e7 = new ArrayList <>();
        Object e8 = new ArrayList< >();
        Object e9 = new ArrayList<> ();
        Object e10 = new HashMap<String,List<String>>();
        Object e11 = new HashMap<String, List<String> >();
        Object e12 = new HashMap<String, List<String >>();
        Object e13 = new HashMap<>() {};
        Object e14 = new Outer<String>.Inner();
        Object e15 = new Outer<String> .Inner();
        Object e16 = (List<String>) x;
        Object e17 = (List<String> ) x;
        Object e18 = (List<String>)x;
        Object e19 = (List <String>) x;
        Object e20 = (List< String>) x;
        Object e21 = (List<String >) x;
        Object e22 = (Map<String, List<String>>) x;
        Object e23 = (Map<String, List<String> >) x;
        Object e24 = (Map<String, List<String>> ) x;
        Object e25 = (List<String> & Runnable) x;
        Object e26 = (List<String>& Runnable) x;
        Object e27 = (List<String> &Runnable) x;
        Object e28 = (List<String>&Runnable) x;
        Object e29 = (List<String>  &  Runnable) x;
        Object e30 = (Runnable & List<String>) x;
        Object e31 = (Runnable &List<String>) x;
        Object e32 = (Runnable& List<String>) x;
        Supplier<List<String>> s1 = ArrayList<String>::new;
        Supplier<List<String>> s2 = ArrayList<String> ::new;
        Supplier<List<String>> s3 = ArrayList <String>::new;
        Supplier<List<String>> s4 = ArrayList< String>::new;
        Supplier<List<String>> s5 = ArrayList<String >::new;

        Supplier<List<String>> s7 = ArrayList::<String>new;
        Supplier<List<String>> s8 = ArrayList:: <String>new;
        Supplier<List<String>> s9 = ArrayList::<String> new;
        Supplier<List<String>> s10 = ArrayList::< String>new;
        Supplier<List<String>> s11 = ArrayList::<String >new;
        Object f1 = Foo.<String>bar();
        Object f2 = Foo. <String>bar();
        Object f3 = Foo.< String>bar();
        Object f4 = Foo.<String >bar();
        Object f5 = Foo.<String> bar();
        Object f6 = this.<String>bar();
        Object f7 = this. <String>bar();
        Object f8 = this.<String,Integer>bar();
        Object f9 = this.<String, Integer>bar();
        Object f10 = this.<String , Integer>bar();
        Object f11 = this.<List<String>>bar();
        Object f12 = this.<List<String> >bar();
        Object f13 = this.<List<String >>bar();
        Object f14 = super.<String>bar();
        Object f15 = new Foo().<String>bar();

        boolean b1 = x instanceof List<?>;
        boolean b2 = x instanceof List<String>;
        boolean b3 = x instanceof List <String>;
        boolean b4 = x instanceof List<String> ;
        boolean b5 = x instanceof List<String>&& true;
        boolean b6 = x instanceof List<? extends Number>;
        boolean b7 = x instanceof List< ?>;
        boolean b8 = x instanceof Box<String> bx;
        boolean b9 = x instanceof Box<String>bx;
        boolean b10 = x instanceof Box<String > bx;
        foo(new ArrayList<String>(),new ArrayList<String>());
        foo(new ArrayList<String>() , new ArrayList<String>());
        foo((List<String>) x, (List<String>) x);
        foo((List<String>) x,(List<String>) x);
        foo(x.<String>get(),x.<String>get());
        for (Map.Entry<String,String> en : m.entrySet()) {}
        for (Map.Entry<String, String> en: m.entrySet()) {}
        for (Map.Entry<String, String>en : m.entrySet()) {}
        for (Map.Entry <String, String> en : m.entrySet()) {}
        for (Map.Entry< String, String> en : m.entrySet()) {}
        for (Map.Entry<String, String > en : m.entrySet()) {}
        try (Foo<String> f = open()) {}
        try (Foo <String> f = open()) {}
        try {} catch (Foo e) {}
        List<String> v1 = null, v2 = null;
        List<String> v3=null;
        List<String>v4 = null;
        final List<String> v5 = null;
        final List <String> v6 = null;
        var v7 = new ArrayList<List<String>>();
        var v8 = new ArrayList<List<String> >();
        var v9 = new ArrayList<List <String>>();
        var v10 = new ArrayList<List<String>>() ;
        var v11 = List.<String>of();
        var v12 = x.<String>get().<Integer>get();
        var v13 = x.<String>get() .<Integer>get();
        Object[] ar1 = new List<?>[3];
        Object[] ar2 = new List<String>[3];
        Object[] ar3 = new List <?>[3];
        Object[] ar4 = new List<? >[3];
        Object ar5 = List<String>[].class;
        Object ar6 = List.class;
        if (x instanceof List<String> ls && ls.size() > 1) {}
        if (a < b) {}
        if (a<b) {}
        if (a > b) {}
        if (a>b) {}
        int sh1 = a >> 2;
        int sh2 = a>>2;
        int sh3 = a >>> 2;
        int sh4 = a>>>2;
        boolean cmp = a < b && c > d;
        boolean cmp2 = i < j == k > l;
        Object lam = (Supplier<List<String>>) () -> null;
        Object lam2 = (Supplier<List<String>> ) () -> null;
        switch (x) {
            case Box<String> bs -> {}
            case Box <String> bs -> {}
            case Box< String> bs -> {}
            case Box<String >bs -> {}
            default -> {}
        }
    }

    abstract <A> void ab1();
    abstract<A> void ab2();
    <A extends List<? super A>> void ab3() {}
    <A extends List<? super A> > void ab4() {}
    <A extends List<?super A>> void ab5() {}
    <A extends List<? super A >> void ab6() {}
    <A extends Map<String,A>> void ab7() {}
    <A extends Map<String, A> > void ab8() {}
    <A> List<A> ab9() { return null; }
    <A> List <A> ab10() { return null; }
    <A> List< A> ab11() { return null; }
    <A> List<A >ab12() { return null; }
    List<String> ab13() { return null; }
    List<String>ab14() { return null; }
    List<String> ab15 () { return null; }
    List<List<String>> ab16() { return null; }
    List<List<String> > ab17() { return null; }
    List<List<String>>ab18() { return null; }
    void ab19(List<String>... xs) {}
    void ab20(List<String> ... xs) {}
    void ab21(List<String>...xs) {}



    void ab25(List<String> a,List<String> b) {}
    void ab26(List<String> a , List<String> b) {}
    void ab27(List<String>a) {}
    void ab28(@Anno List<String> a) {}
    void ab29(@Anno List <String> a) {}
    void ab30(final List<String> a) {}
    void ab31(final List<String>a) {}
    void ab32(List<@Anno String> a) {}
    void ab33(List<@Anno String > a) {}
    void ab34(List<@Anno  String> a) {}
    void ab35(List<@Anno(1) String> a) {}
    void ab36(List<String @Anno []> a) {}

    List
        <String> nl1;
    List
    <String> nl2;
    Map<String, List<String>
        > nl3;
    Map<String, List<String>
    > nl4;
    <A extends Comparable<A> & Runnable> void q1() {}
    <A extends Comparable<A>& Runnable> void q2() {}
    <A extends Comparable<A> &Runnable> void q3() {}
    <A extends Comparable<A>&Runnable> void q4() {}
    <A extends Comparable<A>  &  Runnable> void q5() {}
    <A extends Comparable<A>   & Runnable> void q6() {}
    <A extends Comparable<A> 	& Runnable> void q7() {}
    <A extends Comparable<A> & Comparable<A>> void q8() {}
    <A extends Comparable<A> &Comparable<A>> void q9() {}
    <A extends Comparable<A> & Comparable<A> & Runnable> void q10() {}
    <A extends Comparable<A> & Runnable, B extends Comparable<B> & Runnable> void q11() {}
    <A extends Comparable<A>, B extends Comparable<B>> void q12() {}
    <A extends Comparable<A> , B extends Comparable<B>> void q13() {}
    <A extends Comparable<A>,B extends Comparable<B>> void q14() {}
    <A extends Comparable<A> 	 & Runnable> void q15() {}
    <A extends Comparable<A>	& Runnable> void q16() {}
    <A extends Comparable<A>
        & Runnable> void q17() {}
    <A extends Comparable<A> &
        Runnable> void q18() {}
    <A extends Comparable<A> & Runnable & Comparable<A> > void q19() {}
    <A extends Comparable<A> &Runnable &Comparable<A>> void q20() {}

    void more(Object x, int a, int b) {
        Object x1 = new Foo<String>  ();
        Object x2 = new Foo<String>	();
        Object x3 = new Foo<String>
            ();
        Object x4 = new Foo<String> /*c*/ ();
        Object x5 = new Foo<String>/*c*/ ();
        Object x6 = new Foo<String> [3];
        Object x7 = new Foo<String>[3];
        Object x8 = new Foo<String>() {};
        Object x9 = new Foo<String> () {};
        Object x10 = new Outer<String>.Inner<String>();
        Object x11 = new Outer<String>.Inner<String> ();
        Object x12 = new Outer<String> .Inner<String>();
        Object x13 = new Foo<List<String>> ();
        Object x14 = new Foo<List<String> > ();
        Object x15 = new Foo<>  ();
        Object x16 = new Foo<List<String> >.Inner();
        Object x17 = new Foo<String> (); Object x18 = new Foo<String>();
        Object x19 = foo.new Inner<String> ();
        Object x20 = foo.new Inner<String>();
        Object x21 = new <String>Foo();
        Object x22 = new < String>Foo();
        Object x23 = new <String >Foo();
        Object x24 = new <String> Foo();
        Object x25 = new Foo<String>[] {};
        Object x26 = new Foo<String> [] {};
        Object x27 = new Foo<String>[]{};
        Map<String, List<String> > mm1 = a & b;
        Map<String, List<String>> mm2 = a & b;
        Map<String, List<String>>mm3 = a & b;
        Map<String, List<String>> mm4 = a&b;
        Object c1 = (Comparable<List<String>> & Runnable) x;
        Object c2 = (Comparable<List<String>>& Runnable) x;
        Object c3 = (Comparable<List<String> > & Runnable) x;
        Object c4 = (Comparable<List<String>> &Runnable) x;
        Object c5 = (Comparable<List<String>>  & Runnable) x;
        boolean o1 = x instanceof Map<String, List<String>> && a > b;
        boolean o2 = x instanceof Map<String, List<String> > && a > b;
        boolean o3 = x instanceof Map<String, List<String>>&&a>b;
        Foo<String>.Bar<String> t1;
        Foo<String> .Bar<String> t2;
        Foo<String>. Bar<String> t3;
        Foo<String>.Bar<String > t4;
        Foo<Bar<String>>.Baz<String> t5;
        Foo<Bar<String> >.Baz<String> t6;
        Foo<String>:: new;
        List<String>::size;
        List<String> ::size;
        List<String>:: size;
        List<List<String>>::new;
        List<List<String> >::new;
        Foo.<List<String>>bar();
        Foo.<List<String> >bar();
        Foo.<Map<String, List<String>>>bar();
        Foo.<Map<String, List<String> >>bar();
        Foo.<String>
            bar();
        Foo.<String
            >bar();
        Foo
            .<String>bar();
        Foo.
            <String>bar();
        Foo.<String> bar().<String> baz();
        Foo.<String>bar() .<String>baz();
        int s1 = a<b ? a : b;
        boolean s2 = a < b | c > d;
        boolean s3 = a<b|c>d;
        for (List<String> l : ls) {}
        for (List<String> l: ls) {}
        for (List<String>l : ls) {}

        Object lam = (java.util.function.Function<String, List<String>>) s -> null;
        Object lam3 = (Function<String,List<String>>)s -> null;
        label:
        List<String> lb = null;
        Object[] arr = {new Foo<String>(), new Foo<String> ()};
        Object[] arr2 = new Foo<String>[] {null};
        @Anno List<String> an1 = null;
        @Anno(x = 1) List<String> an2 = null;
        @Anno(x = List.<String>m()) List<String> an3 = null;
    }

    record R5<A>  (A a) {}
    record R6<A>	(A a) {}
    record R7<A>
        (A a) {}
    record R8<A>(A a) implements Runnable { public void run() {} }
    record R9<A> (A a) implements Runnable { public void run() {} }
    record R10<A, B> (A a, B b) {}
    record R11<A extends List<String>> (A a) {}
    record R12<A extends List<String> > (A a) {}
    record R13<A>() {}
    record R14<A> () {}
    class K1<A> extends Foo<A> {}
    class K2<A>  extends Foo<A> {}
    class K3<A> extends Foo<A>  {}
    class K4<A> implements Foo<A> {}
    class K5<A>
        extends Foo<A> {}
    interface L1<A> extends Foo<A> {}
    interface L2<A>extends Foo<A> {}
    interface L3<A> {}
    enum E1 implements Foo<String> { X }
    enum E2 implements Foo<String>{ X }
    void more2() {
        Object y1 = new<String>Foo();
        Object y2 = new  <String>Foo();
        Object y3 = new	<String>Foo();
        Object y4 = new
            <String>Foo();
        Object y5 = new <String>Foo<String>();
        Object y6 = new <String> Foo<String> ();



    }
}
