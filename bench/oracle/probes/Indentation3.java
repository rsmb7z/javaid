package a.b;

import java.io.*;
import java.util.*;

public class Indentation3 {

  int sw(int a) {
    switch (a) {
    case 1:
      return 1;
      case 2:
    case 3:
        return 2;
  default:
      return 3;
    }
  }

  void sw2(int a) {
      switch (a) {
        case 1: foo(); break;
        case 2: {
            foo();
          }
        case 3:
        {
          foo();
        }
        default:
          foo();
      }
    switch (a)
    {
      case 1:
        foo();
    }
    switch (a) { case 1: foo(); }
    switch (a) {
      case 1 ->
        foo();
      case 2 -> {
          foo();
      }
      case 3 ->
      foo();
      case 4 ->
            throw new RuntimeException();
    default -> foo();
    }
  }

  int sw3(Object o) {
    int r = switch (o) {
      case Integer i when i > 0 -> 1;
        case String s -> {
        yield 2;
          }
      case Long l ->
        3;
      default -> {
      yield
        4;
      }
    };
    int q =
      switch (r) {
        case 1 -> 1;
        default -> 2;
      };
    int z = switch (r) {
    case 1:
      yield 1;
      default:
        yield
      2;
    };
    return switch (r) {
        case 1 -> 1;
      default -> 2;
    };
  }

  void sw4(Object o) {
    switch (o) {
      case Point(int x,
        int y) -> foo();
      case Point(int x, int y)
          when x > 0 -> foo();
      default -> {}
    }
    if (o instanceof Point(int x,
            int y)) {
      foo();
    }
    if (o instanceof String s
      && s.isEmpty()) {
      foo();
    }
  }

  record Point(int x, int y) {}

  void tr() throws Exception {
    try (InputStream in = new FileInputStream("a");
        OutputStream out = new FileOutputStream("b")) {
      foo();
    }
    try (
      InputStream in = new FileInputStream("a");
          OutputStream out = new FileOutputStream("b")
    ) {
      foo();
    }
    try (InputStream in =
        new FileInputStream("a")) {
      foo();
    }
    try (InputStream in = new FileInputStream(
        "a");
      InputStream in2 = new FileInputStream("b")
        ) {
      foo();
    } catch (IOException
        | RuntimeException e) {
      foo();
    } catch (
      Exception e) {
      foo();
    } catch (final @SuppressWarnings("x") Error e) {
      foo();
    }
    try {
      foo();
    } catch (Exception e) { foo();
    } finally { foo();
    }
    try { foo(); } catch (Exception e) {
      foo(); }
  }

  void foo() {}

  class Gen<T extends Comparable<T>
      & Serializable> {
    <U extends Number
        & Comparable<U>> U f(U u) {
      return u;
    }
  }

  enum Op implements Runnable {
    PLUS("+") {
      public void run() {
          foo();
      }
    },
    MINUS("-") {
    public void run() {}
      },
    TIMES("*")
    {
      public void run() {}
    };
    private final String s;
    Op(String s) {
      this.s = s;
    }
  }

  enum Lam {
    A(() -> {
      foo();
        }),
    B(x ->
      foo());
    Lam(Runnable r) {}
  }

  public sealed interface Shape permits Circle, Square {}
  final class Circle implements Shape {}
  non-sealed class Square implements Shape {}

  @FunctionalInterface
  public
  interface FI {
    void a();
  }

	void tabs() {
		int a = 1;
	  int b = 2;
  	int c = 3;
		if (a > 0
			&& b > 0) {
			a++;
		  }
		foo(1,
			2,
		  3);
		String s = "ü"
			+ "𝄞"
		+ "x";
	}

  void unicode() {
    String s = "𝄞𝄞"; int x = 1,
      y = 2;
    String t = "😀" + foo2(1,
      2);
  }

  int foo2(int a, int b) { return a; }

  // comments between tokens
  void cm() {
    foo(1, /* c */
      2);
    int x = /* c
      d */ 3
      + 4;
    // comment
      foo();
  }

  void lone() {
    int a =
      1; int b =
    2;
    foo(); foo(
    );
    if (a > 0) { foo(); }
    for (;;) { break; }
    ;
    ;
  }

  Object init = new Object() {
    {
      foo();
    }
    static int x;
      int y;
  };

  static int[] arr = {1, 2,
    3};
  static int[] arr2 = {
      1, 2,
  3};
  static int[] arr3 = { 1,
        2 };
  static int[][] arr4 = { {1}, {
      2}};
  static int[][] arr5 = {
    {
      1
    },
        {
    2
      }
  };
  static Object[] arr6 = new Object[] { new Object() {
      },
    new Object() };
}
