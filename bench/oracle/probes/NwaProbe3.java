package a. b.
    c;
import java.util.List;
public class NwaProbe3 {
    @java.lang.annotation.Target(java.lang.annotation.ElementType.TYPE_USE) @interface A { }
    Object o1 = new @A NwaProbe3 [3];
    Object o2 = new @A NwaProbe3[3] [4];
    Object o3 = new @A int [3];
    Object o4 = new NwaProbe3 @A [3];
    Object o5 = new @A NwaProbe3 [3] [4];
    Object o6 = new NwaProbe3 [3];
    @A NwaProbe3 [] x;
    @A NwaProbe3 y [];
    NwaProbe3 @A [] z;
    Object o7 = new java.lang.@A String [3];
    Object o8 = new java.lang.String [3];
    Object o9 = new @A List<String> [3];
    <C> @A NwaProbe3 [] m1(C c) { return null; }
    <C> @A NwaProbe3[] m2(C c) { return null; }
    <C> @A int [] m3(C c) { return null; }
    <C> @A java.lang.String [] m4(C c) { return null; }
    <C> @A List<String> [] m5(C c) { return null; }
    <C> @A NwaProbe3 [][] m6(C c) { return null; }
    List<@A Object []> f1;
    List<@A Object[]> f2;
    List<@A int []> f3;
    List<@A java.lang.Object []> f4;
    void m(Object o) {
        Object a1 = new @A NwaProbe3
            [3];
        Object a2 = new @A int
            [3];
        Object a3 = new @A NwaProbe3 [3]
            [4];
        Object a4 = new @A @A NwaProbe3 [3];
        Object a5 = new @A(1) NwaProbe3 [3];
        Object a6 = new @A java.lang.@A String [3];
        Object a7 = new @A NwaProbe3 [] { null };
        Object a8 = new @A  NwaProbe3 [3];
        Object a9 = new @A Q [3];
        Object b1 = (@A Object []) o;
        Object b2 = (@A Object[]) o;
        Object b3 = (@A Object []) o;
        Object b4 = new NwaProbe3 [] { null };
    }
    void bounds(List<? extends @  A Object[]> a, List<? extends @ A Object []> b, List<? super @A Object []> c,
            List<@  A Object[]> d, List<@ A Object[] > e) { }
    <T extends @  A Object[]> void bounds2() { }
    <T extends @A Object []> void bounds3() { }
}
