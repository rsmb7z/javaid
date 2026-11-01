public class BoolProbe {
    boolean f(boolean a, boolean b, int x) {
        if (a) return true; else return false;
        if (a) { return true; } else { return false; }
        if (a) { return false; } else return true;
        if (a) return true; else { return false; }
        if (a) { return true; }
        if (a) return true;
        if (a) { return x > 0; } else { return false; }
        if (a) { return (true); } else { return false; }
        if (a) { return true; } else if (b) { return false; } else { return true; }
        if (a) { return true; } else { if (b) { return false; } }
        if (a) { } else { return false; }
        if (a) { return true; x++; } else { return false; }
        if (a) { x++; return true; } else { return false; }
        if (a) { return true; } else { return false; x++; }
        if (a) { return true; } else return;
        if (a) return; else return;
        if (a) { return true; } else { throw new RuntimeException(); }
        if (a) { ; return true; } else { return false; }
        if (a) { /* c */ return true; } else { return false; }
        if (a) return true; else if (b) return false; else return true;
        if (a) { return Boolean.TRUE; } else { return Boolean.FALSE; }
        if (a) { return true; } else { return true; }
        if (a) { return false; } else { return false; }
        if (a) return true; else
            return false;
        int r = switch (x) { case 1 -> { if (a) { yield true ? 1 : 0; } else { yield 2; } } default -> 0; };
        boolean t = switch (x) { case 1 -> { if (a) { yield true; } else { yield false; } } default -> false; };
        boolean t2 = switch (x) { case 1: if (a) { yield true; } else { yield false; } default: yield false; };
        return a;
    }
    boolean g(boolean a, boolean b, Boolean c, int x) {
        boolean r = a == true;
        r = true == a;
        r = a != false;
        r = false != a;
        r = a == false;
        r = a && true;
        r = true && a;
        r = a || false;
        r = false || a;
        r = a || true;
        r = a && false;
        r = !true;
        r = !false;
        r = !(true);
        r = !(a && true);
        r = (a == true);
        r = a == (true);
        r = a == ((false));
        r = (a) == (true);
        r = true == true;
        r = true && false;
        r = !true == false;
        r = a ? true : false;
        r = a ? false : true;
        r = a ? true : b;
        r = a ? b : false;
        r = a ? b : c;
        r = true ? a : b;
        r = false ? a : b;
        r = (true) ? a : b;
        r = a ? (true) : (false);
        r = a ? (true) : b;
        r = a ? true : true;
        r = true ? true : true;
        r = true ? true : b;
        r = a ? b ? true : false : c;
        r = a ? (b ? true : false) : c;
        r = a ? b : c ? true : false;
        r = (a ? true : false);
        r = x > 0 ? true : false;
        r = x == 1 == true;
        r = a & true;
        r = a | false;
        r = a ^ true;
        r = a & !true;
        r = Boolean.TRUE == true;
        r = c == true;
        r = c != null == true;
        r = true;
        r = false;
        r = (true);
        foo(true);
        foo(true, false);
        foo(a == true, !false);
        if (a == true) { }
        while (!false) { }
        do { } while (true && a);
        for (; true || a; ) { }
        assert a == true;
        return a || true;
    }
    void foo(boolean... b) { }
    boolean h(boolean a) {
        return a ? true : false;
    }
    boolean i = true == false;
    boolean j = !true;
    boolean k = true ? true : false;
    Runnable l = () -> { boolean m = true != false; };
    boolean n(boolean a) { return !(a == true) && (a != false) || !!true; }
}
