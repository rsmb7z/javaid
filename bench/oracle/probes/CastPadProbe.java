import java.util.*;
public class CastPadProbe<T> {
    void a(Object o, int i, long l) {
        int a1 = (int) l;
        int a2 = ( int) l;
        int a3 = (int ) l;
        int a4 = ( int ) l;
        int a5 = (int)l;
        int a6 = ((int) l);
        int a7 = ( (int) l);
        int a8 = (int) ( l );
        String s1 = (String) o;
        String s2 = ( String) o;
        String s3 = (String ) o;
        String s4 = ( String ) o;
        List<String> s5 = (List<String>) o;
        List<String> s6 = ( List<String>) o;
        List<String> s7 = (List<String> ) o;
        Runnable r1 = (Runnable & java.io.Serializable) () -> { };
        Runnable r2 = ( Runnable & java.io.Serializable) () -> { };
        Runnable r3 = (Runnable & java.io.Serializable ) () -> { };
        int[] ar1 = (int[]) o;
        int[] ar2 = ( int[]) o;
        int[] ar3 = (int[] ) o;
        int[] ar4 = (int [ ]) o;
        String s8 = (@Deprecated String) o;
        String s9 = ( @Deprecated String) o;
        String s10 = (
            String) o;
        String s11 = (String
            ) o;
        String s12 = (	String	) o;
        String s13 = (String) ( String ) o;
        int m1 = ( ( int ) l );
        int m2 = (int) (int) l;
        int m3 = ( int ) ( int ) l;
        Object o2 = (Object) /* c */ o;
        Object o3 = ( /* c */ Object) o;
        Object o4 = (Object /* c */ ) o;
    }
    void b() {
        for (int i = 0; i < 2; ) { }
        for (int i = 0; i < 2;) { }
        for (int i = 0; ; ) { }
        for (;; ) { }
        for (;;) { }
        for (; ; ) { }
        for (int i = 0; i < 2;	) { }
        for (int i = 0; i < 2;
            ) { }
        for (int i = 0; i < 2;  ) { }
        for (int i = 0; i < 2; /* c */ ) { }
        for (int i = 0; i < 2; i++) { }
        for (int i = 0, j = 0; i < 2; ) { }
        for (Object o : new ArrayList<Object>()) { }
        for ( ; ; ) { }
        for (
            ;;) { }
        for (int i = 0;
            i < 2; ) { }
        for (;;i++) { }
        for (; ; i++) { }
    }
    int i;
}
