import java.util.*;
import java.util.function.*;
import static java.lang.Math.*;

public class WarProbe<T extends Number&Comparable<T>> {
  int f1=1;
  int f2 = 1;
  int[] arr1={1,2};
  int[] arr2 = {1, 2};
  int[] arr3 = { 1, 2 };
  int[] arr4 = {};
  int[] arr5 = { };
  int[][] arr6 = {{1},{2}};
  int[][] arr7 = { {1}, {2} };
  Map<String,List<? extends Number>> m1;
  Map<String, List<? super Integer>> m2;
  List<?> w1;
  List<? > w2;
  List <String> g1;
  List< String> g2;
  static <A extends Object&Comparable<A>> A tight(A a) { return a; }
  static <A extends Object & Comparable<A>> A loose(A a) { return a; }
  void ops(int a, int b, boolean p, boolean q, long l) {
    int c=a+b;
    int d = a + b;
    int e = a +b;
    int f = a+ b;
    c = a-b;
    c = a - b;
    c = a*b;
    c = a * b;
    c = a/b;
    c = a / b;
    c = a%b;
    c = a % b;
    c = a<<b;
    c = a << b;
    c = a>>b;
    c = a >> b;
    c = a>>>b;
    c = a >>> b;
    c = a&b;
    c = a & b;
    c = a|b;
    c = a | b;
    c = a^b;
    c = a ^ b;
    p = a<b;
    p = a < b;
    p = a>b;
    p = a > b;
    p = a<=b;
    p = a <= b;
    p = a>=b;
    p = a >= b;
    p = a==b;
    p = a == b;
    p = a!=b;
    p = a != b;
    p = p&&q;
    p = p && q;
    p = p||q;
    p = p || q;
    c+=1;
    c += 1;
    c-=1;
    c -= 1;
    c*=2;
    c *= 2;
    c/=2;
    c /= 2;
    c%=2;
    c %= 2;
    c<<=1;
    c <<= 1;
    c>>=1;
    c >>= 1;
    c>>>=1;
    c >>>= 1;
    c&=1;
    c &= 1;
    c|=1;
    c |= 1;
    c^=1;
    c ^= 1;
    c = -a;
    c = - a;
    c = +a;
    c = a++ + ++b;
    c = a++ +b;
    c = a - -b;
    c = a--- b;
    p = !p;
    c = ~a;
    c = p?a:b;
    c = p ? a : b;
    c = p ?a: b;
    c = p? a :b;
    c = (p)?a:b;
    c = a+b*c-d/e;
    c = a +b* c;
    l = (long)a+b;
    l = (long) a + b;
    l = (long)(a)+b;
    p = a<b&&b>a;
    p = a < b && b > a;
    p = a instanceof Integer;
    p = (Object) a instanceof Integer i&&i>0;
    p = (Object) a instanceof Integer i && i > 0;
    p = (Object) a instanceof Integer i&& i > 0;
  }
  void kw(boolean p, int a, List<String> list) throws Exception {
    if(p) {}
    if (p) {}
    if (p){}
    if (p) { a = 1; }else{ a = 2; }
    if (p) { a = 1; } else { a = 2; }
    if (p) { a = 1; }
    else { a = 2; }
    if (p) a = 1; else a = 2;
    if (p) a = 1;else a = 2;
    while(p) { a++; }
    while (p) { a++; }
    while (p){ a++; }
    while (p) {a++;}
    while (p) {}
    while (p) { }
    while (p) ;
    for(;;) { break; }
    for (;;) { break; }
    for (int i=0;i<2;i++) { a++; }
    for (int i = 0; i < 2; i++) { a++; }
    for (;;) {}
    for (String s:list) { a++; }
    for (String s : list) { a++; }
    for (String s :list) { a++; }
    for (String s: list) { a++; }
    for (String s : list) {}
    for (String s : list) ;
    do{ a++; }while(p);
    do { a++; } while (p);
    do { a++; }
    while (p);
    do a++; while (p);
    do {} while (p);
    do { } while (p);
    switch(a) { case 1: break; default: break; }
    switch (a) { case 1: break; default: break; }
    switch (a){ case 1: break; }
    switch (a) {
      case 1: { break; }
      case 2:{ break; }
      case 3: {}
      case 4: { }
      case 5:
        break;
      case 6: a++; break;
      case 7:a++; break;
      default:
        break;
    }
    switch (a) {
      case 1 -> a++;
      case 2->a++;
      case 3 ->a++;
      case 4-> a++;
      case 5 -> { a++; }
      case 6 -> {}
      case 7 -> { }
      case 8 ->{ a++; }
      default -> a--;
    }
    switch (a) {
      case 1, 2 -> a++;
      default->a--;
    }
    int r = switch (a) { case 1 -> 2; default -> 3; };
    int r2 = switch (a) {
      case 1: yield 2;
      default: yield 3;
    };
    int r3 = switch(a) { case 1 -> 2; default -> 3; };
    Object o = list;
    String sw = switch (o) {
      case String s when s.isEmpty() -> "e";
      case String s when(s.length() > 2) -> "l";
      case String s when s.length() > 3-> "m";
      case String s->"s";
      default -> "d";
    };
    synchronized(this) { a++; }
    synchronized (this) { a++; }
    synchronized (this){ a++; }
    synchronized (this) {}
    try{ a++; }catch(RuntimeException ex){ a--; }finally{ a++; }
    try { a++; } catch (RuntimeException ex) { a--; } finally { a++; }
    try { a++; }
    catch (RuntimeException ex) { a--; }
    finally { a++; }
    try { a++; } catch (RuntimeException ex) {}
    try { a++; } catch (RuntimeException ex) { }
    try { a++; } catch (RuntimeException|Error ex) { a--; }
    try { a++; } catch (RuntimeException | Error ex) { a--; }
    try (Scanner sc = new Scanner("")) { a++; }
    try(Scanner sc = new Scanner("")){ a++; }
    try (Scanner sc = new Scanner("");Scanner sc2 = new Scanner("")) { a++; }
    assert(p);
    assert p;
    assert p:"x";
    assert p : "x";
    assert p: "x";
    assert p :"x";
    assert (p) : "x";
    lbl:for (;;) { break lbl; }
    lbl2: for (;;) { break lbl2; }
    lbl3 :for (;;) { break lbl3; }
  }
  int ret1() { return(1); }
  int ret2() { return (1); }
  int ret3() { return 1; }
  void ret4() { return; }
  void ret5() { return ; }
  void ret6() {return;}
  Runnable l1 = ()->{};
  Runnable l2 = () -> {};
  Runnable l3 = () -> { };
  Runnable l4 = () -> {
  };
  Runnable l5 = ()->{ int x=1; };
  Runnable l6 = () ->{ int x = 1; };
  Runnable l7 = () ->System.out.println();
  Runnable l8 = () -> System.out.println();
  Function<Integer,Integer> l9 = x->x+1;
  Function<Integer, Integer> l10 = x -> x + 1;
  Function<Integer, Integer> l11 = (x)->x;
  Function<Integer, Integer> l12 = (Integer x)->{ return x; };
  Function<Integer, Integer> l13 = (Integer x) -> { return x; };
  BiFunction<Integer, Integer, Integer> l14 = (x,y) -> x+y;
  Supplier<List<String>> l15 = ArrayList<String>::new;
  Supplier<List<String>> l16 = ArrayList :: new;
  Supplier<Object> an1 = new Supplier<Object>() { public Object get() { return null; } };
  Supplier<Object> an2 = new Supplier<Object>(){ public Object get() { return null; } };
  Supplier<Object> an3 = new Supplier<Object>() {
    public Object get() { return null; }
  };
  Supplier<Object> an4 = new Supplier<Object>() {};
  Supplier<Object> an5 = new Supplier<Object>() { };
  Object dbl1 = new ArrayList<String>() {{ add("a"); }};
  Object dbl2 = new ArrayList<String>() { { add("a"); } };
  Object dbl3 = new ArrayList<String>(){{ add("a"); }};
  Object dbl4 = new ArrayList<String>() {{
    add("a");
  }};
  void empty1() {}
  void empty2() { }
  void empty3(){}
  void empty4() {
  }
  WarProbe() {}
  WarProbe(int x) { }
  WarProbe(long x){}
  WarProbe(char x) {
  }
  WarProbe(short x) { this(); }
  void m1() { { } }
  void m2() { {} }
  void m3() { { int z = 1; } }
  void m4() {;}
  void m5() { ; }
  static { }
  static {}
  static { int s = 1; }
  { }
  {}
  { int t = 1; }
  class Inner1 {}
  class Inner2 { }
  class Inner3{}
  class Inner4 {
  }
  class Inner5 { int x; }
  class Inner6{ int x; }
  interface I1 {}
  interface I2 { }
  interface I3{ void m(); }
  enum E1 {}
  enum E2 { }
  enum E3 { A, B }
  enum E4 {A, B}
  enum E5 { A { void m() {} }, B }
  enum E6{ A }
  @interface An1 {}
  @interface An2 { }
  record R1() {}
  record R2() { }
  record R3(int x) {}
  record R4(int x){}
  record R5(int x) { R5 { } }
  record R6(int x) { R6 {} }
  record R7(int x) { R7{ } }
  @SuppressWarnings({"a","b"})
  void ann1() {}
  @SuppressWarnings({ "a", "b" })
  void ann2() {}
  @SuppressWarnings({})
  void ann3() {}
  @SuppressWarnings( { "a" } )
  void ann4() {}
  void var1(String... xs) {}
  void var2(String ... xs) {}
  void var3(String...xs) {}
  void gen1() { List<String> a = new ArrayList<>(); }
  void gen2() { List<String> a = new ArrayList< >(); }
  void gen3() { Map<String,Integer> a = new HashMap<String,Integer>(); }
  void gen4() { Collections.<String>emptyList(); }
  void gen5() { Collections. <String>emptyList(); }
  <A,B> void gen6() {}
  <A, B> void gen7() {}
  <A> void gen8() {}
  <A>void gen9() {}
  void arr() {
    int[] a = new int[]{1, 2};
    int[] b = new int[] {1, 2};
    int[] c = new int[] { 1, 2 };
    int[] d = new int[] { };
    int[] e = new int[] {};
    int[][] f = new int[][]{{1}, {2}};
    int[][] g = {{1,2},{3}};
    int[][] h = { {1,2}, {3} };
    Object[] i = {new int[]{1}, new int[] {2}};
    Object[] j = { new int[] { 1 } };
  }
  void res() throws Exception {
    try (Scanner a1= new Scanner("")) { }
    try (Scanner a2 =new Scanner("")) { }
    try (Scanner a3=new Scanner("")) { }
  }
}
