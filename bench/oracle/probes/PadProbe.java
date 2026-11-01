package ok.pkg;

public class PadProbe {
    @interface A { int v( ) default 1; int w() default 2; }
    @A( 1 ) int annotated;
    enum E { X( 1 ), Y(2); E( int a ) {} }
    record R( int a ) {}
    PadProbe( int a ) { this( a, 1 ); }
    PadProbe( int a, int b ) { super( ); }
    void m( int a ) {}
    void n( ) {}
    void body( Object o, int[] xs, boolean x ) throws Exception {
        foo( 1 );
        foo( );
        int a = ( 1 + 2 );
        int b = ( int ) o.hashCode();
        int c = ( x ) ? 1 : 2;
        int d = x ? ( 1 ) : ( 2 );
        a = ( ( 1 ) );
        new Object( );
        new PadProbe( 1 );
        o.toString( );
        ( o ).toString();
        if ( x ) {}
        while ( x ) {}
        do {} while ( x );
        for ( int i = 0; i < 1; i++ ) {}
        for ( int i : xs ) {}
        for ( ;; ) { break; }
        switch ( a ) { default: }
        synchronized ( this ) {}
        try ( java.io.StringReader r = null ) {}
        try ( java.io.StringReader r = null; ) {}
        try { } catch ( Exception e ) {}
        Runnable run = ( ) -> {};
        java.util.function.IntBinaryOperator f = ( p, q ) -> p;
        if (o instanceof R( int z )) {}
        if ( o instanceof R( int z ) ) {}
        foo(
            1
        );
        foo( // c
            1 );
        foo(1 );
        foo( 1);
    }
    void foo( int... a ) {}
}
