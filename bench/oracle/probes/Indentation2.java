package a.b;

import java.util.*;
import java.util.function.*;
import java.util.stream.*;

public class Indentation2 {

  Runnable f1 = () -> {
        System.out.println();
      };
  Runnable f2 = () ->
      System.out.println();
  Runnable f3 =
    () -> {
      System.out.println();
  };
  Function<Integer, Integer> f4 = x ->
  x + 1;
  BiFunction<Integer, Integer, Integer> f5 = (x,
      y) -> x + y;
  BiFunction<Integer, Integer, Integer> f6 = (
        x, y) -> x + y;
  Supplier<Runnable> f7 = () -> () -> {
    System.out.println();
  };

  void streams(List<String> list) {
    list.stream()
      .map(s -> s.trim())
        .filter(s -> !s.isEmpty())
    .collect(Collectors.toList());
    list.stream().map(s -> {
        return s.trim();
      }).forEach(s -> {
    System.out.println(s);
    });
    list.forEach(s ->
      System.out.println(s));
    list.forEach(
      s -> System.out.println(s));
    list.forEach(
          s -> {
      System.out.println(s);
        });
    list.forEach(s -> {
      System.out.println(s);
        }
    );
    list.stream()
        .map(s -> s.length())
        .reduce(0, (a, b) ->
          a + b);
    Collections.sort(list, (a, b) -> {
          return a.compareTo(b);
    });
    Collections.sort(list,
      (a, b) -> {
    return a.compareTo(b);
    });
    run(() -> {
        return 1;
      },
      () -> {
        return 2;
    });
    Object o = (Runnable) () -> {
    };
    Runnable rr = list.isEmpty() ? () -> {
        } : () -> {
      };
  }

  Runnable ret() {
    return () -> {
      System.out.println();
        };
  }

  Function<Integer, Integer> ret2() {
    return x ->
    x + 1;
  }

  Function<Integer, Integer> ret3() {
    return
      x -> x + 1;
  }

  int run(Supplier<Integer> a, Supplier<Integer> b) {
    return a.get() +
      b.get();
  }

  int run(Supplier<Integer> a) { return 1; }

  void chains() {
    StringBuilder sb = new StringBuilder();
    sb.append("a")
      .append("b")
        .append("c");
    sb.append("a").append(
        "b").append("c"
            );
    sb
      .append("x")
      .append("y");
    String s = sb.toString()
        .trim()
      .toLowerCase();
    String s2 = foo(bar(1,
        2),
      baz(3));
    String s3 = foo(
      bar(
        1),
    baz(
          3)
        );
    int v = Math.max(1,
          Math.min(2,
      3));
    this.foo(1,
      2).bar(
        3);
    foo(new Object(),
      new Object() {
        int a;
      });
    foo(new Object() {
        int a;
      },
      new Object() {
    int b;
    });
    foo(1, new int[] {
        1, 2
      });
    foo(1,
        new int[] {
      1, 2
    });
  }

  String foo(Object... a) { return ""; }
  String bar(Object... a) { return ""; }
  String baz(Object... a) { return ""; }

  void news() {
    Object a = new Object();
    Object b =
        new Object();
    Object c =
      new Object() {
    };
    Object d = flag()
        ? new Object()
      : new Object();
    Object e = flag()
        ? flag()
          ? new Object()
        : new Object()
        : new Object();
    Object f = new ArrayList<String>(
        Arrays.asList(
          "a",
      "b"));
    Object g = new HashMap<String,
      String>() {
        {
      put("a", "b");
    }
    };
    List<Object> h = Arrays.asList(
      new Object(),
        new Object());
    Object i = new Object[] {
      new Object(),
        new Object() {
      }
    };
    Object[][] j = new Object[][] {
      {new Object()},
        {
      new Object()
          }
    };
    int[] k = new int[3];
    k[0] =
      1;
    k[
      0] = 2;
    int l = k[
        0];
    Object m = Indentation2
        .class;
    Object n = new Indentation2()
      .new Inner();
    Object p = new Indentation2.Inner(
    );
    throw new RuntimeException(
        new Exception("x",
      new Exception()));
  }

  class Inner {}

  boolean flag() { return true; }

  void conditions(int a, int b) {
    if (a > 0 &&
      b > 0) {
      foo();
    }
    if (a > 0
        && b > 0
    && a > b) {
      foo();
    }
    if (
        a > 0) {
      foo();
    }
    if (a > 0
    ) {
      foo();
    }
    while (
      a > 0) {
      a--;
    }
    for (int i = 0,
        j = 0; i < 1;
          i++, j++) {
      foo();
    }
    for (int i : new int[] {
          1, 2}) {
      foo();
    }
    switch (a
      + b) {
      default:
        foo();
    }
    if (a > 0) foo();
    else if (a > 1)
      foo();
    else
      foo();
    if (a > 0) {
    } else
      if (a > 1) {
      }
    if (a > 0) {
      foo();
    } else if (a > 1) {
      foo();
    }
      else if (a > 2) {
    foo();
    }
  }

  void foo() {}

  @SafeVarargs
    private final <T> void ann(
      T... xs) {
  }

  @Deprecated(since =
    "1", forRemoval = true)
  void ann2() {}

  @SuppressWarnings({"a",
      "b"})
  void ann3() {}

  @SuppressWarnings(value = {
      "a",
    "b"
        })
  void ann4() {}

  @SuppressWarnings(
    {
      "a"
    })
  void ann5() {}

  @interface Ann {
    String[] value() default {};
  }

  @Ann({
    "a",
      "b"})
  @Deprecated
    void ann6() {}

  @Ann(value =
    {"x"})
  void ann7() {}

  void txt() {
    String s = """
        hello
          world
      """;
    String t = """
      a""" + """
        b
      """;
    foo("""
        x
    """,
        "y");
    String u = foo(
        """
      z
    """);
  }
}
