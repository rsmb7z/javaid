package ok.pkg;

// ---
class U1 { U1() {} int r; }
// ---
class U2 { { } int r; }
// ---
class U3 { static { } int r; }
// ---
class U4 { void m() {} void n() {} int r; }
// ---
class U5 { int q; U5() {} class N {} }
// ---
class U6 { abstract void a(); abstract void b(); int c; }
// ---
interface U7 { void a(); void b(); int C = 1; }
// ---
@interface U8 { int v(); int w(); int x() default 1; }
// ---
class U9 { void m() { } @Deprecated int r; }
// ---
class U10 { void m() { } /* c */ int r; }
// ---
class U11 { void m() { }
 int r; }
// ---
class U12 { int a;
 int b; int c; }
// ---
class U13 { void m() { int a; if (a > 0) { a++; } else { a--; } a = 1; } }
// ---
class U14 { record R(int a) { } int r; }
// ---
class U15 { enum E { A } int r; }
// ---
class U16 { void m() { while (true) { } } int r; }
// ---
class U17 { @interface Q { } int r; }
// ---
class U18 { public static void main(String[] a) { } public int r; }
// ---
enum U19 { A, B; U19() {} int r; }
// ---
record U20(int a) { U20 { } int r; }
// ---
class U21 { Object o = new Object() { }; int r; }
// ---
class U22 { void m(int x) { switch (x) { case 1: break; default: } } int r; }
