package ok.pkg;

import java.util.List;

/** Non-default naming options: allowOneCharVarInForLoop, ignoreOverridden, allowClassName. */
public class Probe3 {
    void loops(int[] arr, List<String> list) {
        for (int I = 0; I < 3; I++) {}
        for (int J : arr) {}
        for (String Kk : list) {}
        int L = 0;
        for (int i = 0, J2 = 1; ; ) {}
        for (final int M : arr) {}
        for (int Ab = 0; ; ) {}
        for (int x = 0; x < 2; x++) {
            int N = 1;
        }
    }

    static class Base {
        void run(int Bad_P) {}
    }

    static class Derived extends Base {
        @Override
        void run(int Bad_P) {}

        void other(int Bad_Q) {}
    }

    static class lower {
        void lower() {}
    }
}
