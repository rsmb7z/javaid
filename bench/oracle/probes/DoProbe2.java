package p;

import java.util.function.Supplier;

public class DoProbe2 {
    private static int s1 = 1;
    public static int s2 = s1;
    private int i1;
    public int i2 = i1 + later();
    public int i3 = new Later().x;
    public Supplier<Integer> sup = () -> i1;
    private int i4 = i5;
    public int i5, i6 = i4, i7 = 0;
    @Deprecated
    protected int ann;
    /** doc */
    public final int documented = 0;
    private int[] arr[] = new int[1][];
    public int[] arr2 = arr[0];

    DoProbe2() { }

    @Deprecated
    DoProbe2(int a) { }

    static { }
    { }

    int later() { return 0; }
    static class Later { int x; }
    @Deprecated DoProbe2(String s) { }
    <T> DoProbe2(T t, int x) { }
    private int afterMethod;
    static int afterMethodS;
}

interface DoI {
    void m();
    int K = 1;
    static void s() { }
    int L = 2;
}

enum DoE {
    A(1), B(2);
    int f;
    static int g;
    private int h;
    DoE(int x) { }
    static int sf;
}

record DoR(int a) {
    static int x;
    int y;
    DoR { }
    void m() { }
    DoR(int a, int b) { this(a); }
}

class DoAccess {
  int count;
  private static final Object duh = new Object();
  Object mode;
  public int pub;
  void m() { }
  private static int afterM;
  int pkgAfter;
}
