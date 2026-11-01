package probes;

import java.io.IOException;
import java.util.List;
import java.util.function.BiFunction;
import java.util.function.Function;

public class AbProbe {
    int HTTPServer;
    int httpSERVER;
    int aBCd;
    int ABc;
    int abC;
    int ABCD;
    int HTTP_Server;
    int a_BCD_e;
    int XMLHttpRequest;
    int xmlHTTPRequest;
    static int STATICfield;
    static int staticFIELD;
    final int FINALfield = 1;
    final int finalFIELD = 1;
    static final int STATIC_FINAL_ABCDE = 1;
    static final int STATICFinalABCDE = 1;
    static final int staticFinalABCDE = 1;
    private int ID;
    private int IDs;
    private int theID;
    private int theIDs;

    interface IFace {
        int CONST_ABCD = 1;
        int constABCDE = 2;
        String HTTPValue = "x";
        void methodABCD();
    }

    @interface ANNOTation {
        int valueABCD();
        int ok();
    }

    class InnerHTTPClass {}
    class InnerHttpClass {}
    class XMLParser {}
    class Xml {}
    interface URLProvider {}
    enum HTTPCodes { OK, NOTFOUND, NotFOUND, aBCDEF }
    record RecABC(int xABCD, int yAbc, int ZZ) {}
    record Recabc(int xABCD, int ok) {}
    @interface ABCDAnno {}

    void methodABCD() {}
    void methodAbcd() {}
    void getHTTPResponse() {}
    void getHttpResponse() {}
    void getHTTP() {}
    void ABC() {}
    void AB() {}
    @Override
    public String toString() { return ""; }
    @Override public int hashCodeABCD() { return 0; }
    @java.lang.Override public int equalsABCD() { return 0; }
    @Override() public int compareABCD() { return 0; }
    @Deprecated void deprecatedABCD() {}

    void params(int paramABCD, int paramAbcd, final int finalABCD, int... varARGS) {}
    void rcv(AbProbe this, int xABCD) {}
    AbProbe(int ctorABCD) {}
    AbProbe() {}

    void locals() {
        int localABCD = 1;
        final int finalABCD = 1;
        static_ok();
        for (int loopABCD = 0; loopABCD < 1; loopABCD++) {}
        for (final int loopFINALabcd : new int[0]) {}
        for (String eachABCD : List.of("a")) {}
        try (java.io.StringReader resABCD = new java.io.StringReader("")) {
        } catch (IOException exABCD) {
        } catch (RuntimeException eABC) {}
        try (final java.io.StringReader FINALRES = new java.io.StringReader("")) {} catch (IOException e) {}
        Function<Integer, Integer> f = xABCD -> xABCD;
        BiFunction<Integer, Integer, Integer> g = (aABCD, bABCD) -> aABCD;
        BiFunction<Integer, Integer, Integer> h = (Integer cABCD, Integer dABCD) -> cABCD;
        Object o = "x";
        if (o instanceof String sABCD) {}
        if (o instanceof final String sFINALABCD) {}
        switch (o) {
            case Integer iABCD -> {}
            case String s -> {}
            default -> {}
        }
        Runnable r = new Runnable() {
            int anonABCD;
            public void run() {
                int innerABCD = 1;
            }
        };
        var varABCD = 1;
        int arr[] = {1}, otherABCD, ok;
    }

    void static_ok() {}
    <TABCD> void generic(TABCD t) {}
}

enum EnumABCD { FIRST, SECONDABCD }

record RecordABCD(int xABCD) {
    RecordABCD {}
}
