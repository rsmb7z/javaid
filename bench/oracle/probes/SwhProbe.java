package p;

import java.util.*;

@SuppressWarnings("membername")
class A {
  int Bad_One = 1;
  void Bad_M(int x) { int y = 42; }
  @SuppressWarnings("all") void Bad_N(int x) { int y = 43; }
  @SuppressWarnings("checkstyle:magicnumber") int z = 44; int w = 45;
  @SuppressWarnings({"finalparameters", "MethodName"}) void Bad_O(int p) { int q = 46; }
  @java.lang.SuppressWarnings("MagicNumber") void f() { int a = 47; int Bad_P = 48; }
  @SuppressWarnings(value = "MagicNumber") void g() { int a = 49; }
  @SuppressWarnings(value = {"MagicNumber", "FinalParameters"}) void h(int pp) { int a = 50; }
  @SuppressWarnings("unchecked") void i(int pp) { int a = 51; }
  @SuppressWarnings("") void j() { int a = 52; }
  @SuppressWarnings(CONST) void k() { int a = 53; }
  @SuppressWarnings(A.CONST) void l() { int a = 54; }
  @SuppressWarnings("Check") void m() { int a = 55; }
  @SuppressWarnings("magicnumbercheck") void n() { int a = 56; }
  @SuppressWarnings("MagicNumberCheck") void o() { int a = 57; }
  @SuppressWarnings("tag") void pq() { int a = 58; }
  @SuppressWarnings("myid") void r() { int a = 59; }
  @SuppressWarnings("LineLength") void s() { int a = 60;                                                                                                       }
  void t(@SuppressWarnings("FinalParameters") int pa, int pb) { int a = 61; }
  void u() { @SuppressWarnings("MagicNumber") int a = 62; int b = 63; }
  @SuppressWarnings("MagicNumber") int a1 = 64, a2 = 65;
  @SuppressWarnings("MagicNumber")
  enum E { X(66), Y(67); E(int v) { } }
  @SuppressWarnings("MagicNumber") interface I { int Q = 68; }
  @SuppressWarnings("MagicNumber") record R(int c) { static int zz = 69; }
  @SuppressWarnings("""
     MagicNumber""") void tb() { int a = 70; }
  @SuppressWarnings("MagicNumber") /* c */ void cm() { int a = 71; }
  static final String CONST = "x";
	@SuppressWarnings("MagicNumber") void tabbed() { int a = 72; }
}
