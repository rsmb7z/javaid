import a.b.C;

@Deprecated
open module bench.oracle.probes {
    requires a;
    requires transitive b;

    requires static c;
    exports p1;
    exports p2 to m1, m2;

    exports p3;

    opens q1;
    opens q2 to m3;
    uses a.b.C;

    provides a.b.C with d.E, f.G;
}
