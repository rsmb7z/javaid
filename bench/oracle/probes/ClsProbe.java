public class ClsProbe {
    static class A { private A() { } }
    static class B { private B() { } static class BB extends B { } }
    static class C { private C() { } C(int x) { } }
    static class D { private D() { } private D(int x) { } }
    static class E { private E() { } private E(int x) { } static class F extends E { F() { super(); } } }
    final static class G { private G() { } }
    abstract static class H { private H() { } }
    static class I { }
    static class J { J() { } }
    static class K { protected K() { } }
    static class L { private L() { } }
    class M extends L { M() { super(); } }
    interface N { private static void p() { } }
    enum O { X; private O() { } }
    record P(int a) { }
    static class Q { private Q() { } void m() { new Q() { }; } }
    static class R { private R() { } void m() { new R(); } }
    static class S { private S() { } static class T { } }
    sealed static class U permits V { private U() { } }
    static final class V extends U { V() { } }
    static class W { private W() { } static W x = new W() { }; }
}
class Util1 { static void f() { } }
class Util2 { static int x; }
class Util3 { static void f() { } void g() { } }
class Util4 { static void f() { } Util4() { } }
class Util5 { static void f() { } private Util5() { } }
class Util6 { static void f() { } protected Util6() { } }
class Util7 extends Object { static void f() { } }
class Util8 extends Util1 { static void f() { } }
class Util9 { }
class Util10 { static final int X = 1; static { } }
class Util11 { static class Inner { } }
class Util12 { static void f() { } public Util12() { } }
class Util13 { static void f() { } Util13(int a) { } }
abstract class Util14 { static void f() { } }
final class Util15 { static void f() { } }
class Util16 { public static void main(String[] a) { } }
class Util17 { int x; static void f() { } }
class Util18 { static void f() { } { } }
class Util19 implements Runnable { static void f() { } public void run() { } }
class Util20 { @Deprecated static void f() { } }
interface Util21 { static void f() { } }
enum Util22 { ; static void f() { } }
record Util23() { static void f() { } }
@interface Util24 { }
class Util25 { static void f() { } class In { void g() { } } }
class Util26 { static void f() { } static class In { void g() { } } }
class Util27 { static void f() { } public Util27() { } private Util27(int a) { } }
interface Marker1 { }
interface Marker2 extends Marker1 { }
interface Consts { int A = 1; }
interface Consts2 { static final int A = 1; String B = ""; }
interface Meths { void f(); }
interface Mixed { int A = 1; void f(); }
interface Gen<T> { }
interface Def { default void f() { } }
interface Stat { static void f() { } }
interface Nest { class X { } }
interface Priv { private void f() { } }
@interface Ann { int X = 1; }
@interface Ann2 { }
class Outer { interface InnerM { } interface InnerC { int A = 1; } }
