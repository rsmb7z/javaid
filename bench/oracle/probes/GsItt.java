package bench.oracle.probes;

public class GsItt {
    // a line comment with a tab:	end
    /* a block comment x
       with	a tab and \u0009 and \012 */
    /** javadoc \u0020 text */
    String s1 = "tab:	here";
    String s2 = "nbsp: here";
    String s3 = "escaped \t tab";
    String s4 = "unicode \u0009 tab";
    String s5 = "unicode \u000A newline";
    String s6 = "octal \012 newline";
    String s7 = "octal \11 tab";
    String s8 = "octal \0 nul";
    String s9 = "quote \u0027 and \u005C\u005C and \u005c\u005c";
    String s10 = "space \u0020";
    String s11 = "ok \u0041 ok";
    String s12 = "\\u0009 is not an escape here";
    String s13 = "\\012 neither";
    String s14 = "em space: here";
    String s15 = "form feed:here vt:\u000Bhere";
    char c1 = '	';
    char c2 = '\u0009';
    char c3 = '\011';
    char c4 = '\'';
    char c5 = ' ';
    char c6 = '\u0027';
    char c7 = '\134';
    char c8 = ' ';
    String t1 = """
        text	block with tab
        and \u0009 and \012
          and nbsp: here
        """;
    String t2 = """
        plain
        """;
    String t3 = """
        escaped \t \n \s
        """;
    String t4 = """	
        tab after opener
        """;
    int i1 = 1234;
    int i2 = 0x1F;
    int i3 = 017;
    int i4 = 0b101;
    int i5 = 1_000;
    long l1 = 123L;
    long l2 = 0xFFl;
    float f1 = 1.5f;
    float f2 = 2F;
    double d1 = 3.14;
    double d2 = 1e10;
    double d3 = 4d;
    double d4 = 0x1.0p3;
    float f3 = 0x1p3f;
    void aXyz(int bXyz) {
        int ab1 = 1;
        String xyzString = "x";
        Object lambda = (java.util.function.Function<Integer, Integer>) abc -> abc;
        label:
        for (;;) {
            break label;
        }
    }

    @Deprecated(since = "tab	value")
    void annotated() {
    }
}
