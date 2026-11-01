package ok.pkg;

/** TypeName scoped by token kind: classes get one format, everything else another. */
public class Probe4 {
    class Good1 {}

    class Bad_Class {}

    class Foo1_2 {}

    interface Iface1_2 {}

    interface Bad_Iface {}

    enum Enum1_2 {
        A
    }

    enum Bad_Enum {
        B
    }

    @interface Ann1_2 {}

    @interface Bad_Ann {}

    record Rec1_2(int a) {}

    record Bad_Rec(int a) {}

    void local() {
        class Local1_2 {}
        class Local_Bad {}
        record LocalRec1_2(int a) {}
        interface LocalIface1_2 {}
    }
}
