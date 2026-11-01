package p;

public class MtProbe {
    public class PubInner { }
    protected class ProtInner { }
    class PkgInner { }
    private class PrivInner { }
    /** doc */ public class DocInner { }
    // line comment only
    public class LineOnly { }
    /* block */ public class BlockOnly { }
    /** doc */
    @Deprecated
    public class DocAnn { }
    @Deprecated
    /** doc after annotation */
    public class DocAfterAnn { }
    /** doc */
    // between
    public class DocThenLine { }
    @Generated public class Gen { }
    @SuppressWarnings("x") public class Sup { }
    public interface PubIface { }
    public enum PubEnum { A }
    public @interface PubAnn { }
    public record PubRec(int a) { }
    interface Outer {
        class Implicit { }
        interface Nested { }
        enum En { X }
    }
    public static class PubOuter {
        protected class DeepProt { }
        class DeepPkg { }
        public class DeepPub { }
    }
    private static class PrivOuter {
        public class InPriv { }
    }
    void m() {
        class Local { }
        Object o = new Object() { };
    }
}

class PkgTop { }
/** doc */
class PkgTopDoc { }
public class PubTop { }
/** doc */
public class PubTopDoc { }
protected interface Weird { }
