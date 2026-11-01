package probes;

public class UcProbe {
    String a = "plain";
    String b = "caf\u00e9";
    String c = "\u00e9";
    String d = "\u00e9\u00e8";
    String e = "tab\u0009x";
    String f = "ctl\u0001\u200b";
    String g = "np\u00a0";
    String h = "\\\u00e9";
    String i = "\\\\u00e9";
    String j = "\uuu00e9";
    char k = '\u0041';
    char l = '\u0000';
    char m = '\n';
    String n = "\u00e9"; // trailing comment
    String o = "\u00e9"; /* trailing block */
    String p = "\u00e9" /* mid */ + "x";
    String q = "\u00e9"
        // own line comment
        + "x";
    String r = "\u00e9" + "\u00e8"; // two on a line
    String s = "\u00e9"; String t = "\u00e8"; // two stmts
    String u = "\"\u00e9\"";
    String v = "\r\n\u000a\u000d";
    String w = "\ufeff";
    String x = "\uffff\u2028";
    String y = "\u0600";
    String z = """
        caf\u00e9
        """;
    String z2 = """
        caf\u00e9
        """; // text block tail
    String z3 = """
        \u00e9
        """;
    String z4 = """
        plain
        """;
    String z5 = """
        \u0009\u200b
        """;
    // comment with \u00e9 inside
    /* block \u00e9 */
    /** doc \u00e9 */
    String all = "\u00e9\"\n\\'";
    String all2 = "\u00e9\t\u00e8";
    String all3 = "\u00e9\u00e8\\";
    String t1 = "a\u00e9"; /* c1 */ /* c2 */
    String t2 = "\u00e9" /* c1 */
        ;
}
