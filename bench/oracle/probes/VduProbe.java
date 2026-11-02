package probe;

import java.util.*;
import java.util.function.*;

public class VduProbe {
    int field;
    static final int K = 1;

    interface I { int C = 3; }

    void basic() {
        int a = 1;
        int b = 2;
        foo();
        foo();
        foo();
        foo();
        use(a);
    }

    void basicFinalIgnored() {
        final int a = 1;
        foo();
        foo();
        foo();
        foo();
        use(a);
    }

    void exactlyAllowed() {
        int a = 1;
        foo();
        foo();
        foo();
        use(a);
    }

    void neverUsed() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        foo();
    }

    void sameStatements() {
        int count = 0;
        Object o = new Object();
        String s = "";
        foo();
        foo();
        foo();
        foo();
        for (int i = 0; i < 3; i++) {
            count++;
        }
    }

    void ifUsage(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f) {
            use(x);
        }
    }

    void ifCondUsage(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (x > 3) {
            foo();
        }
    }

    void whileUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        while (true) {
            use(x);
        }
    }

    void doUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        do {
            use(x);
        } while (x > 0);
    }

    void forUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        for (int i = 0; i < 2; i++) {
            use(x);
        }
    }

    void forEachUsage(List<String> l) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        for (String s : l) {
            use(x + s.length());
        }
    }

    void switchUsage(int k) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        switch (k) {
            case 1:
                use(x);
                break;
            default:
                foo();
        }
    }

    void switchRuleUsage(int k) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        switch (k) {
            case 1 -> use(x);
            default -> foo();
        }
    }

    int switchExprUsage(int k) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        return switch (k) {
            case 1 -> x;
            default -> 0;
        };
    }

    void tryUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try {
            use(x);
        } catch (RuntimeException e) {
            foo();
        } finally {
            foo();
        }
    }

    void tryFinallyUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try {
            foo();
        } catch (RuntimeException e) {
            foo();
        } finally {
            use(x);
        }
    }

    void tryResUsage() throws Exception {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try (java.io.StringReader r = new java.io.StringReader("" + x)) {
            foo();
        }
    }

    void syncUsage(Object lock) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        synchronized (lock) {
            use(x);
        }
    }

    void blockUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        {
            use(x);
        }
    }

    void lambdaUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        Runnable r = () -> use(x);
        r.run();
    }

    void lambdaBlockUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        Runnable r = () -> {
            use(x);
        };
        r.run();
    }

    void anonymousUsage() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        Object o = new Object() {
            int y = x;
        };
    }

    void initSequence() {
        StringBuilder sb = new StringBuilder();
        foo();
        foo();
        foo();
        foo();
        sb.append(1);
        sb.append(2);
    }

    void initSequence2() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        f.a();
        f.b();
    }

    void initSequenceBroken() {
        Foo f = new Foo();
        Foo g = new Foo();
        foo();
        foo();
        foo();
        foo();
        f.a();
        g.b();
    }

    void initSequenceNoInstance() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        a();
        f.b();
    }

    void initSequenceArray() {
        int[] arr = new int[3];
        foo();
        foo();
        foo();
        foo();
        arr[0] = 1;
        arr[1] = 2;
    }

    void initSequenceChain() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        f.c().a();
        f.c();
    }

    void initSequenceThis() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        this.f.a();
        this.f.b();
    }

    void initSequenceGeneric() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        f.<String>a();
        f.<String>b();
    }

    void initSequenceNew() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        new Foo().a();
        new Foo().b();
    }

    void initSequenceCast() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        ((Foo) f).a();
        ((Foo) f).b();
    }

    void initSequenceLit() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        "x".length();
        "x".length();
    }

    void initSequenceArrayCall() {
        Foo[] f = new Foo[2];
        foo();
        foo();
        foo();
        foo();
        f[0].a();
        f[1].b();
    }

    void initSequenceIf() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        if (true) {
            f.a();
        }
    }

    void initSequenceEmptyBlock() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        {
        }
        f.a();
    }

    void initSequenceSuper() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        super.toString();
        f.a();
    }

    void initSequenceMethodRef() {
        Foo f = new Foo();
        foo();
        foo();
        foo();
        foo();
        java.util.Objects.requireNonNull(f);
        java.util.Objects.requireNonNull(f);
    }

    void withSemi() {
        int a;
        a = 1;
        foo();
        foo();
        foo();
        foo();
        use(a);
    }

    void multiDecl() {
        int a = 1, b = 2;
        foo();
        foo();
        foo();
        foo();
        use(b);
    }

    void arrayDecl() {
        int[] a = {1, 2};
        int c[] = {1};
        foo();
        foo();
        foo();
        foo();
        use(a[0]);
        use(c[0]);
    }

    void shadow() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        Runnable r = () -> { int a2 = 1; };
        String a3 = "a";
    }

    void nestedDecl() {
        int outer = 1;
        {
            int inner = 2;
            foo();
            foo();
            foo();
            foo();
            use(inner);
        }
        use(outer);
    }

    void usedInDeclaration() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        int b = a + 1;
        use(b);
    }

    void usedInSecondDeclaration() {
        int a = 1;
        int b = 2;
        foo();
        foo();
        foo();
        int c = b;
        use(a);
    }

    void labels() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        label:
        for (;;) {
            use(a);
            break label;
        }
    }

    void caseGroupVars(int k) {
        switch (k) {
            case 1:
                int a = 1;
                foo();
                foo();
                foo();
                foo();
                use(a);
                break;
            case 2:
                int b = 2;
                foo();
                foo();
                foo();
                foo();
                use(b);
                break;
            default:
                break;
        }
    }

    void ifElseUsage(boolean f, boolean g) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f) {
            foo();
        } else if (g) {
            use(x);
        } else {
            foo();
        }
    }

    void ifElseBoth(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f) {
            use(x);
        } else {
            use(x);
        }
    }

    void ifNoBraces(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f)
            use(x);
        else
            foo();
    }

    void ifNoBracesElse(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f)
            foo();
        else
            use(x);
    }

    void ifNoBracesNoElse(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f)
            use(x);
    }

    void whileNoBraces() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        while (true)
            use(x);
    }

    void forNoBraces() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        for (;;)
            use(x);
    }

    void doNoBraces() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        do
            use(x);
        while (true);
    }

    void tryInTry() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try {
            try {
                use(x);
            } finally {
                foo();
            }
        } finally {
            foo();
        }
    }

    void nestedIfs(boolean f) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        if (f) {
            if (f) {
                foo();
                use(x);
            }
        }
    }

    void nestedLoopsWithUse() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        for (int i = 0; i < 2; i++) {
            while (i > 0) {
                use(x);
            }
        }
    }

    void usedTwice() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        use(x);
        use(x);
    }

    void usedInSwitchTwoBranches(int k) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        switch (k) {
            case 1:
                use(x);
                break;
            case 2:
                use(x);
                break;
            default:
        }
    }

    void usedInSwitchRuleBranch(int k) {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        switch (k) {
            case 1 -> foo();
            case 2 -> {
                foo();
                use(x);
            }
            default -> foo();
        }
    }

    void usedInTryCatchBoth() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try {
            use(x);
        } catch (RuntimeException e) {
            use(x);
        }
    }

    void usedInCatchOnly() {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try {
            foo();
        } catch (RuntimeException e) {
            foo();
            use(x);
        }
    }

    void usedInTryResourceHeader() throws Exception {
        int x = 5;
        foo();
        foo();
        foo();
        foo();
        try (java.io.StringReader r = new java.io.StringReader("" + x); java.io.StringReader q = new java.io.StringReader("")) {
            use(x);
        }
    }

    void lambdaParam() {
        Function<Integer, Integer> f = y -> {
            int a = y;
            foo();
            foo();
            foo();
            foo();
            return a;
        };
    }

    static {
        int sa = 1;
        foo2();
        foo2();
        foo2();
        foo2();
        use2(sa);
    }

    {
        int ia = 1;
        foo();
        foo();
        foo();
        foo();
        use(ia);
    }

    VduProbe() {
        int ca = 1;
        foo();
        foo();
        foo();
        foo();
        use(ca);
    }

    enum E {
        A {
            void m() {
                int ea = 1;
                foo2();
                foo2();
                foo2();
                foo2();
                use2(ea);
            }
        };

        void foo2() {}

        void use2(int x) {}
    }

    record R(int a) {
        R {
            int ra = 1;
            foo2();
            foo2();
            foo2();
            foo2();
            use2(ra);
        }

        static void foo2() {}

        static void use2(int x) {}
    }

    void patternVars(Object o) {
        if (o instanceof String s) {
            int a = 1;
            foo();
            foo();
            foo();
            foo();
            use(a);
        }
    }

    void ignoredName() {
        int ignoredVar = 1;
        foo();
        foo();
        foo();
        foo();
        use(ignoredVar);
        int otherVar = 1;
        foo();
        foo();
        foo();
        foo();
        use(otherVar);
    }

    void unusedAtEnd() {
        int a = 1;
    }

    void varDecl() {
        var a = 1;
        foo();
        foo();
        foo();
        foo();
        use(a);
    }

    void annotated() {
        @SuppressWarnings("x") int a = 1;
        foo();
        foo();
        foo();
        foo();
        use(a);
    }

    void comments() {
        int a = 1;
        // c
        foo();
        /* c */ foo();
        foo(); // c
        foo();
        use(a);
    }

    void forInitVar() {
        for (int i = 0, j = 0; i < 3; i++) {
            foo();
        }
    }

    void usedInReturn() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        return;
    }

    int usedInReturn2() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        return a;
    }

    void emptyStatements() {
        int a = 1;
        ;
        ;
        ;
        ;
        use(a);
    }

    void labeledAndAssert() {
        int a = 1;
        assert a > 0;
        foo();
        foo();
        foo();
        foo();
        assert a > 0 : "x";
    }

    void throwAndSync(Object lock) {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        throw new RuntimeException("" + a);
    }

    void usedInElseIfCondition(boolean f) {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        if (f) {
            foo();
        } else if (a > 0) {
            foo();
        }
    }

    void usedInFor3Parts() {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        for (int i = 0; i < a; i++) {
            foo();
        }
    }

    void usedInForEachExpr(List<Integer> l) {
        int a = 1;
        foo();
        foo();
        foo();
        foo();
        for (int i : java.util.Collections.nCopies(a, 1)) {
            foo();
        }
    }

    void chained() {
        StringBuilder sb = new StringBuilder();
        String s = "";
        foo();
        foo();
        foo();
        foo();
        sb.append(s);
    }

    void twoVars() {
        int a = 1;
        int b = 2;
        foo();
        foo();
        foo();
        foo();
        use(b);
        use(a);
    }

    void foo() {}

    void a() {}

    void use(int x) {}

    static void foo2() {}

    static void use2(int x) {}

    Foo f;

    static class Foo {
        void a() {}

        void b() {}

        Foo c() {
            return this;
        }
    }
}
