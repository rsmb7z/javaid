package ok.pkg;

import java.util.List;
import java.util.function.Function;

public class OwProbe<T> {
    void ops(int a, int b, int c) {
        int e0a = a *
            b;
        int e0b = a
            * b;
        int e0c = a
            *
            b;
        int e0d = a * b;
        int e0e = a * 
            b;
        int e0f = a * // c
            b;
        int e0g = a * /* c */
            b;
        int e0h = a /* c */ *
            b;
        int e0i = a
            /* c */ * b;
        int e0j = a
		* b;
        int e0k = a * b *
            c;
        int e0l = a *
            b * c;
        int e1a = a /
            b;
        int e1b = a
            / b;
        int e1c = a
            /
            b;
        int e1d = a / b;
        int e1e = a / 
            b;
        int e1f = a / // c
            b;
        int e1g = a / /* c */
            b;
        int e1h = a /* c */ /
            b;
        int e1i = a
            /* c */ / b;
        int e1j = a
		/ b;
        int e1k = a / b /
            c;
        int e1l = a /
            b / c;
        int e2a = a %
            b;
        int e2b = a
            % b;
        int e2c = a
            %
            b;
        int e2d = a % b;
        int e2e = a % 
            b;
        int e2f = a % // c
            b;
        int e2g = a % /* c */
            b;
        int e2h = a /* c */ %
            b;
        int e2i = a
            /* c */ % b;
        int e2j = a
		% b;
        int e2k = a % b %
            c;
        int e2l = a %
            b % c;
        int e3a = a +
            b;
        int e3b = a
            + b;
        int e3c = a
            +
            b;
        int e3d = a + b;
        int e3e = a + 
            b;
        int e3f = a + // c
            b;
        int e3g = a + /* c */
            b;
        int e3h = a /* c */ +
            b;
        int e3i = a
            /* c */ + b;
        int e3j = a
		+ b;
        int e3k = a + b +
            c;
        int e3l = a +
            b + c;
        int e4a = a -
            b;
        int e4b = a
            - b;
        int e4c = a
            -
            b;
        int e4d = a - b;
        int e4e = a - 
            b;
        int e4f = a - // c
            b;
        int e4g = a - /* c */
            b;
        int e4h = a /* c */ -
            b;
        int e4i = a
            /* c */ - b;
        int e4j = a
		- b;
        int e4k = a - b -
            c;
        int e4l = a -
            b - c;
        int e5a = a <<
            b;
        int e5b = a
            << b;
        int e5c = a
            <<
            b;
        int e5d = a << b;
        int e5e = a << 
            b;
        int e5f = a << // c
            b;
        int e5g = a << /* c */
            b;
        int e5h = a /* c */ <<
            b;
        int e5i = a
            /* c */ << b;
        int e5j = a
		<< b;
        int e5k = a << b <<
            c;
        int e5l = a <<
            b << c;
        int e6a = a >>
            b;
        int e6b = a
            >> b;
        int e6c = a
            >>
            b;
        int e6d = a >> b;
        int e6e = a >> 
            b;
        int e6f = a >> // c
            b;
        int e6g = a >> /* c */
            b;
        int e6h = a /* c */ >>
            b;
        int e6i = a
            /* c */ >> b;
        int e6j = a
		>> b;
        int e6k = a >> b >>
            c;
        int e6l = a >>
            b >> c;
        int e7a = a >>>
            b;
        int e7b = a
            >>> b;
        int e7c = a
            >>>
            b;
        int e7d = a >>> b;
        int e7e = a >>> 
            b;
        int e7f = a >>> // c
            b;
        int e7g = a >>> /* c */
            b;
        int e7h = a /* c */ >>>
            b;
        int e7i = a
            /* c */ >>> b;
        int e7j = a
		>>> b;
        int e7k = a >>> b >>>
            c;
        int e7l = a >>>
            b >>> c;
        int e8a = a <
            b;
        int e8b = a
            < b;
        int e8c = a
            <
            b;
        int e8d = a < b;
        int e8e = a < 
            b;
        int e8f = a < // c
            b;
        int e8g = a < /* c */
            b;
        int e8h = a /* c */ <
            b;
        int e8i = a
            /* c */ < b;
        int e8j = a
		< b;
        int e8k = a < b <
            c;
        int e8l = a <
            b < c;
        int e9a = a >
            b;
        int e9b = a
            > b;
        int e9c = a
            >
            b;
        int e9d = a > b;
        int e9e = a > 
            b;
        int e9f = a > // c
            b;
        int e9g = a > /* c */
            b;
        int e9h = a /* c */ >
            b;
        int e9i = a
            /* c */ > b;
        int e9j = a
		> b;
        int e9k = a > b >
            c;
        int e9l = a >
            b > c;
        int e10a = a <=
            b;
        int e10b = a
            <= b;
        int e10c = a
            <=
            b;
        int e10d = a <= b;
        int e10e = a <= 
            b;
        int e10f = a <= // c
            b;
        int e10g = a <= /* c */
            b;
        int e10h = a /* c */ <=
            b;
        int e10i = a
            /* c */ <= b;
        int e10j = a
		<= b;
        int e10k = a <= b <=
            c;
        int e10l = a <=
            b <= c;
        int e11a = a >=
            b;
        int e11b = a
            >= b;
        int e11c = a
            >=
            b;
        int e11d = a >= b;
        int e11e = a >= 
            b;
        int e11f = a >= // c
            b;
        int e11g = a >= /* c */
            b;
        int e11h = a /* c */ >=
            b;
        int e11i = a
            /* c */ >= b;
        int e11j = a
		>= b;
        int e11k = a >= b >=
            c;
        int e11l = a >=
            b >= c;
        int e12a = a ==
            b;
        int e12b = a
            == b;
        int e12c = a
            ==
            b;
        int e12d = a == b;
        int e12e = a == 
            b;
        int e12f = a == // c
            b;
        int e12g = a == /* c */
            b;
        int e12h = a /* c */ ==
            b;
        int e12i = a
            /* c */ == b;
        int e12j = a
		== b;
        int e12k = a == b ==
            c;
        int e12l = a ==
            b == c;
        int e13a = a !=
            b;
        int e13b = a
            != b;
        int e13c = a
            !=
            b;
        int e13d = a != b;
        int e13e = a != 
            b;
        int e13f = a != // c
            b;
        int e13g = a != /* c */
            b;
        int e13h = a /* c */ !=
            b;
        int e13i = a
            /* c */ != b;
        int e13j = a
		!= b;
        int e13k = a != b !=
            c;
        int e13l = a !=
            b != c;
        int e14a = a &
            b;
        int e14b = a
            & b;
        int e14c = a
            &
            b;
        int e14d = a & b;
        int e14e = a & 
            b;
        int e14f = a & // c
            b;
        int e14g = a & /* c */
            b;
        int e14h = a /* c */ &
            b;
        int e14i = a
            /* c */ & b;
        int e14j = a
		& b;
        int e14k = a & b &
            c;
        int e14l = a &
            b & c;
        int e15a = a ^
            b;
        int e15b = a
            ^ b;
        int e15c = a
            ^
            b;
        int e15d = a ^ b;
        int e15e = a ^ 
            b;
        int e15f = a ^ // c
            b;
        int e15g = a ^ /* c */
            b;
        int e15h = a /* c */ ^
            b;
        int e15i = a
            /* c */ ^ b;
        int e15j = a
		^ b;
        int e15k = a ^ b ^
            c;
        int e15l = a ^
            b ^ c;
        int e16a = a |
            b;
        int e16b = a
            | b;
        int e16c = a
            |
            b;
        int e16d = a | b;
        int e16e = a | 
            b;
        int e16f = a | // c
            b;
        int e16g = a | /* c */
            b;
        int e16h = a /* c */ |
            b;
        int e16i = a
            /* c */ | b;
        int e16j = a
		| b;
        int e16k = a | b |
            c;
        int e16l = a |
            b | c;
        int e17a = a &&
            b;
        int e17b = a
            && b;
        int e17c = a
            &&
            b;
        int e17d = a && b;
        int e17e = a && 
            b;
        int e17f = a && // c
            b;
        int e17g = a && /* c */
            b;
        int e17h = a /* c */ &&
            b;
        int e17i = a
            /* c */ && b;
        int e17j = a
		&& b;
        int e17k = a && b &&
            c;
        int e17l = a &&
            b && c;
        int e18a = a ||
            b;
        int e18b = a
            || b;
        int e18c = a
            ||
            b;
        int e18d = a || b;
        int e18e = a || 
            b;
        int e18f = a || // c
            b;
        int e18g = a || /* c */
            b;
        int e18h = a /* c */ ||
            b;
        int e18i = a
            /* c */ || b;
        int e18j = a
		|| b;
        int e18k = a || b ||
            c;
        int e18l = a ||
            b || c;
    }

    void ternary(boolean c, int a, int b, Object x, Object y) {
        int t1 = c ?
            a : b;
        int t2 = c
            ? a : b;
        int t3 = c ? a :
            b;
        int t4 = c ? a
            : b;
        int t5 = c
            ?
            a
            :
            b;
        int t6 = c ? a : b;
        int t7 = c ? // c
            a : b;
        int t8 = c ? a : // c
            b;
        int t9 = c ? c ?
            a : b : b;
        int t10 = c ? c
            ? a : b : b;
        int t11 = c ? a : c ?
            a : b;
        int t12 = c ? a : c
            ? a : b;
        Object t13 = c ? x :
            y;
        String t14 = c ? "a" :
            "b";
    }

    void colons(int k, int[] xs, Object x) {
        switch (k) {
            case 1:
                break;
            case 2:
            case 3:
                break;
            default:
                break;
        }
        switch (k) {
            case 1 -> k++;
            default -> k--;
        }
        label:
        for (int i : xs) {
            break label;
        }
        for (int i :
                xs) {
        }
        for (int i
                : xs) {
        }
        assert k > 0 :
            "msg";
        assert k > 0
            : "msg";
        switch (x) {
            case Integer i when i >
                1 -> k++;
            case String s when s.length() > 1
                && s.isEmpty() -> k++;
            default -> k--;
        }
    }

    void instanceofs(Object x, Object y) {
        boolean i1 = x instanceof
            String;
        boolean i2 = x
            instanceof String;
        boolean i3 = x instanceof String;
        boolean i4 = x instanceof String s &&
            s.isEmpty();
        boolean i5 = x instanceof String s
            && s.isEmpty();
        boolean i6 = x
            instanceof
            String;
        boolean i7 = x instanceof // c
            String;
        boolean i8 = x instanceof /* c */
            String;
        boolean i9 = x instanceof Box(String s) &&
            s.isEmpty();
    }

    <A extends Number &
        Comparable<A>> void te1() {}
    <A extends Number
        & Comparable<A>> void te2() {}
    <A extends Number & Comparable<A>> void te3() {}
    <A extends Number &
        Comparable<A> &
        Runnable> void te4() {}
    <A extends Number
        &
        Comparable<A>> void te5() {}
    void casts(Object x) {
        Object c1 = (Number &
            Comparable<?>) x;
        Object c2 = (Number
            & Comparable<?>) x;
        Object c3 = (Number & Comparable<?>) x;
    }

    void catches() {
        try {
            run();
        } catch (IllegalStateException |
                 IllegalArgumentException e) {
        }
        try {
            run();
        } catch (IllegalStateException
                 | IllegalArgumentException e) {
        }
        try {
            run();
        } catch (IllegalStateException | IllegalArgumentException e) {
        }
    }
    void assigns(int a, int b, int[] arr) {
        a =
            b;
        a
            = b;
        a
            =
            b;
        a = b;
        a = // c
            b;
        a = 
            b;
        a +=
            b;
        a
            += b;
        a
            +=
            b;
        a += b;
        a += // c
            b;
        a += 
            b;
        a -=
            b;
        a
            -= b;
        a
            -=
            b;
        a -= b;
        a -= // c
            b;
        a -= 
            b;
        a *=
            b;
        a
            *= b;
        a
            *=
            b;
        a *= b;
        a *= // c
            b;
        a *= 
            b;
        a /=
            b;
        a
            /= b;
        a
            /=
            b;
        a /= b;
        a /= // c
            b;
        a /= 
            b;
        a %=
            b;
        a
            %= b;
        a
            %=
            b;
        a %= b;
        a %= // c
            b;
        a %= 
            b;
        a <<=
            b;
        a
            <<= b;
        a
            <<=
            b;
        a <<= b;
        a <<= // c
            b;
        a <<= 
            b;
        a >>=
            b;
        a
            >>= b;
        a
            >>=
            b;
        a >>= b;
        a >>= // c
            b;
        a >>= 
            b;
        a >>>=
            b;
        a
            >>>= b;
        a
            >>>=
            b;
        a >>>= b;
        a >>>= // c
            b;
        a >>>= 
            b;
        a &=
            b;
        a
            &= b;
        a
            &=
            b;
        a &= b;
        a &= // c
            b;
        a &= 
            b;
        a |=
            b;
        a
            |= b;
        a
            |=
            b;
        a |= b;
        a |= // c
            b;
        a |= 
            b;
        a ^=
            b;
        a
            ^= b;
        a
            ^=
            b;
        a ^= b;
        a ^= // c
            b;
        a ^= 
            b;
        int d1 =
            1;
        int d2
            = 1;
        int d3
            =
            1;
        int d4 = 1;
        int d5 = // c
            1;
        int d6 =
            1, d7 =
            2;
        int[] d8 =
            {1, 2};
        int[] d9
            = {1, 2};
        int[] d10 = {1 +
            2, 3};
        int[] d11 = {1
            + 2, 3};
        arr[0] =
            1;
        arr[0]
            = 1;
        for (int i =
                 0; i < 1; i++) {}
        for (int i
                 = 0; i < 1; i++) {}
        try (Res r =
                 open()) {}
        try (Res r
                 = open()) {}
    }
    @interface An { int x(); int y() default 1; }
    @An(x =
        1, y = 2)
    void an1() {}
    @An(x
        = 1, y = 2)
    void an2() {}
    @An(x = 1,
        y = 2)
    void an3() {}
    static final int F1 =
        1;
    static final int F2
        = 1;
    static final int F3 = 1;

    void refs(Function<String, Integer> f) {
        Function<String, Integer> r1 = String::
            length;
        Function<String, Integer> r2 = String
            ::length;
        Function<String, Integer> r3 = String::length;
        Function<String, Integer> r4 = String
            ::
            length;
        Function<String, Integer> r5 = String:: // c
            length;
        Function<String, Integer> r6 = this::
            foo;
        Function<String, Integer> r7 = this
            ::foo;
        Function<String, Integer> r8 = List<String>::
            size;
        Object r9 = int[]::
            new;
        Object r10 = int[]
            ::new;
        Object r11 = foo()
            ::bar;
        Object r12 = foo().bar()::
            baz;
    }

    void lambdas() {
        Function<String, Integer> l1 = s ->
            s.length();
        Function<String, Integer> l2 = s
            -> s.length();
        Function<String, Integer> l3 = s -> s.length();
        Function<String, Integer> l4 = (s) ->
            s.length();
        Function<String, Integer> l5 = (s)
            -> s.length();
        Function<String, Integer> l6 = s ->
        {
            return 1;
        };
        Function<String, Integer> l7 = s
            ->
            s.length();
        Function<String, Integer> l8 = s -> // c
            s.length();
        Function<String, Integer> l9 = s -> {
            return 1;
        };
        Function<String, Integer> l10 = s
            -> {
            return 1;
        };
        Runnable l11 = () ->
            run();
        Runnable l12 = ()
            -> run();
        java.util.function.BiFunction<Integer, Integer, Integer> l13 = (a, b) ->
            a + b;
        java.util.function.BiFunction<Integer, Integer, Integer> l14 = (a, b)
            -> a + b;
        int sw = switch (1) {
            case 1 ->
                2;
            case 2
                -> 3;
            default -> 4;
        };
    }

    void misc(int a, int b, boolean c, List<String> xs) {
        int u1 = -
            a;
        int u2 = a -
            -b;
        int u3 = a
            - -b;
        int u4 = a - -
            b;
        int u5 = +
            a;
        int u6 = a +
            +b;
        int u7 = ~
            a;
        boolean u8 = !
            c;
        a++
            ;
        int u9 = a
            ++;
        int u10 = a++ +
            b;
        int u11 = a +
            ++b;
        int u12 = a++
            + b;
        List<
            String> g1;
        List<String
            > g2;
        java.util.Map<String,
            List<String>> g3;
        int u13 = a<b ? a : b;
        int u14 = a <b ? a : b;
        int u15 = (int)
            a;
        int u16 = (a)
            + b;
        int u17 = foo(a +
            b, c);
        int u18 = foo(a
            + b, c);
        int u19 = xs.
            size() +
            1;
        String u20 = "a" +
            "b" +
            "c";
        String u21 = "a"
            + "b"
            + "c";
        String u22 = "a" + "b"
            + "c" +
            "d";
        int[] u23 = new int[a +
            b];
        int[] u24 = new int[a
            + b];
        Object u25 = new Object[] {a +
            b, a
            + b};
        if (a > 1 &&
            b > 1 ||
            a < 0) {}
        if (a > 1
            && b > 1
            || a < 0) {}
        while (a <
            b) {}
        while (a
            < b) {}
        return;
    }
    void quirk(int a, int b) {
        a+ b;
        a +b;
        a+b;
        a
        +b;
        a
        + b;
        a;
        b
        +
        a;
        a /* c */
        + b;
        a /* c */ + b;
        a/* c */
        + b;
    }
    void tails(int a, int b, int c, boolean k, int[] arr) {
        Object x0 = -
            a;
        Object x1 = +
            a;
        Object x2 = !
            c;
        Object x3 = ~
            a;
        Object x4 = ++
            a;
        Object x5 = --
            a;
        Object x6 = - -
            a;
        Object x7 = -
            -
            a;
        Object x8 = (
            a);
        Object x9 = (int)
            a;
        Object x10 = (int) -
            a;
        Object x11 = -(
            a);
        Object x12 = !(
            a);
        Object x13 = (-
            a);
        Object x14 = new
            Foo();
        Object x15 = new Foo
            ();
        Object x16 = new Foo(
            a);
        Object x17 = new int[]
            {1};
        Object x18 = new int[] {
            1};
        Object x19 = this
            .foo();
        Object x20 = foo
            ();
        Object x21 = foo.
            bar();
        Object x22 = foo(
            a);
        Object x23 = "s"
            + b;
        Object x24 = 1
            + 2;
        Object x25 = arr[
            0];
        Object x26 = arr[0]
            ;
        Object x27 = c ?
            a : b;
        Object x28 = c ? a :
            b;
        Object x29 = x ->
            x;
        Object x30 = () ->
            a;
        Object x31 = (p, q) ->
            a;
        Object x32 = switch (k) {
            default -> 1; };
        Object x33 = new Object() {
            };
        Object x34 = - // c
            a;
        Object x35 = -/* c */
            a;
        Object x36 = /* c */ -
            a;
        Object x37 = -- /* c */
            a;
        Object x38 = // c
            a;
        Object x39 = /* c */
            a;
        Object x40 = a.b.c.
            d;
        Object x41 = a
            .b;
        Object x42 = -a
            + b;
        Object x43 = -a +
            b;
        Object x44 = (Foo)
            a;
        Object x45 = (Foo) -
            a;
        Object x46 = super
            .foo();
        Object x47 = foo()
            .bar();
        Object x48 = new Foo()
            .bar();
        Object x49 = a++
            ;
        Object x50 = a--
            ;
        Object x51 = a[0]
            .b;
        Object x52 = Foo::
            bar;
        Object x53 = new
            Foo();
        int[] ai = {
            1, 2};
        int[] aj =
            {1, 2};
        Object e4 = a /* multi
            line */ + b;
        Object e5 = a /* multi
            line */
            + b;
        Object e6 = a + /* multi
            line */ b;
        Object e7 = a +
            /* c */
            b;
        Object e8 = a + (
            b);
        Object e9 = a + (b
            );
        Object e10 = (a +
            b) +
            c;
        Object e11 = (a
            + b)
            + c;
        Object e12 = (a) +
            (b);
        Object e13 = (a)
            + (b);
        Object e14 = ((a
            ) + (b
            ));
        Object e15 = c ?
            (a) : b;
        Object e16 = c ? (a
            ) : b;
        Object e17 = c ? a : (
            b);
        Object e18 = c ? (a) :
            (b);
        Object e19 = c ? a : -
            b;
        Object e20 = c ? -
            a : b;
        Object e21 = foo(a,
            b) +
            c;
        Object e22 = foo(a,
            b)
            + c;
        Object e23 = a[0] +
            b[1]
            + c;
    }
}
