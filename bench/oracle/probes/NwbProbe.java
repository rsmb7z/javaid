package ok . pkg ;

import java . util . List ;
import java.util.function.IntBinaryOperator;

public class NwbProbe < A , B > {
    enum E { X , Y , Z ; }
    int[] arr = { 1 , 2 , 3 } ;
    int[] arr2 = { 1, 2, 3 };
    int[] arr3 = {
        1
        , 2
    };
    void f(int a , int b) {}
    void g(int a, int b) {}
    void h(int ... xs) {}
    void i(int... xs) {}
    void j(
        int a
        , int b) {}
    void body(List<String> xs , int n) {
        foo(1 , 2);
        foo(1, 2);
        foo(1
            , 2);
        int a = 1 ;
        int b = 2;
        a ++ ;
        a -- ;
        a++;
        a--;
        ++ a;
        -- a;
        a
            ++;
        for (int i = 0 ; i < 2 ; i ++) {}
        for (int i = 0; i < 2; i++) {}
        for ( ; ; ) { break ; }
        for (;;) { break; }
        for (int i = 0; ; i++) {}
        for (int i = 0 ; ; ) {}
        for (; n < 2 ;) {}
        ;
        ;;
        lbl : for (int i = 0; i < 2; i++) { continue lbl ; }
        lbl2: for (int i = 0; i < 2; i++) { continue lbl2; }
        lbl3
            : for (;;) { break lbl3; }
        switch (n) { case 1 : break; case 2: break; default : break; }
        int t = n > 1 ? 1 : 2;
        int u = n > 1 ? 1 : 2;
        for (String s : xs) {}
        for (String s: xs) {}
        Object o = xs . size ();
        Object p = xs.size();
        Object q = xs
            .size();
        IntBinaryOperator m = Math :: max;
        IntBinaryOperator m2 = Math::max;
        IntBinaryOperator m3 = Math
            ::max;
        try (java.io.StringReader r = null ; java.io.StringReader r2 = null ;) {} catch (Exception e) {}
        if (n > 1) ;
        if (n > 1) {} else ;
        while (n > 100) ;
        for (int k = 0; k < 1; k++) ;
        do ; while (n > 100);
        lbl9 : ;
        switch (n) { case 1 : ; break; default : ; }
        { ; }
        return ;
    }
    void foo(int... a) {}
    class Inner {} ;
	void tabbed() { foo(1)
	; }
}
;
