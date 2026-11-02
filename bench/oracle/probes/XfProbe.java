package p;

import java.util.List;

/**
 * Class summary without period
 */
public class XfProbe {
  private int count;
  private String name;

  // TODO: fix this
  @Test
  public void Test_One(int count, final String name) {
    int a = 42; // trailing
    if (a > 1) a++;
    for (int i = 0; i < 99; i++) {}
    while (a>3) { a--; }
    { int nested = 7; }
    if (a==1) { /* comment only */ }
  }

  @org.junit.Test void Bad_Two(int x) { int y = 100; }

  /** Javadoc without period */
  public void Bad_Three(int x) {
    Runnable r = () -> { };
    Runnable q = () -> { /* c */ };
    int z = 5 +6;
    switch (z) { case 1: z = 11; break; default: break; }
  }

  void Bad_Four(int a, int b) { return; }
  void Bad_Five() { }
  void Bad_Six() {
    // only a comment
  }
  void Bad_Seven() { int k = 12; }
  static class Inner {
    void Bad_Inner(int q) { int w = 13; }
    @MyTest(1) void Bad_Annot() { int w = 14; }
  }
  interface I { void Bad_Iface(int p); }
  enum E { A(15), B(16); E(int v) { } void Bad_Enum() { } }
  record Rec(int count) { void Bad_Rec() { int h = 17; } }
  void Bad_Cm(){/* c */}
  void Bad_Cm2(){}
  void Bad_Cm3(){ // c
  }
  class Z {/* c */}
  class Z2 {}
}
