package Bad.Pkg;

import java.io.IOException;
import java.io.Reader;
import java.util.List;
import java.util.function.BiFunction;
import java.util.function.Function;

/** One distinctively-named violation per construct: Checkstyle's output says which check owns it. */
public class Probe_Type<Bad_CT> {
    int Bad_Member;
    static int Bad_Static;
    static final int bad_const = 1;
    final int Bad_FinalInstance = 1;
    static final long serialVersionUID = 1L;
    private static final java.io.ObjectStreamField[] serialPersistentFields = {};

    Probe_Type() {}

    <Bad_X> Probe_Type(int Bad_CtorParam) {}

    <Bad_MT> void Probe_Method(int Bad_Param, final int Bad_FinalParam, int... Bad_Varargs) {
        int Bad_Local = 1;
        final int Bad_FinalLocal = 2;
        for (int Bad_ForInit = 0; ; ) {}
        for (final int Bad_ForFinal : new int[0]) {}
        for (String Bad_ForEach : new String[0]) {}
        try (Reader Bad_Res = null) {
        } catch (IOException | RuntimeException Bad_Catch) {
        } catch (final Exception Bad_FinalCatch) {
        }
        Function<Integer, Integer> f1 = Bad_Lambda1 -> Bad_Lambda1;
        BiFunction<Integer, Integer, Integer> f2 = (Bad_L2a, Bad_L2b) -> Bad_L2a;
        BiFunction<Integer, Integer, Integer> f3 = (Integer Bad_L3a, final Integer Bad_L3b) -> Bad_L3a;
        Object o = null;
        if (o instanceof String Bad_Pat) {}
        if (o instanceof final String Bad_FinalPat) {}
        if (o instanceof Point(int Bad_RecPx, var Bad_RecPy)) {}
        switch (o) {
            case Integer Bad_SwitchPat when Bad_SwitchPat > 1 -> {}
            default -> {}
        }
        class Local_Class {}
        Runnable r = new Runnable() {
            int Bad_AnonField;

            public void run() {}
        };
    }

    interface Bad_Iface<Bad_IT> {
        int Bad_IfaceConst = 1;

        <Bad_IM> void ok(Bad_IM x);
    }

    @interface Bad_Ann {
        int Bad_AnnConst = 1;

        int Bad_Elem() default 1;
    }

    enum Bad_Enum {
        Bad_EnumConst;
        int Bad_EnumField;
        static int Bad_EnumStatic;
    }

    record Bad_Rec<Bad_RT>(int Bad_Comp) {
        Bad_Rec {}

        static int Bad_RecStatic;

        void Bad_RecMethod(int Bad_RecParam) {}
    }

    static void Probe_Static() {}

    @Override
    public String Bad_OverrideMethod() {
        return null;
    }
}
