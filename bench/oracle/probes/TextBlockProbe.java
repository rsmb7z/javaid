package p;

import java.util.List;

@Anno("""
    ann""")
public class TextBlockProbe {
    String ok = """
        fine
        """;
    String ok2 = """
        fine
        """ + "x";
    String badOpen = """ hi
        text
        """;
    String badOpen2 = foo("""
        text
        """);
    String x = foo(1, """
        text
        """);
    String x2 = foo(1,
        """
        text
        """);
    String closeBad = """
        text""";
    String closeBad2 = """
        text   """;
    String closeBad3 = """
        text
        more   """;
    String misaligned = """
        text
      """;
    String misaligned2 = """
        text
          """;
    String lessIndented = """
        text
      less
        """;
    String blankLines = """
        a

        b
        """;
    String ws = """
        a

        b
        """;
    String cont = """
        a \
        b
        """;
    String empty = """
        """;
    String sp = """
    """;
    String esc = """
        a\"""b
        """;
    String twoOnLine = foo("""
        a
        """, """
        b
        """);
    String twoOnLine2 = foo("""
        a
        """,
        """
        b
        """);
    String concat = "a" + """
        b
        """;
    String concat2 = "a"
        + """
        b
        """;
    String cond = true ? """
        a
        """ : """
        b
        """;
    String cond2 = true
        ? """
        a
        """
        : """
        b
        """;

    @Anno(value = """
        v
        """)
    void m(String a, String b) {
    }

    @Anno(value = "x",
        other = """
        v
        """)
    void m2() {
    }

    @Anno("""
        v
        """) void m3() {
    }

    void body() {
        String s = """
            x
            """;
        foo("""
            x
            """);
        foo("a", """
            x
            """);
        foo(
            """
            x
            """);
        foo(bar("""
            x
            """));
        foo(bar(1,
            """
            x
            """));
        foo(bar(1, """
            x
            """));
        String t = """
          x
        """;
        var l = List.of("""
            x
            """, "y");
        Runnable r = () -> foo("""
            a
            """);
        Runnable r2 = () ->
            foo("""
            a
            """);
        switch (s) {
            case """
                a
                """ -> foo();
            default -> { }
        }
        return;
    }

    enum E {
        A("""
            a
            """),
        B(1, """
            b
            """);
        E(String s) { }
        E(int i, String s) { }
    }

    record R(String a) {
        R {
            a = """
                x
                """;
        }
        static final String K = """
            k
            """;
    }

    interface I {
        String S = """
            s
            """;
        default String d() {
            return """
                d
                """;
        }
    }

    Object anon = new Object() {
        String f = """
            f
            """;
        public String toString() {
            return foo("""
                a
                """);
        }
    };
		String tabs = """
				tabbed
				""";
		String tabs2 = foo(1, """
				x
				""" );
		String tabs3 = """
				x
			""";
    String uni = "é€" + """
        é
        """;
    String é = """
        é
       """;
    String weird = """
        z
        """.trim();
    String weird2 = """
        z
        """
        .trim();
    String weird3 = foo(
        """
        z
        """)
        .trim();
    int i = foo(x -> """
        z
        """);
    int j = (int) foo(x, """
        z
        """);
    Object arr = new String[] {"""
        a
        """, """
        b
        """};
    Object arr2 = new String[] {
        """
        a
        """,
        """
        b
        """};
    void gen() {
        String q = this.<String>foo("""
            a
            """);
        String q2 = new Foo<>("""
            a
            """);
        String q3 = new Foo<>(
            """
            a
            """);
        assert x : """
            a
            """;
        if (x.equals("""
            a
            """)) { }
        for (String z : List.of("""
            a
            """)) { }
        throw new X("""
            a
            """);
    }

    String g1 =
        """
        fine
        """;
    String g2 =
        """
        fine
        """.trim();
    String g3 = foo(
        """
        fine
        """,
        """
        fine2
        """);
    String g4 = foo(1,
        """
        fine
        """);
    String g5 = foo(
        bar(
            """
            fine
            """));
    String g6 = foo(1, 2,
        """
        a
          deeper
        """);
    String g7 = foo(
        """
        a
        """, 3);
    String g8 = foo(
        1,
        """
        a
        """);
    String g9 = "x"
        + """
        a
        """
        + """
        b
        """;
    String g10 =
        """
        a
    less
        """;
    String g11 =
        """
        a
          b
        """ + """
        c
        """;
    @Anno(
        """
        v
        """)
    void ma() {
    }
    @Anno(a = 1,
        b = """
        v
        """)
    void mb() {
    }
		String tg = foo(1,
				"""
				x
				""");
		String tg2 =
				"""
				  x
				""";
    void lam() {
        Runnable r =
            () -> foo(
                """
                a
                """);
        foo(x ->
            """
            a
            """);
        foo(x -> y,
            """
            a
            """);
        var v = new Foo(
            """
            a
            """) {
        };
        var w = new Foo(1, """
            a
            """) {
            void q() { }
        };
    }
}
