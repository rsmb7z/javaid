package bench.oracle.probes;

public class GsCaseColon2 {
    void f(int x) {
        switch (x) {
            case 1 /* c */:
                break;
            case 2 /* c */ :
                break;
            case 3/* c */:
                break;
            case 4/* c */ :
                break;
            case 5 /* a
                b */:
                break;
            case 6 /* c */ /* d */:
                break;
            case 7 /* c */  :
                break;
            case 8/**/:
                break;
            case 9 /**/ :
                break;
            case /* c */ 10:
                break;
            case /* c */ 11 :
                break;
            case 12 /* c */ , 13:
                break;
            case 14, /* c */ 15 /* c */:
                break;
            case 16 /* c */ /* d */ :
                break;
            case 17 /* c */ // line
                :
                break;
            case 18 // line
                /* c */:
                break;
            default /* c */:
                break;
        }
        switch (x) {
            default /* c */ :
                break;
        }
        switch (x) {
            default/* c */:
                break;
        }
        switch (x) {
            default/* c */ :
                break;
        }
        switch (x) {
            default /* a
               b */:
                break;
        }
        switch (x) {
            default /* a
               b */ :
                break;
        }
        switch (x) {
            default /* c */ /* d */:
                break;
        }
        switch (x) {
            default // c
                /* d */:
                break;
        }
        switch (x) {
            default:/* c */
                break;
            case 1:/* c */
                break;
        }
    }
}
