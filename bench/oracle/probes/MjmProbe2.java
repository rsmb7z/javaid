package p;

public class MjmProbe2<T> {
    @java.lang.Override public String toString() { return ""; }
    @Override public int hashCode() { return 1; }
    @ Override public boolean equals(Object o) { return true; }
    @Foo.Override public void fooOverride() { }
    @SuppressWarnings("x") public void sw() { }
    public <U> U getGeneric() { return null; }
    public T getT() { return null; }
    public int getArr()[] { return null; }
    public void setCompound(int x) { this.x += x; }
    public void setThis(int x) { this.x = x; }
    public void setTwo(int x) { this.x = x; return; }
    public void setNoBody(int x) { // comment
        this.x = x;
    }
    public void setVarargs(int... x) { this.x = x; }
    public void setGen(T t) { this.t = t; }
    public void setCall(int x) { foo(x); }
    public void setLocal(int x) { int y = x; }
    public void setChain(int x) { this.x = y = x; }
    public void setArr(int[] x) { this.x[0] = x[0]; }
    public void set(int x) { this.x = x; }
    public void setlower(int x) { this.x = x; }
    public void setX(final int x) { this.x = x; }
    public int getWithParam(int a) { return a; }
    public int getNested() { if (true) { return 1; } return 2; }
    public int getLambda() { return 1; // trailing
    }
    public int getCommentFirst() { /* c */ return 1; }
    public int getTwo() { return 1; }
    public String isString() { return ""; }
    public MjmProbe2(int a) { }
    public MjmProbe2() throws Exception { }
    <V> MjmProbe2(V v) { }
    /** doc */ @Deprecated public void both() { }
    /**/ public void tiny() { }
    // comment
    public void lineC() { }
    /* block */ public void blockC() { }
    /** one */ /** two */ public void twoDocs() { }
    @Deprecated /** misplaced */ public void misplaced() { }
    public /** misplaced2 */ void misplaced2() { }
    public void bodyLines() {
        // only a comment
    }
    public void bodyLines2() {

    }
    public void bodyLines3() {
        a();

    }
    public void sameLine() { a();
    }
    public void sameLine2() {
        a(); }
    public void semi();
    public abstract void abs2();
    public enum E {
        A { public void constantBody() { } };
        public abstract void ea();
    }
    public static class Gen<X> {
        public X get() { return null; }
    }
    public interface I<A> {
        A get();
        <B> B conv(A a);
    }
    public @interface Ann {
        int a();
        /** d */ int b();
        Class<?> c() default Object.class;
    }
}
