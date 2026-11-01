package ok.pkg;

public class FtYield {
    void foo() {}
    int f(int x) {
        int r = switch (x) {
            case 1:
                foo();
                yield 1;
            case 2:
                foo();
            case 3:
                yield 3;
            default:
                yield 4;
        };
        switch (x) {
            case 1 -> foo();
            case 2 -> { foo(); }
            default -> foo();
        }
        return r;
    }
}
