module m {
    exports p1;
    requires a;
    opens q1;
    exports p2;
    uses u.V;
    requires b;
    provides s.T with x.Y;
    uses u.W;
    opens q2;
    requires c;
    provides s.U with x.Z;
    exports p3;
    opens q3;
}
