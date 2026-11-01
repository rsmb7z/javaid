package probes;

public class UcTail {
    void m(Object o, int[] arr) {
        foo("\u00e9") // chain before dot
            .bar();
        foo("\u00e9"); // after semi
        foo("\u00e9") // before semi
            ;
        foo("\u00e9", // after comma
            1);
        foo("\u00e9" // before paren
        );
        String s = "\u00e9" // before plus
            + "x";
        String t = // after assign
            "\u00e9";
        String u = "\u00e9" + // after plus
            "x";
        int[] a = {1, "\u00e9".length(), // in init
            2};
        if (o.equals("\u00e9")) { // after brace
        }
        if (o.equals("\u00e9")) // before block
        {
        }
        foo(bar("\u00e9") // nested before paren
            );
        foo(bar("\u00e9"), // nested comma
            2);
        foo("\u00e9").bar("\u00e9") // second call chain
            .baz();
        foo("\u00e9")
            .bar("\u00e9") // chain end
            .baz();
        Object p = new Object() { // anon
            String f = "\u00e9";
        };
        String v = o == null ? "\u00e9" // ternary
            : "y";
        String w = o == null ? "x" : // ternary colon
            "\u00e9";
        return2("\u00e9" /* block before dot */
            ).bar();
        for (String x : new String[] {"\u00e9"}) // for each
            foo(x);
        switch (1) {
            case 1: foo("\u00e9"); // case body
            default:
        }
        label: foo("\u00e9"); // labeled
        throw new RuntimeException("\u00e9"); // throw
    }
    void foo(Object... x) {}
    Object bar(Object... x) { return null; }
    Object return2(Object... x) { return null; }
}
