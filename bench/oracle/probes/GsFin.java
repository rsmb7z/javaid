package bench.oracle.probes;

import java.lang.annotation.Retention;

public class GsFin {
    protected void finalize() throws Throwable {
    }

    void finalize(int x) {
    }

    int finalize(int... xs) {
        return 0;
    }

    @Override
    public void finalize
    (
    ) {
    }

    <T> void finalize() {
    }

    abstract static class A {
        abstract void finalize();

        void notFinalize() {
            finalize();
            this.finalize();
        }

        /* c */ void /* d */ finalize /* e */ ( /* f */ ) { }
    }

    interface I {
        void finalize();

        default void finalize(String s) {
        }

        static void finalize(Object o, Object p) {
        }
    }

    enum E {
        A {
            void finalize() {
            }
        };

        void finalize() {
        }
    }

    record R(int a) {
        void finalize() {
        }

        public int finalize(int a) {
            return a;
        }
    }

    @interface Ann {
        int finalize();
    }

    Object o = new Object() {
        @Override
        protected void finalize() {
        }
    };

    void finalizer() {
    }

    void Finalize() {
    }

    void local() {
        class L {
            void finalize() {
            }
        }
        Runnable r = () -> {
        };
    }

    void finalize(GsFin this) {
    }
}
