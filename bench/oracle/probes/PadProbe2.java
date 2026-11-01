package ok.pkg;

public class PadProbe2 {
    int f;
    int[] arr = { ( 1 ), 2 };
    void body(int a, int b, boolean x, int[] xs, String s) {
        int v1 = ( a );
        int v2 = ( a ) + b;
        int v3 = a + ( b );
        int v4 = ( a ) * ( b );
        boolean v5 = !( x );
        int v6 = -( a );
        int v7 = ( a ) > 0 ? 1 : 2;
        int v8 = x ? 1 : ( a );
        int v9 = ( ( x ) ? 1 : 2 );
        a = ( b );
        a += ( b );
        a = x ? ( b ) : 1;
        foo( ( a ) );
        foo( ( a ), ( b ) );
        return;
    }
    int r1(int a) { return ( a ); }
    int r2(int a) { return ( a ) + 1; }
    void stmt(int a, int[] xs) {
        xs[ ( a ) ] = 1;
        int w = xs[ ( a ) ];
        int z = ( xs ).length;
        int y = ( this ).f;
        String t = ( "a" ).trim();
        int u = ( a )++ ;
        Runnable q = () -> ( foo( 1 ) );
        java.util.function.IntSupplier is = () -> ( a );
        assert ( a > 0 );
        assert a > 0 : ( "m" );
        throw ( new RuntimeException() );
    }
    int sw(int a) { return switch ( a ) { case 1 -> ( 2 ); default -> 3; }; }
    boolean inst(Object o) { return ( o ) instanceof String; }
    boolean cond(boolean a, boolean b) { return ( a ) && ( b ) || ( a ); }
    int cast(Object o) { return ( ( Integer ) o ); }
    void lbl(int a) { while ( a > 0 ) a = ( a ) - 1; }
    void foo(int a) {}
}
