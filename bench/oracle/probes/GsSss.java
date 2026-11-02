package   bench.oracle.probes;

import  java.util.List;
import java.util.Map;   // trailing  comment

public   class GsSss {
    int a  = 1;
    int b =  2;
    int  c = 3;
    int d = 4;   // comment with   spaces
    int e = 5;	// tab before comment
    int f	= 6;
    int g = 7;  /* block */  int h = 8;
    int i = 9; /* block */ int j = 10;
    /* lead */  int k = 11;
    /* lead */ int l = 12;
        int m = 13;
    int n  =  14;
    String s = "a  b   c";
    String t = "x"  +  "y";
    char c1 = ' ',  c2 = 'x';
    String u = """
        two  spaces   inside  a block
        """;
    String v = """
        text   """;
    String w = """
        text """;
    /**
     * Javadoc  with   spaces.
     */
    void f1(int  x,  int y) {
        f1( x , y );
        List<String>  ls = null;
        Map<String,  Integer> mp = null;
        if (x  > 1) {
        }
        for (int q = 0;  q < 1;   q++) {
        }
        foo(x)  ;
        x = x  +  1;
        int[]  arr  = {1,  2};
        Object o = (String)  null;
        label:  for (;;) {
            break  label;
        }
    }

    @Deprecated  @SuppressWarnings("x")
    void f2() {
    }

    @Deprecated   void f3() {
    }

    void f4()   {
    }

    void f5() {   }

    void f6() { return;   }

   int sh = 1;
  int sh2  =  2;
 int sh3 = 3;
int sh4  = 4;
    int e1,   e2;
    /* a   */ int e3;
    /*   */
    int e4; /*   */
    int e5; /* x   */
    // c  ;
}
