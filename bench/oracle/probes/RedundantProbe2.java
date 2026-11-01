package p;

import java.util.function.BiFunction;

public class RedundantProbe2 {
    void unnamed(Object o) {
        final int _ = 1;
        final int a = 2, _ = 3;
        final int x = 2, y = 3;
        for (final int _ : new int[0]) { }
        for (final int i : new int[0]) { }
        try { } catch (final RuntimeException _) { }
        try { } catch (final RuntimeException e) { }
        BiFunction<Integer, Integer, Integer> f = (final Integer _, final Integer b) -> b;
        BiFunction<Integer, Integer, Integer> g = (Integer _, Integer b) -> b;
        if (o instanceof final String _) { }
        if (o instanceof final String s) { }
        if (o instanceof R(final int _, final int w)) { }
        try (final AutoCloseable _ = null) { } catch (Exception e) { }
    }

    record R(int a, int b) { }

    strictfp void sfp() { }
    static strictfp class S { }
    public strictfp interface SI { }
}
