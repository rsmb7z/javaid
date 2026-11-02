package bench.oracle.probes;

public class GsCaseColon {
    void f(int x, String s, Object o) {
        switch (x) {
            case 1:
                break;
            case 2 :
                break;
            case 3  :
                break;
            case 4
                :
                break;
            case 5 /* c */:
                break;
            case 6/* c */ :
                break;
            case 7 // line
                :
                break;
            case (8) :
                break;
            case 9, 10 :
                break;
            case 11,
                12:
                break;
            default :
                break;
        }
        switch (x) {
            default:
                break;
        }
        switch (x) {
            default
                :
                break;
        }
        switch (x) {
            default  :
                break;
        }
        switch (x) {
            default /* c */:
                break;
        }
        switch (x) {
            default/* c */ :
                break;
        }
        switch (x) {
            default // c
            :
                break;
        }
        switch (s) {
            case "a" :
            case "b":
            case "c"	:
                break;
        }
        switch (x) {
            case 1 -> {
            }
            case 2 , 3 -> {
            }
            default -> {
            }
        }
        switch (o) {
            case String t :
                break;
            case Integer i when i > 0 :
                break;
            case Integer i when i > 1:
                break;
            case Long l:
                break;
            default :
                break;
        }
        int y = switch (x) {
            case 1 :
                yield 1;
            default :
                yield 2;
        };
        label :
        for (;;) {
            break label;
        }
        int z = x > 1 ? 1 : 2;
        int w = x > 1 ?
            1 :
            2;
    }

    enum E { A }

    void g(E e) {
        switch (e) {
            case A :
                break;
            case /* c */ A:
                break;
            case A /* c */ :
                break;
            case
                A:
                break;
        }
        switch (e) {
            case A:
            default :
        }
        switch (e) {
            case A: { }
            default
            :
        }
    }
}
