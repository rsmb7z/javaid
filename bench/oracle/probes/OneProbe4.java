package ok.pkg;

// ---
class T1 { class N {} }
// ---
class T2 { int a; }
// ---
class T3 { void m() {} }
// ---
class T4 { class N {} class M {} }
// ---
class T5 { enum E { A } }
// ---
class T7 { interface I {} int x; }
// ---
class T8 { void m() { class L {} } }
// ---
class T9 { void m() { int a; } }
// ---
class T10 { void m() { foo(); } void foo() {} }
// ---
class T11 { int a; int b; }
// ---
class T12 { class N { int q; } }
// ---
class T13 { class N { int q; } int r; }
// ---
class T14 { void m() {} int r; }
// ---
class T15 { void m() {} class N {} }
// ---
class T16 { int a; void m() {} }
// ---
class T17 { void m() { } void n() { foo(); } void foo() {} }
// ---
class T18 { void m() { foo(); } void foo() {} class N {} }
// ---
class T19 { void m() { if (true) { foo(); } } void foo() {} }
// ---
class T20 { void m() { Runnable r = () -> { foo(); }; } void foo() {} }
