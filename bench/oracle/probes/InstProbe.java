package a.b;

import java.util.Date;
import java.util.*;
import com.x.Thing;
import com.y.Other;
import static java.lang.Math.abs;

public class InstProbe {
    Object a = new Boolean(true);
    Object b = new java.lang.Boolean(true);
    Object c = new Integer(1);
    Object d = new String("x");
    Object e = new String();
    Object f = new Date();
    Object g = new java.util.Date();
    Object h = new Local();
    Object i = new a.b.Local();
    Object j = new Thing();
    Object k = new com.x.Thing();
    Object l = new Other();
    Object m = new Thread();
    Object n = new Thread() { };
    Object o = new Boolean[1];
    Object p = new Integer[] { 1 };
    Object q = Boolean.valueOf(true);
    Object r = new Boolean(true) { };
    Object s = new Outer.Inner();
    Object t = new Local.Nested();
    Object u = new com.x.Thing.Sub();
    Object v = new ArrayList<String>();
    void m() {
        Object w = new Boolean(false);
        class Integer { }
        Object x = new Integer();
        Runnable y = () -> new Boolean(true);
    }
    static class Boolean { }
    static class Local { static class Nested { } }
    static class Date { }
    Object z = new Boolean(true);
    Object aa = new Date();
}
class Other2 {
    Object a = new Boolean(true);
    Object b = new Local();
    Object c = new Date();
}
