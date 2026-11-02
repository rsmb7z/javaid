package bench.oracle.probes;

import java.util.function.Supplier;

public class GsEmptyBody {
    static {}

    static
    {}

    {}

    void a() {}

    void b(){}

    void c()
    {}

    void d()  {}

    void e() /* c */{}

    void f() { }

    void g() {
    }

    void h() {
        // comment
    }

    void i(){
    }

    void j(){ /* c */ }

    GsEmptyBody() {}

    GsEmptyBody(int x){}

    <T> void k(T t)throws Exception{}

    abstract static class A {}

    static class B{}

    static class C
    {}

    interface I{}

    interface J {}

    enum E{}

    enum F {}

    enum G {
        A{},
        B {},
        C
        {};
    }

    @interface Ann{}

    @interface Ann2 {}

    record R(){}

    record S() {}

    record T(int a){
        T{}
    }

    record U(int a) {
        U{}
    }

    void control(int x, Object o, boolean b) throws Exception {
        if (b){}
        if (b) {}
        if (b)  {}
        if (b) {} else{}
        if (b) {} else {}
        if (b) {} else if (x > 1){}
        while (b){}
        while (b) {}
        for (;;){}
        for (int i = 0; i < 1; i++) {}
        for (Object q : new Object[0]){}
        do{} while (b);
        do {} while (b);
        try{} finally{}
        try {} finally {}
        try {} catch (Exception e){}
        try {} catch (Exception e) {}
        try (AutoCloseable r = null){}
        try (AutoCloseable r = null) {}
        synchronized (this){}
        synchronized (this) {}
        switch (x){}
        switch (x) {}
        switch (x)
        {}
        switch (x) {
        }
        switch (x) { case 1: }
        switch (x){ default: }
        {}
        { }
        Runnable r1 = ()->{};
        Runnable r2 = () -> {};
        Runnable r3 = () ->{};
        Runnable r4 = ()-> {};
        Runnable r5 = () -> { };
        Supplier<Object> s1 = () -> { return null; };
        java.util.function.Consumer<Object> c1 = x1->{};
        java.util.function.Consumer<Object> c2 = x2 -> {};
        java.util.function.Consumer<Object> c3 = (x3)->{};
        java.util.function.BiConsumer<Object, Object> c4 = (x4, y4)->{};
        Object n1 = new Object(){};
        Object n2 = new Object() {};
        Object n3 = new Object()  {};
        Object n4 = new Object() { };
        Object n5 = new Object() {
        };
        Object n6 = new Object(){
        };
        Object n7 = new Object() {
            // c
        };
        Object n8 = new Object(){ /* c */ };
        Object n9 = new java.util.ArrayList<String>(){};
        Object n10 = new int[]{};
        Object n11 = new int[] {};
        int[] arr = {};
        int[] arr2 = { };
        label:{}
        label2: {}
        try {} catch (RuntimeException | Error e){}
    }

    void more(Object o) {
        switch (o) {
            case String s -> {}
            case Integer i ->{}
            default -> {}
        }
        int y = switch (o) {
            default -> { yield 1; }
        };
        class Loc{}
        class Loc2 {}
        record Lr(){}
        enum Le{}
        interface Li{}
    }

    void lam() {
        Runnable r = () -> {
            Runnable inner = ()->{};
        };
        new Thread(() ->{}).start();
        new Thread(()->{ }).start();
    }
}
