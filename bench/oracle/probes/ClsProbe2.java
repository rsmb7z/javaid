package a.b;
public class ClsProbe2 extends X {
  Object m() {
    new B() { }.go();
    foo(new C() { });
    Object o = new D() { }.go();
    int x = new E() { }.go() + 1;
    return new G() { };
  }
  private static class B { }
  private static class C { }
  private static class D { }
  private static class E { }
  private static class G { }
  private static class H extends B { }
  private static class I { private I() { } }
  private class J { }
  private class K implements Runnable { public void run() { new K() { }; } }
  private static class L { }
  private static class M extends L { }
  private static class N extends a.b.L { }
}
class Assigns {
  Object o;
  void m(boolean x) {
    o = new A() { };
    o = (Object) new B() { };
    o = x ? new C() { } : null;
    o = foo(new D() { });
    foo(o = new E() { });
    Object p = o = new G() { };
    o = new H() { }.toString();
    o += "" + new I() { };
    Runnable r = () -> { o = new J() { }; };
    Runnable r2 = () -> { new K() { }; };
    Object q = new L() { };
  }
  private static class A { }
  private static class B { }
  private static class C { }
  private static class D { }
  private static class E { }
  private static class G { }
  private static class H { }
  private static class I { }
  private static class J { }
  private static class K { }
  private static class L { }
}
class Qualified {
  void m() {
    Object a = new P<Integer>() { }.new Q();
    Object b = new P<Integer>().new Q();
  }
  private static class P<T> { private P() { } class Q { } }
}
