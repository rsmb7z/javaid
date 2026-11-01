package a.b;;
import java.util.List;;
;
public class EmptyProbe2 {
    void a(int x) {
        switch (x) { case 1: int y = 2; ; break; }
        int r = switch (x) { case 1 -> { ; yield 2; } default -> 3; };
        switch (x) { case 1 -> { ; } default -> { } }
        if (x > 0) ; else ;
        for (int i : new int[0]) ;
        for (int i = 0;; i++) ;
        try (java.io.StringReader s = new java.io.StringReader("");) { } catch (Exception e) { ; }
        synchronized (this) { ; }
        outer: for (;;) { break outer; }
    }
    enum G { A; ; ; }
    enum H { ; }
    enum J { A, B, ; }
    enum K { A, B, ;; }
    @interface Ann { ; int v(); ; }
    record Q(int a) { ; Q { ; } }
    interface Z { void m();; }
    EmptyProbe2() { ; }
    int g() { return 1;; }
};
;
