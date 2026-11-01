package ok.pkg; import java.util.List; import java.util.Map; class Z { int zz; }
enum E1 { A; int x; }
enum E2 { A, B; }
enum E3 { A; void m() { foo(); } int y; }
interface I1 { void a(); int X = 1; }
abstract class C1 { abstract void a(); int y; }
class C2 { int a; ; int b; }
class C3 { void m() { int a; } int b; }
class C4 { int a; class N {} }
class C5 { class N {} int b; }
class C6 { int a; enum EN { P } }
@interface An { int v(); int w(); }
class C7 { void m() { foo(); do { } while (true); bar(); } void foo() {} void bar() {} }
class C8 { void m() { foo(); foo()
            .toString(); } String foo() { return ""; } }
class C9 { void m() { int a; foo()
            .toString(); } String foo() { return ""; } }
class C10 { void m() { int a; String s = foo()
            + foo(); int b; } String foo() { return ""; } }
class C11 { void m() { int a; int b =
             1; int c; } }
class C12 { void m() { foo(
             ); bar(); } void foo() {} void bar() {} }
class C13 { void m() { int a; /* multi
   line */ int b; } }
class C14 { void m() { foo(); switch (1) { case 1: bar(); } baz(); } void foo() {} void bar() {} void baz() {} }
class C15 { void m() { foo(); Runnable r = () -> foo(); } void foo() {} }
class C16 { void m() { Runnable r = () -> foo(); foo(); } void foo() {} }
class C17 { void m() { int a; try (java.io.StringReader s = null) { } int b; } }
class C18 { void m() { foo(); Object o = new Object() { void q() { } }; } void foo() {} }
class C19 { void m() { for (int i = 0; i < 1; i++) { } int b; int c; } }
