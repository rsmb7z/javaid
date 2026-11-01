public class EmptyProbe {
    int f;;
    ;
    void a() {
        ;
        int x = 1;;
        if (x > 0) ;
        for (;;) ;
        while (x < 0) ;
        do ; while (x < 0);
        for (int i = 0; i < 1; i++) ;
        if (x > 1) { } else ;
        label: ;
        switch (x) { case 1: ; break; default: ; }
        try { ; } finally { ; }
        Runnable r = () -> { ; };
        ;;
    }
    class In { ; }
    enum E { A, B; ; }
    enum F { A; }
    interface I { ; }
    record R(int a) { ; }
    static { ; }
    { ; }
}
