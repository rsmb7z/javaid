package bench.oracle.probes;

import java.util.List;

public class GsRecAcc {
    record R(int x, String name, List<String> items, int... rest) {
        public int x() {
            return x;
        }

        @Override
        public String name() {
            return name;
        }

        @java.lang.Override
        public List<String> items() {
            return items;
        }

        public int[] rest() {
            return rest;
        }

        public int y() {
            return 0;
        }

        public int x(int a) {
            return a;
        }

        @Deprecated
        public int rest(int... more) {
            return 1;
        }

        @SuppressWarnings("x")
        String name(String a) {
            return a;
        }

        static int x(R other) {
            return other.x;
        }
    }

    record S(@Deprecated int a, int b) {
        @Override()
        public int a() {
            return a;
        }

        int b() {
            return b;
        }

        public int c() {
            return 0;
        }

        S {
        }

        <T> int a() {
            return 0;
        }

        int a() throws Exception {
            return 0;
        }

        @Deprecated @Override public int hashCode() {
            return 0;
        }
    }

    record T() {
        int a() {
            return 0;
        }
    }

    class NotRecord {
        int x;

        int x() {
            return x;
        }
    }

    interface I {
        int x();
    }

    enum E {
        A;

        int a() {
            return 0;
        }
    }

    void method() {
        record Local(int q) {
            int q() {
                return q;
            }
        }
        Object o = new Object() {
            int q() {
                return 0;
            }
        };
    }

    record Outer(int z) {
        record Nested(int w) {
            int z() {
                return 0;
            }

            int w() {
                return w;
            }
        }

        class C {
            int z() {
                return 0;
            }
        }

        int z() {
            return z;
        }
    }
}
