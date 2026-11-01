package ok.pkg;

public class SwitchProbe {
    enum Color { RED, GREEN }
    sealed interface Shape permits Circle, Square {}
    record Circle() implements Shape {}
    record Square() implements Shape {}

    void f(int x, Object o, Color c, Shape sh, String str, int y) {
        switch (x) { case 1: break; }
        switch (x) { case 1: break; default: break; }
        switch (x) { default: break; }
        switch (x) { }
        switch (x) { case 1: break; case 2: break; }
        int a = switch (x) { case 1 -> 1; default -> 0; };
        int b = switch (x) { case 1 -> 1; case 2 -> 2; };
        switch (x) { case 1 -> foo(); }
        switch (x) { case 1 -> foo(); default -> bar(); }
        switch (c) { case RED: break; case GREEN: break; }
        switch (c) { case RED -> foo(); case GREEN -> bar(); }
        switch (o) { case String s -> foo(); case Integer i -> bar(); }
        switch (o) { case String s -> foo(); case null, default -> bar(); }
        switch (o) { case null -> foo(); default -> bar(); }
        switch (sh) { case Circle ci -> foo(); case Square sq -> bar(); }
        switch (x) { case 1: default: break; }
        switch (x) { default: case 1: break; }
        switch (x) { case 1, 2 -> foo(); case 3 -> bar(); }
        switch (str) { case "a": foo(); }
        switch (o) { case Integer i when i > 0 -> foo(); case Integer i -> bar(); }
        switch (o) { case String s: foo(); break; }
        Runnable r = () -> { switch (x) { case 1: break; } };
        switch (x) { case 1: switch (y) { case 2: break; } break; default: break; }
        switch (x) { case 1: switch (y) { case 2: break; default: break; } break; }
        int d = switch (o) { case String s -> 1; case Integer i -> 2; default -> 3; };
        int e = switch (o) { case String s -> 1; case Integer i -> 2; };
        switch (o) { case Circle(), Square() -> foo(); }
        switch (o) { case null: break; }
        switch (o) { case null: break; default: break; }
        switch (x) { case 1 -> { foo(); } }
        switch (x) { case 1: { break; } }
        switch (c) { case RED: }
    }
    class L { void m(int q) { switch (q) { case 1: } } }
    void foo() {}
    void bar() {}
}
