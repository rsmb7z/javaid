package probes;

public class UcOps {
    void m() {
        Object a1 = "\u00e9" // c
            .toString();
        Object a2 = "\u00e9" // c
            + "x";
        Object a3 = 1 - "\u00e9".length() // c
            - 2;
        Object a4 = "\u00e9".length() // c
            * 2;
        Object a5 = "\u00e9".isEmpty() // c
            && true;
        Object a6 = "\u00e9".isEmpty() // c
            || true;
        Object a7 = "\u00e9" // c
            == "x";
        Object a8 = "\u00e9".isEmpty() // c
            ? 1 : 2;
        Object a9 = true ? "\u00e9" // c
            : "y";
        Object a10 = "\u00e9" // c
            instanceof String;
        Object a11 = "\u00e9" // c
            .length();
        Object a12 = new String[] {"\u00e9"} // c
            [0];
        String a13 = ""; a13 // c
            += "\u00e9";
        String a14; a14 // c
            = "\u00e9";
        foo("\u00e9" // c
            );
        Object a15 = new String[] {"\u00e9"}[0 // c
            ];
        Object a16 = new String[] {"\u00e9" // c
            };
        foo("\u00e9" // c
            , 1);
        foo("\u00e9") // c
            ;
        Object a17 = (String) "\u00e9" // c
            ;
        Runnable r = () -> foo("\u00e9") // c
            ;
        Object a18 = "\u00e9".length() // c
            > 1;
        Object a19 = "\u00e9".length() // c
            << 1;
        Object a20 = "\u00e9".length() // c
            & 1;
        int a21 = 0; a21 // c
            ++;
        if (foo("\u00e9")) // c
            {}
        if (true) foo("\u00e9"); // c
        else foo();
        foo("\u00e9"); // c
        foo();
    }
    boolean foo(Object... x) { return true; }
}
