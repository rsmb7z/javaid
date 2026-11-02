package a.b;
  import java.util.List;
import java.util.Map;
 import static java.lang.Math.max;
import java.util.function.
    Function;
import java.util.function.Supplier
    ;

   public class Indentation1<T>
  extends Object
      implements Runnable,
    java.io.Serializable {

  private int a;
    private int b = 1,
        c = 2;
 protected
  String s;
      static final int[] ARR = {
    1, 2,
        3
  };
  @Deprecated
    int annotated;
    @SuppressWarnings(
      "unchecked")
  List<String> list;
  @SuppressWarnings({
        "a",
    "b"
  })
  int multi;
@Override
  public void run() {
      a = 1;
   b = 2;
    if (a > 0) {
        b = 3;
    }
    else {
          b = 4;
    }
    if (a > 1)
      b = 5;
    else
  b = 6;
    if (a > 2) { b = 7; } else if (a > 3) {
b = 8;
      } else {
        b = 9; }
    for (int i = 0;
            i < 3;
      i++) {
    a++;
      }
    for (int i = 0; i < 3; i++)
    a++;
    for (String x :
        list) {
    a++;
  }
    while (a < 10)
      a++;
    while (a < 10
        && b < 10) {
      a++; }
    do {
          a++;
    } while (a < 5);
    do
      a++;
    while (a < 5);
      do { a++; }
      while (a < 4);
    synchronized (this) {
        a++;
    }
      synchronized (this)
    {
      a++;
    }
    try {
      a++;
    } catch (RuntimeException e) {
        a++;
    }
      catch (Error e) {
  a++;
    } finally {
        a++;
    }
    try
    {
      a++;
    }
    finally
    {
    a++;
    }
    label:
        for (;;) {
      break label;
    }
  lab2: while (true) {
        break lab2;
    }
    {
        a++;
    }
      {
      a++;
        }
    int[] arr = new int[] {
        1, 2
    };
    int[][] arr2 = {
        {1, 2},
      {3,
         4},
            {5}
    };
    int[] arr3 = new int[]
    {
      1, 2
    };
    int[] arr4 =
    {
      1, 2
    };
    String t = "a"
      + "b"
    + "c"
          + "d";
    foo(1,
      2,
        3);
    foo(
      1,
          2);
    foo(1, 2
    );
    foo(1, 2
        );
      foo(1,
    2);
    this.<String>bar(
        "x");
    x.y.z(
      1);
    new Indentation1<String>()
      .foo(1)
        .foo(2)
    .foo(3);
    new Indentation1<String>() {
      public void run() {
          a++;
      }
  };
    Runnable r = new Runnable() {
    @Override
        public void run() {
      a++;
        }
      };
    Object o = new Object() {
        int q;
          };
    return;
  }

  void foo(int... x) {}
  <U> void bar(U u) {}

  static {
      int z = 1;
  }

    static
  {
  int y;
    }

  {
        int w = 2;
  }
   {
     int w2 = 2;
  }

 void m1(
    int a,
      int b)
         throws Exception,
    RuntimeException {
  }

  void m2(int a,
      int b)
      throws Exception {
  }

  void m3(int a, int b
  ) {
  }

  void m4(
  int a, int b
      ) {
  }

   void m5()
        throws Exception
      {
  }

  void m6() throws
      Exception {
  }

  public
    static
  void m7() {
  }

  abstract static class Abs {
    abstract void ab();
      abstract void ab2(
    int x);
    public
    abstract
        void ab3();
  }

    interface I {
      void i1();
  default void i2() {
        }
   int X = 3;
  }

  enum E {
    A, B,
        C {
      void f() {}
        },
      D(1) {
    },
  E2;
    E() {}
    E(int x) {}
    void f() {}
  }

   @interface Ann {
      int value() default 1;
    String[] names() default {
    "a", "b"
        };
  }

  @interface
      Ann2 {
  }

  record Rec(int a,
      int b) {
        Rec {
      a++;
    }
    int sum() {
          return a + b;
    }
  }

  record Rec2(
    int a) implements Runnable {
    public void run() {}
  }

      class Inner {
    }

  class Inner2
  {
  }
    class
  Inner3 {
  }

  void generics(Map<String,
      List<String>> m,
    Function<String,
          Integer> f) {
  }

  int ret() {
    return
    a;
  }

  int ret2() {
    return a
        + b
    + c;
  }

  int ret3() {
    return foo2(1,
      2);
  }

  int foo2(int a, int b) { return 1; }

  void thr() {
    throw
      new RuntimeException(
    "x");
  }

  void thr2() {
    throw new RuntimeException("a"
        + "b");
  }

  void asrt() {
    assert a > 0
      : "msg";
    assert
        a > 0;
  }

  void ternary() {
    int x = a > 0
        ? 1
      : 2;
    int y = a > 0 ?
      1 :
    2;
    Object z = a > 0
      ? new Object()
        : new Object() {
        };
  }

  void locals() {
    final int
        q = 1;
    final
    int r = 2;
    List<String>
      l1 = null;
    @SuppressWarnings("x")
    int ann = 1;
      @SuppressWarnings("x") int ann2 = 2;
    @SuppressWarnings("x")
      @Deprecated
    int ann3 = 3;
  }
}
