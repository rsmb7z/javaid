package ok.pkg;

import java.util.function.Function;

/** Second batch of naming edge cases. */
public class Probe2<T extends Comparable<T>, Bad_U extends T> {
    static {
        int Bad_StaticInit = 1;
    }

    {
        int Bad_InstInit = 2;
    }

    void switchy(int k) {
        switch (k) {
            case 1:
                int Bad_CaseLocal = 1;
                break;
            default:
                final int Bad_CaseFinal = 2;
        }
        Function<Integer, Integer> f = x -> {
            int Bad_LambdaLocal = 1;
            return x;
        };
        var Bad_Var = 3;
        for (var Bad_ForVar : new int[0]) {}
        record Bad_LocalRecord(int Bad_LocalComp) {}
        enum Bad_LocalEnum {
            A
        }
        interface Bad_LocalIface {}
        new Object() {
            void Bad_AnonMethod(int Bad_AnonParam) {}
        };
        int goodOne = 1, Bad_SecondDeclarator = 2;
        int[] Bad_ArrayLocal[] = null;
    }

    static class lower {
        void lower() {}

        int Lower() {
            return 0;
        }
    }

    class Same {
        int Same() {
            return 0;
        }
    }

    @Override
    public String toString() {
        return null;
    }

    enum E {
        A {
            int Bad_ConstBodyField;

            void Bad_ConstBodyMethod() {}
        }
    }

    int[] Bad_ArrayField[];
    int a, Bad_B, c;
    int Bad_Multi1, Bad_Multi2;
    static final int A_B_C = 1, bad_d = 2;
    public static final String CamelCase = "x";
    private static final long serialVersionUID = 1L;
    protected static final Object LOCK_ = new Object();
    static final int X1 = 1, x2 = 2;
    static final int A__B = 3;
    static final int _A = 4;
    private final int Bad_PrivateFinal = 5;
    public int Bad_Public;
    protected int Bad_Protected;

    interface Nested {
        class InnerInIface {
            int Bad_InnerField;
        }

        int ok_const_in_iface_bad = 1;
    }

    public void sameNameParams(int a, int B) {}

    abstract static class Abs {
        abstract void Bad_Abstract(int Bad_AbsParam);
    }

    native void Bad_Native(int Bad_NativeParam);
}
