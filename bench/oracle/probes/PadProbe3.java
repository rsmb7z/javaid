package ok.pkg;

public class PadProbe3 {
    void a() {
        foo( 
            1);
        foo(	1);
        foo(1
            );
        foo(1	);
        foo(1 
            );
        for (int i = 0; i < 2; ) {}
        for (;; i++ ) {}
        for ( ; ; ) {}
        for (int i = 0; ; i++) {}
        try (java.io.StringReader r = null; java.io.StringReader s = null; ) {}
        try (java.io.StringReader r = null; java.io.StringReader s = null ) {}
        a( /* c */ );
        a(/* c */ 1 /* d */);
        a(1);
        a(1 , 2);
        a(1);
    }
    void foo(int... a) {}
}
