module m {
    requires a;
    requires b;
    exports p1;
    exports p2;
    // comment between

    opens q1;


    opens q2;
    uses u.V; uses u.W;
    uses u.X;

    /* block */

    provides s.T with x.Y;
    provides s.U with
        x.Z;

    // trailing
}
