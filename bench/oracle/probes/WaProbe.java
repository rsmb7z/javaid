package ok.pkg;

import java.util.List;

public class WaProbe<A,B> {
    enum E { X,Y,Z; }
    int[] arr = {1,2,3};
    int[] arr2 = {1, 2, 3};
    int[] arr3 = {1,};
    int[] arr4 = {1, 2,};
    int[] arr5 = {1,
        2,
        3};
    void f(int a,int b) {}
    void g(int a, int b) {}
    void h(int a,
           int b) {}
    void i(int a, 
           int b) {}
    void body(List<String> xs, int n, Object o) {
        foo(1,2);
        foo(1, 2);
        foo(1 ,2);
        foo(1,/* c */ 2);
        foo(1,// c
            2);
        int a=1;int b=2;
        int c = 1; int d = 2;
        int e = 1;	int f = 3;
        a++;}
    void s2(int n) {
        for(int i=0;i<2;i++) {}
        for (int i = 0; i < 2; i++) {}
        for(;;) { break; }
        for (;;) { break; }
        for (int i = 0;;i++) {}
        for (int i = 0; ;) {}
        for (int i = 0;i < 2;) {}
        for (; n < 2;) {}
        for (; n < 2; ) {}
        ;;
        ; ;
        foo(1);// c
        foo(1);/* c */
        foo(1);
        try (java.io.StringReader r = null;java.io.StringReader r2 = null;) {} catch (Exception ex) {}
        try (java.io.StringReader r = null; java.io.StringReader r2 = null) {} catch (Exception ex) {}
        return;
    }
    class Inner {};
    class Inner2 {}; 
    void casts(Object o, int x, double dd) {
        int a = (int)x;
        int b = (int) x;
        int c = (int)(x);
        int d = (int)-x;
        int e = (int)+x;
        String s = (String)o;
        String s2 = (String) o;
        List<String> l = (List<String>)o;
        List<String> l2 = (List<String>) o;
        Object r = (Runnable & java.io.Serializable)() -> {};
        Object r2 = (Runnable & java.io.Serializable) () -> {};
        int g = (int)
            x;
        int h = ((int)x);
        int i = ((int) x);
        long k = (long)dd;
        Object t = (Object)null;
        boolean u = (boolean)true;
        int v = (int)'a';
        int w = (int)"a".length();
        int y = (int)x++;
        int z = (int)new Integer(1);
        int q = (int)(x);
        int p = (int)this.hashCode();
    }
    void kw(boolean x, int n) {
        if(x) {}
        if (x) {}
        if
            (x) {}
        if (x) {}else{}
        if (x) {} else {}
        if (x) {}
        else
        {}
        if (x) {} else if(x) {} else if (x) {}
        if (x) foo(1); else foo(2);
        if (x) foo(1); else{ foo(2); }
        while(x) {}
        while (x) {}
        while
            (x) {}
        do{} while(x);
        do {} while (x);
        do
        {} while
            (x);
        do foo(1); while (x);
        do foo(1);while(x);
        for(int i : new int[0]) {}
        for (int i : new int[0]) {}
        for
            (int i : new int[0]) {}
        x = x ? true : false;
        switch(n) { default: }
        synchronized(this) {}
        try{} finally{}
        try {} catch(Exception e) {}
        return;
    }
    int ret1(int a) { return(a); }
    int ret2(int a) { return (a); }
    int ret3(int a) { return a; }
    int ret4(int a) {
        return
            a;
    }
    void ret5() { return; }
    void ret6() { return ; }
    int yl(int a) {
        return switch (a) {
            case 1 -> { yield(2); }
            case 2 -> { yield 3; }
            case 3 -> { yield
                4; }
            default -> 0;
        };
    }
    void cs(int n, Object o) {
        switch (n) {
            case(1): break;
            case 2: break;
            case 3,4: break;
            default: break;
        }
        switch(n) {
            case 5 -> {}
            default -> {}
        }
        switch (n) { case 1: case 2: break; }
    }
    void lam() {
        java.util.function.IntUnaryOperator a = x->x;
        java.util.function.IntUnaryOperator b = x -> x;
        java.util.function.IntUnaryOperator c = x ->x;
        java.util.function.IntUnaryOperator d = (x)->x;
        Runnable e = ()->{};
        Runnable f = () -> {};
        Runnable g = () ->
            {};
        Runnable h = ()->
            {};
    }
    void tr() {
        try{} finally {}
        try {} finally{}
        try {}
        finally
        {}
        try {} catch(Exception e) {} 
        try {} catch (Exception e) {}
        try {} catch (RuntimeException | Error e) {}
        try
        {} catch (Exception e)
        {}
        try(java.io.StringReader r = null) {}
        try (java.io.StringReader r = null) {}
        synchronized(this) {}
        synchronized (this) {}
        synchronized
            (this) {}
        Runnable x = () -> {};
    }
    void va(int...a) {}
    void vb(int... a) {}
    void vc(int ...a) {}
    void vd(int ... a) {}
    void ve(String... a) {}
    void vf(String...
            a) {}
    void pat(Object o) {
        switch (o) {
            case String s when(s.length() > 1) -> {}
            case Integer i when i > 1 -> {}
            case Long l when
                l > 1 -> {}
            default -> {}
        }
    }
    void arrows(int n) {
        switch (n) {
            case 7->{}
            case 8 ->{}
            case 9-> foo(1);
            default->{}
        }
        int r = switch (n) { case 1->1; default->2; };
    }
    void foo(int... a) {}
}
