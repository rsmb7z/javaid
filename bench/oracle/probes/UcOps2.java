package probes;

public class UcOps2 {
    void m(String s) {
        Object a = s.concat("x",
            "\u00e9") // rparen line differs from lparen line
            .toString();
        Object b = s.concat(
            "\u00e9") // c
            .toString();
        Object c = "x" +
            "\u00e9" // second operand line, plus follows
            + "y";
        Object d = "\u00e9"
            .concat("z") // chain 2
            .toString();
        Object e = s
            .concat("\u00e9") // chain 3
            .toString();
        Object f = foo("\u00e9",
            2) // c
            + "y";
        Object g = s.length() > 1 ? "\u00e9" // c
            : "n";
        Object h = new String[] {"\u00e9",
            "x"} // c
            [0];
        Object i = - 1 + foo("\u00e9", // c
            - 1);
        Object j = foo("\u00e9" // c
            - 1);
        String k = ""; k += "x"; k // c
            = "\u00e9";
    }
    Object foo(Object... x) { return null; }
}
