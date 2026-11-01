import java.util.List;
import module java.base;

int counter = 0;
static final String NAME = "x", OTHER = "y";

void main() {
    System.out.println("hi");
}

private static int helper(int a) {
    return a + 1;
}

record R(int a) {}

class K {}

String greeting() { return "hello"; }
