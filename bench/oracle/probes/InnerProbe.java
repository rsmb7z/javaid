import java.util.*;
import java.util.function.*;
public class InnerProbe {
    int f, g, h;
    int[] arr = new int[3];
    void a(int x, int y, boolean b, String s, java.io.BufferedReader r) throws Exception {
        x = y;
        x = y = 1;
        x = (y = 1);
        x += y -= 1;
        int z = x = 2;
        int w = (x = 3) + 1;
        f = g = h = 0;
        x = y + (x = 1);
        arr[x = 1] = 2;
        arr[0] = x = 3;
        foo(x = 1);
        foo((x = 1));
        foo(x += 1, y = 2);
        if ((x = 1) > 0) { }
        if (b = true) { }
        if ((b = false)) { }
        while ((s = r.readLine()) != null) { }
        while (x != (x = y)) { }
        while ((x = y) > 0) { }
        while (b = true) { }
        do { } while ((s = r.readLine()) != null);
        do { } while ((x = y) > 0);
        for (x = 0; x < 2; x = x + 1) { }
        for (x = y = 0; x < 2; x++, y += 1) { }
        for (int i = (x = 0); i < 2; i++) { }
        for (int i = 0; (x = i) < 2; i++) { }
        for (int i = 0; i < 2; i = x = 1) { }
        for (Object o : (s = null) == null ? new ArrayList<Object>() : null) { }
        b = (x = 1) > 0;
        b = x > (y = 1);
        s = b ? (s = "a") : "b";
        s = (b = true) ? "a" : "b";
        return2(x = 1);
        Runnable rr = () -> x2 = 1;
        Function<Integer, Integer> fn = q -> q = 1;
        Function<Integer, Integer> fn2 = q -> (q = 1);
        Supplier<Integer> sp = () -> f = 1;
        Supplier<Integer> sp2 = () -> { return f = 1; };
        switch (x = 1) { case 1: break; }
        switch (x) { case 1: y = 1; }
        int t = switch (x = 1) { default -> 0; };
        assert (x = 1) > 0;
        assert b : (s = "x");
        throw2(new Exception(s = "m"));
        synchronized (s = "l") { }
        s = "a" + (x = 1);
        x = y = z = 0;
        x = -(y = 1);
        x = (int) (y = 1);
        b = !(b = true);
        x = arr[0] = 1;
        x = (arr[0] += 1);
        x = y++;
        x = ++y;
        Object o = (Object) (x = 1);
        int[] ia = { x = 1, y = 2 };
        int[][] ib = { { x = 1 } };
        String s2 = "a" + (s += "b");
        x++;
        x = (y);
        label: x = 1;
        if (b) x = 1; else y = 2;
        if (b) x = y = 1;
        while (b) x = y = 2;
        x <<= y >>= 1;
        x = y >>>= 1;
        x = y &= 1;
        x = y |= 1;
        x = y ^= 1;
        x = y %= 1;
        x = y *= 1;
        x = y /= 1;
        x = y <<= 1;
        x = y >>= 1;
        x = y -= 1;
        x = y += 1;
    }
    int m() { return f = 1; }
    int m2() { return (f = 1); }
    int m3(int a) { return a = 1; }
    void n(Object o) { var v = o instanceof String q ? q : null; }
    int field = f = 1;
    int field2 = (f = 2) + 1;
    { f = g = 1; }
    static int sf = 1;
    static { sf = sf = 2; }
}
