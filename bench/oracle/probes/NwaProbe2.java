import java. util.List;
import java.util. *;
import static java.lang. Math.abs;
import java.
    util.Map;
public class NwaProbe2<T extends Comparable<? super T>> {
    @interface Ann { int[] v() default { 1 }; String[] w() default {"a"}; Ann2 z() default @ Ann2; }
    @interface Ann2 { }
    @Ann(v = { 1, 2 }) int f1;
    @Ann(v = {1}) int f2;
    @Ann(v = {
        1}) int f3;
    java.util. List<String> f4;
    java.util.List <String> f5;
    int @Deprecated [] f6;
    int[] @Deprecated [] f7;
    void m(Object o, int x, String[] arr) throws Exception {
        Object a = (String) o;
        Object b = (String)o;
        Object c = (String)
            o;
        Object d = (int[]) o;
        Object e = (List<String>) o;
        Object e2 = (Runnable & java.io.Serializable) () -> { };
        Object e3 = (Runnable & java.io.Serializable)() -> { };
        if (x > 0) { }
        if(x > 0) { }
        for (int i = 0; i < 2; i++) { }
        for(int i = 0; i < 2; i++) { }
        while (x > 0) { x--; }
        while(x > 0) { x--; }
        do { x--; } while (x > 0);
        do { x--; } while(x > 0);
        synchronized (this) { }
        synchronized(this) { }
        try { } catch (Exception ex) { }
        try { } catch(Exception ex) { }
        Runnable r = this :: toString;
        Runnable r2 = this::toString;
        java.util.function.Function<Integer, int[]> f = int[] ::new;
        java.util.function.Function<Integer, int[]> g = int [] ::new;
        java.util.function.Function<Integer, String[]> h = String [] ::new;
        int y = arr [0].length;
        int z = arr.length [0];
        int w = ( arr ) [0].length;
        int[] v = new int [] {1}  [0] == 0 ? null : null;
        Object u = new int[] {1, 2} [0];
        int t = - 1 + + 2 - ~ 3;
        boolean s = ! (x > 0);
        x = - -x;
        x = + +x;
        x = ++ x;
        x = x ++ + ++ x;
        x = x -- - -- x;
        Object q = o. hashCode();
        Object p = o .hashCode();
        Object n = this. f1;
        Object n2 = NwaProbe2. this;
        Object n3 = Map. Entry.class;
        Object n4 = o.
            hashCode();
        Object n5 = - /* c */ x;
        Object n6 = ! /* c */ true;
        Object n7 = - // c
            x;
        String[][] aa = new String[2] [3];
        String[][] ab = new String[2][];
        String[] [] ac = null;
        String ad [] [] = null;
        String[] ae [] = null;
        T[] af = (T[]) new Comparable [1];
    }
    void n(List<String []> a, Map<String, int []> b) { }
    String [] n2() { return null; }
    String n3() [] { return null; }
    String n4()
        [] { return null; }
    abstract static class Inner { abstract int n5 ()   [ ]  ; }
}
