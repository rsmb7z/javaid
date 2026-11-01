package probes;

import org.junit.Test;

public class GnProbe {
    int goodName;
    int a;
    int aB;
    int aBc;
    int ab;
    int aB1;
    int a1;
    int a1B;
    int abC;
    int abCD;
    int HTTPServer;
    int Upper;
    int _lead;
    int trail_;
    int dou__ble;
    int snake_case;
    int snake_1;
    int field_1_2;
    int field1_2;
    int a_1;
    int x_b;
    int x1_y;
    int ab_1b;
    int ü;
    int naïve;
    int xyZ9;
    int $dollar;
    int a$b;
    static int STATIC_FIELD;
    static int badStatic_;
    final int FINAL_FIELD = 1;
    int multi, Multi2, mUlti3;

    interface I {
        int BadName = 1;
    }

    void method() {
        int Local_Bad = 1;
        for (int Loop_Bad = 0; ; ) {}
    }

    class Inner {
        int Inner_Bad;
        int innerOk;
    }

    enum E { A_B; int Bad_Enum; int ok; }
    record R(int Bad_Comp) { static int BAD; }

    Object o = new Object() {
        int Anon_Bad;
    };

    void goodMethod() {}
    void a() {}
    void ab() {}
    void aB() {}
    void aBc() {}
    void aBC() {}
    void aBcD() {}
    void abcD() {}
    void Upper() {}
    void _lead() {}
    void trail_() {}
    void dou__ble() {}
    void snake_case() {}
    void snake_1() {}
    void method_1_2() {}
    void method1_2() {}
    void method_a1() {}
    void a1b2() {}
    void a1() {}
    void getHTTPResponse() {}
    void x_1() {}
    void ü() {}
    @Override public String toString() { return ""; }
    @Override public int hashCode_x() { return 0; }
    @java.lang.Override public int equals_x() { return 0; }
    @Override() public int compare_x() { return 0; }
    @Test void testOk() {}
    @Test void test_snake_case() {}
    @Test void test_Bad() {}
    @Test void test__double() {}
    @Test void _test() {}
    @Test void test_() {}
    @Test void test_1() {}
    @Test void test_a_1() {}
    @Test void test1_a() {}
    @Test void Test_bad() {}
    @Test void a() {}
    @Test void aB_c() {}
    @org.junit.Test void qualified_name_ok() {}
    @org.junit.jupiter.api.Test void jupiter_name() {}
    @ParameterizedTest void param_test() {}
    @RepeatedTest(3) void repeated_test() {}
    @TestFactory void factory_test() {}
    @org.junit.jupiter.api.TestFactory void factory_q_test() {}
    @MyTest void my_test() {}
    @Test @Deprecated void both_annots() {}
}
