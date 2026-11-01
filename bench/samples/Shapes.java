package com.example.Shapes;

import java.io.IOException;
import java.io.Reader;
import java.util.function.BiFunction;
import java.util.function.Function;

/** Every declaration shape the naming checks care about, in one file. Not meant to compile. */
public class Shapes<T, UU> extends Base<T> implements Iface<T> {
    private int instanceField;
    private static int staticField;
    private static final int CONSTANT = 1;
    public static final int bad_constant = 2;
    final int finalInstance = 3;
    int a, b[], c = 1;
    static final long serialVersionUID = 1L;

    Shapes() {}

    <X, Y> Shapes(X x, Y y) {}

    <M> void generic(M m, final int fp, int... va) {}

    void locals(int paramOne, Object o, List<String> list, int[] arr) {
        int localOne = 1;
        final int localFinal = 2;
        int a1, b1 = 2;
        for (int i = 0, j = 1; i < 3; i++) {}
        for (final int k : arr) {}
        for (String s : list) {}
        try (Reader r = open(); final Reader r2 = open()) {
        } catch (IOException | RuntimeException ex) {
        } catch (Exception e) {
        }
        Runnable r1 = () -> {};
        Function<Integer, Integer> f1 = x1 -> x1;
        BiFunction<Integer, Integer, Integer> f2 = (p, q) -> p;
        BiFunction<Integer, Integer, Integer> f3 = (Integer p2, final Integer q2) -> p2;
        if (o instanceof String str) {}
        if (o instanceof final String fstr) {}
        if (o instanceof Point(int px, var py)) {}
        switch (o) {
            case Integer ii when ii > 1 -> {}
            case Point(int x1, int y1) -> {}
            default -> {}
        }
    }

    interface Iface<I> {
        int IFACE_CONST = 1;
        int bad = 2;

        <Z> void m(Z z);
    }

    @interface Ann {
        int value() default 1;

        String Bad_Name = "x";
    }

    enum Color {
        RED,
        GREEN {
            void x() {}
        };
        int ev;
    }

    record Rec<R>(int compOne, String CompTwo) {
        Rec {}

        static int sv;
    }

    class Inner<I2> {}
}
