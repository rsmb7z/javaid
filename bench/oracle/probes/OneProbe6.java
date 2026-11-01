package ok.pkg;

// ---
class D1 { void m(boolean b) { do { } while (b); bar(); } void bar() {} }
// ---
class D2 { void m(boolean b) { do { } while (b); int x; } }
// ---
class D3 { void m(boolean b) { do ; while (b); bar(); } void bar() {} }
// ---
class D4 { void m(boolean b) { do { }
 while (b); bar(); } void bar() {} }
// ---
class D5 { int m(boolean b) { do { }
 while (b); return 1; } }
// ---
class D6 { void m(boolean b) { do { bar(); } while (b); bar(); } void bar() {} }
// ---
class D7 { void m(boolean b) { bar(); do { } while (b); } void bar() {} }
// ---
class D8 { void m(boolean b) { do { } while (b); do { } while (b); } }
// ---
class D9 { void m(boolean b) { while (b) { } bar(); } void bar() {} }
// ---
class D10 { void m(boolean b) { if (b) { } else { } bar(); } void bar() {} }
// ---
class D11 { void m(boolean b) { do { } while (b); bar(); bar(); } void bar() {} }
