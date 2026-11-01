package ok.pkg;

import java.util.List;

public class LcProbe<T> extends Base<T> {
    class C1 {
        int f;
    }
    class C2
    {
        int f;
    }
    class C3 { int f; }
    class C4 {}
    class C5 { }
    class C6 {
    }
    class C7 { // c
        int f;
    }
    class C8
    // c
    {
        int f;
    }
    class C9 { int f;
    }
    class C10 /* c */ {
        int f;
    }
    class C11
    { int f;
    }
    class C12 {

        int f;
    }
    class C13

    {
        int f;
    }
    class C14 {  
        int f;
    }
    class C15 { /* c */
        int f;
    }
    class C16	{
        int f;
    }
    class C17 {int f;
    }
    class C18
    {}
    class C19
    { }
    class C20 {
    }
    class C21 {  // c
    }

    interface I22 {
        void m();
    }
    interface I23
    {
        void m();
    }
    interface I24 { void m(); }
    interface I25 {}
    interface I26 { }
    interface I27 {
    }
    interface I28 { // c
        void m();
    }
    interface I29
    // c
    {
        void m();
    }
    interface I30 { void m();
    }
    interface I31 /* c */ {
        void m();
    }
    interface I32
    { void m();
    }
    interface I33 {

        void m();
    }
    interface I34

    {
        void m();
    }
    interface I35 {  
        void m();
    }
    interface I36 { /* c */
        void m();
    }
    interface I37	{
        void m();
    }
    interface I38 {void m();
    }
    interface I39
    {}
    interface I40
    { }
    interface I41 {
    }
    interface I42 {  // c
    }

    enum E43 {
        X, Y;
    }
    enum E44
    {
        X, Y;
    }
    enum E45 { X, Y; }
    enum E46 {}
    enum E47 { }
    enum E48 {
    }
    enum E49 { // c
        X, Y;
    }
    enum E50
    // c
    {
        X, Y;
    }
    enum E51 { X, Y;
    }
    enum E52 /* c */ {
        X, Y;
    }
    enum E53
    { X, Y;
    }
    enum E54 {

        X, Y;
    }
    enum E55

    {
        X, Y;
    }
    enum E56 {  
        X, Y;
    }
    enum E57 { /* c */
        X, Y;
    }
    enum E58	{
        X, Y;
    }
    enum E59 {X, Y;
    }
    enum E60
    {}
    enum E61
    { }
    enum E62 {
    }
    enum E63 {  // c
    }

    @interface A64 {
        int x();
    }
    @interface A65
    {
        int x();
    }
    @interface A66 { int x(); }
    @interface A67 {}
    @interface A68 { }
    @interface A69 {
    }
    @interface A70 { // c
        int x();
    }
    @interface A71
    // c
    {
        int x();
    }
    @interface A72 { int x();
    }
    @interface A73 /* c */ {
        int x();
    }
    @interface A74
    { int x();
    }
    @interface A75 {

        int x();
    }
    @interface A76

    {
        int x();
    }
    @interface A77 {  
        int x();
    }
    @interface A78 { /* c */
        int x();
    }
    @interface A79	{
        int x();
    }
    @interface A80 {int x();
    }
    @interface A81
    {}
    @interface A82
    { }
    @interface A83 {
    }
    @interface A84 {  // c
    }

    record R85(int a) {
        int g() { return 1; }
    }
    record R86(int a)
    {
        int g() { return 1; }
    }
    record R87(int a) { int g() { return 1; } }
    record R88(int a) {}
    record R89(int a) { }
    record R90(int a) {
    }
    record R91(int a) { // c
        int g() { return 1; }
    }
    record R92(int a)
    // c
    {
        int g() { return 1; }
    }
    record R93(int a) { int g() { return 1; }
    }
    record R94(int a) /* c */ {
        int g() { return 1; }
    }
    record R95(int a)
    { int g() { return 1; }
    }
    record R96(int a) {

        int g() { return 1; }
    }
    record R97(int a)

    {
        int g() { return 1; }
    }
    record R98(int a) {  
        int g() { return 1; }
    }
    record R99(int a) { /* c */
        int g() { return 1; }
    }
    record R100(int a)	{
        int g() { return 1; }
    }
    record R101(int a) {int g() { return 1; }
    }
    record R102(int a)
    {}
    record R103(int a)
    { }
    record R104(int a) {
    }
    record R105(int a) {  // c
    }

    @Deprecated enum F106 {
        X;
    }
    @Deprecated enum F107
    {
        X;
    }
    @Deprecated enum F108 { X; }
    @Deprecated enum F109 {}
    @Deprecated enum F110 { }
    @Deprecated enum F111 {
    }
    @Deprecated enum F112 { // c
        X;
    }
    @Deprecated enum F113
    // c
    {
        X;
    }
    @Deprecated enum F114 { X;
    }
    @Deprecated enum F115 /* c */ {
        X;
    }
    @Deprecated enum F116
    { X;
    }
    @Deprecated enum F117 {

        X;
    }
    @Deprecated enum F118

    {
        X;
    }
    @Deprecated enum F119 {  
        X;
    }
    @Deprecated enum F120 { /* c */
        X;
    }
    @Deprecated enum F121	{
        X;
    }
    @Deprecated enum F122 {X;
    }
    @Deprecated enum F123
    {}
    @Deprecated enum F124
    { }
    @Deprecated enum F125 {
    }
    @Deprecated enum F126 {  // c
    }

    @Deprecated
    enum FF127 {
        X;
    }
    @Deprecated
    enum FF128
    {
        X;
    }
    @Deprecated
    enum FF129 { X; }
    @Deprecated
    enum FF130 {}
    @Deprecated
    enum FF131 { }
    @Deprecated
    enum FF132 {
    }
    @Deprecated
    enum FF133 { // c
        X;
    }
    @Deprecated
    enum FF134
    // c
    {
        X;
    }
    @Deprecated
    enum FF135 { X;
    }
    @Deprecated
    enum FF136 /* c */ {
        X;
    }
    @Deprecated
    enum FF137
    { X;
    }
    @Deprecated
    enum FF138 {

        X;
    }
    @Deprecated
    enum FF139

    {
        X;
    }
    @Deprecated
    enum FF140 {  
        X;
    }
    @Deprecated
    enum FF141 { /* c */
        X;
    }
    @Deprecated
    enum FF142	{
        X;
    }
    @Deprecated
    enum FF143 {X;
    }
    @Deprecated
    enum FF144
    {}
    @Deprecated
    enum FF145
    { }
    @Deprecated
    enum FF146 {
    }
    @Deprecated
    enum FF147 {  // c
    }

    static final class G148 extends Base<T> implements Comparable<G148> {
        int f;
    }
    static final class G149 extends Base<T> implements Comparable<G149>
    {
        int f;
    }
    static final class G150 extends Base<T> implements Comparable<G150> { int f; }
    static final class G151 extends Base<T> implements Comparable<G151> {}
    static final class G152 extends Base<T> implements Comparable<G152> { }
    static final class G153 extends Base<T> implements Comparable<G153> {
    }
    static final class G154 extends Base<T> implements Comparable<G154> { // c
        int f;
    }
    static final class G155 extends Base<T> implements Comparable<G155>
    // c
    {
        int f;
    }
    static final class G156 extends Base<T> implements Comparable<G156> { int f;
    }
    static final class G157 extends Base<T> implements Comparable<G157> /* c */ {
        int f;
    }
    static final class G158 extends Base<T> implements Comparable<G158>
    { int f;
    }
    static final class G159 extends Base<T> implements Comparable<G159> {

        int f;
    }
    static final class G160 extends Base<T> implements Comparable<G160>

    {
        int f;
    }
    static final class G161 extends Base<T> implements Comparable<G161> {  
        int f;
    }
    static final class G162 extends Base<T> implements Comparable<G162> { /* c */
        int f;
    }
    static final class G163 extends Base<T> implements Comparable<G163>	{
        int f;
    }
    static final class G164 extends Base<T> implements Comparable<G164> {int f;
    }
    static final class G165 extends Base<T> implements Comparable<G165>
    {}
    static final class G166 extends Base<T> implements Comparable<G166>
    { }
    static final class G167 extends Base<T> implements Comparable<G167> {
    }
    static final class G168 extends Base<T> implements Comparable<G168> {  // c
    }

    void m169(int a) {
        x();
    }
    void m170(int a)
    {
        x();
    }
    void m171(int a) { x(); }
    void m172(int a) {}
    void m173(int a) { }
    void m174(int a) {
    }
    void m175(int a) { // c
        x();
    }
    void m176(int a)
    // c
    {
        x();
    }
    void m177(int a) { x();
    }
    void m178(int a) /* c */ {
        x();
    }
    void m179(int a)
    { x();
    }
    void m180(int a) {

        x();
    }
    void m181(int a)

    {
        x();
    }
    void m182(int a) {  
        x();
    }
    void m183(int a) { /* c */
        x();
    }
    void m184(int a)	{
        x();
    }
    void m185(int a) {x();
    }
    void m186(int a)
    {}
    void m187(int a)
    { }
    void m188(int a) {
    }
    void m189(int a) {  // c
    }

    void n190(int a,
            int b) {
        x();
    }
    void n191(int a,
            int b)
    {
        x();
    }
    void n192(int a,
            int b) { x(); }
    void n193(int a,
            int b) {}
    void n194(int a,
            int b) { }
    void n195(int a,
            int b) {
    }
    void n196(int a,
            int b) { // c
        x();
    }
    void n197(int a,
            int b)
    // c
    {
        x();
    }
    void n198(int a,
            int b) { x();
    }
    void n199(int a,
            int b) /* c */ {
        x();
    }
    void n200(int a,
            int b)
    { x();
    }
    void n201(int a,
            int b) {

        x();
    }
    void n202(int a,
            int b)

    {
        x();
    }
    void n203(int a,
            int b) {  
        x();
    }
    void n204(int a,
            int b) { /* c */
        x();
    }
    void n205(int a,
            int b)	{
        x();
    }
    void n206(int a,
            int b) {x();
    }
    void n207(int a,
            int b)
    {}
    void n208(int a,
            int b)
    { }
    void n209(int a,
            int b) {
    }
    void n210(int a,
            int b) {  // c
    }

    void o211() throws Exception {
        x();
    }
    void o212() throws Exception
    {
        x();
    }
    void o213() throws Exception { x(); }
    void o214() throws Exception {}
    void o215() throws Exception { }
    void o216() throws Exception {
    }
    void o217() throws Exception { // c
        x();
    }
    void o218() throws Exception
    // c
    {
        x();
    }
    void o219() throws Exception { x();
    }
    void o220() throws Exception /* c */ {
        x();
    }
    void o221() throws Exception
    { x();
    }
    void o222() throws Exception {

        x();
    }
    void o223() throws Exception

    {
        x();
    }
    void o224() throws Exception {  
        x();
    }
    void o225() throws Exception { /* c */
        x();
    }
    void o226() throws Exception	{
        x();
    }
    void o227() throws Exception {x();
    }
    void o228() throws Exception
    {}
    void o229() throws Exception
    { }
    void o230() throws Exception {
    }
    void o231() throws Exception {  // c
    }

    @Anno
    void p232() {
        x();
    }
    @Anno
    void p233()
    {
        x();
    }
    @Anno
    void p234() { x(); }
    @Anno
    void p235() {}
    @Anno
    void p236() { }
    @Anno
    void p237() {
    }
    @Anno
    void p238() { // c
        x();
    }
    @Anno
    void p239()
    // c
    {
        x();
    }
    @Anno
    void p240() { x();
    }
    @Anno
    void p241() /* c */ {
        x();
    }
    @Anno
    void p242()
    { x();
    }
    @Anno
    void p243() {

        x();
    }
    @Anno
    void p244()

    {
        x();
    }
    @Anno
    void p245() {  
        x();
    }
    @Anno
    void p246() { /* c */
        x();
    }
    @Anno
    void p247()	{
        x();
    }
    @Anno
    void p248() {x();
    }
    @Anno
    void p249()
    {}
    @Anno
    void p250()
    { }
    @Anno
    void p251() {
    }
    @Anno
    void p252() {  // c
    }

    @Anno public
    void pp253() {
        x();
    }
    @Anno public
    void pp254()
    {
        x();
    }
    @Anno public
    void pp255() { x(); }
    @Anno public
    void pp256() {}
    @Anno public
    void pp257() { }
    @Anno public
    void pp258() {
    }
    @Anno public
    void pp259() { // c
        x();
    }
    @Anno public
    void pp260()
    // c
    {
        x();
    }
    @Anno public
    void pp261() { x();
    }
    @Anno public
    void pp262() /* c */ {
        x();
    }
    @Anno public
    void pp263()
    { x();
    }
    @Anno public
    void pp264() {

        x();
    }
    @Anno public
    void pp265()

    {
        x();
    }
    @Anno public
    void pp266() {  
        x();
    }
    @Anno public
    void pp267() { /* c */
        x();
    }
    @Anno public
    void pp268()	{
        x();
    }
    @Anno public
    void pp269() {x();
    }
    @Anno public
    void pp270()
    {}
    @Anno public
    void pp271()
    { }
    @Anno public
    void pp272() {
    }
    @Anno public
    void pp273() {  // c
    }

    LcProbe(int a) {
        x();
    }
    LcProbe(int a)
    {
        x();
    }
    LcProbe(int a) { x(); }
    LcProbe(int a) {}
    LcProbe(int a) { }
    LcProbe(int a) {
    }
    LcProbe(int a) { // c
        x();
    }
    LcProbe(int a)
    // c
    {
        x();
    }
    LcProbe(int a) { x();
    }
    LcProbe(int a) /* c */ {
        x();
    }
    LcProbe(int a)
    { x();
    }
    LcProbe(int a) {

        x();
    }
    LcProbe(int a)

    {
        x();
    }
    LcProbe(int a) {  
        x();
    }
    LcProbe(int a) { /* c */
        x();
    }
    LcProbe(int a)	{
        x();
    }
    LcProbe(int a) {x();
    }
    LcProbe(int a)
    {}
    LcProbe(int a)
    { }
    LcProbe(int a) {
    }
    LcProbe(int a) {  // c
    }

    LcProbe(int a,
            int b) {
        x();
    }
    LcProbe(int a,
            int b)
    {
        x();
    }
    LcProbe(int a,
            int b) { x(); }
    LcProbe(int a,
            int b) {}
    LcProbe(int a,
            int b) { }
    LcProbe(int a,
            int b) {
    }
    LcProbe(int a,
            int b) { // c
        x();
    }
    LcProbe(int a,
            int b)
    // c
    {
        x();
    }
    LcProbe(int a,
            int b) { x();
    }
    LcProbe(int a,
            int b) /* c */ {
        x();
    }
    LcProbe(int a,
            int b)
    { x();
    }
    LcProbe(int a,
            int b) {

        x();
    }
    LcProbe(int a,
            int b)

    {
        x();
    }
    LcProbe(int a,
            int b) {  
        x();
    }
    LcProbe(int a,
            int b) { /* c */
        x();
    }
    LcProbe(int a,
            int b)	{
        x();
    }
    LcProbe(int a,
            int b) {x();
    }
    LcProbe(int a,
            int b)
    {}
    LcProbe(int a,
            int b)
    { }
    LcProbe(int a,
            int b) {
    }
    LcProbe(int a,
            int b) {  // c
    }

    static {
        x();
    }
    static
    {
        x();
    }
    static { x(); }
    static {}
    static { }
    static {
    }
    static { // c
        x();
    }
    static
    // c
    {
        x();
    }
    static { x();
    }
    static /* c */ {
        x();
    }
    static
    { x();
    }
    static {

        x();
    }
    static

    {
        x();
    }
    static {  
        x();
    }
    static { /* c */
        x();
    }
    static	{
        x();
    }
    static {x();
    }
    static
    {}
    static
    { }
    static {
    }
    static {  // c
    }

    <A> void q337(A a) {
        x();
    }
    <A> void q338(A a)
    {
        x();
    }
    <A> void q339(A a) { x(); }
    <A> void q340(A a) {}
    <A> void q341(A a) { }
    <A> void q342(A a) {
    }
    <A> void q343(A a) { // c
        x();
    }
    <A> void q344(A a)
    // c
    {
        x();
    }
    <A> void q345(A a) { x();
    }
    <A> void q346(A a) /* c */ {
        x();
    }
    <A> void q347(A a)
    { x();
    }
    <A> void q348(A a) {

        x();
    }
    <A> void q349(A a)

    {
        x();
    }
    <A> void q350(A a) {  
        x();
    }
    <A> void q351(A a) { /* c */
        x();
    }
    <A> void q352(A a)	{
        x();
    }
    <A> void q353(A a) {x();
    }
    <A> void q354(A a)
    {}
    <A> void q355(A a)
    { }
    <A> void q356(A a) {
    }
    <A> void q357(A a) {  // c
    }

    static final class H358
        extends Base<T>
        implements Comparable<H358> {
        int f;
    }
    static final class H359
        extends Base<T>
        implements Comparable<H359>
    {
        int f;
    }
    static final class H360
        extends Base<T>
        implements Comparable<H360> { int f; }
    static final class H361
        extends Base<T>
        implements Comparable<H361> {}
    static final class H362
        extends Base<T>
        implements Comparable<H362> { }
    static final class H363
        extends Base<T>
        implements Comparable<H363> {
    }
    static final class H364
        extends Base<T>
        implements Comparable<H364> { // c
        int f;
    }
    static final class H365
        extends Base<T>
        implements Comparable<H365>
    // c
    {
        int f;
    }
    static final class H366
        extends Base<T>
        implements Comparable<H366> { int f;
    }
    static final class H367
        extends Base<T>
        implements Comparable<H367> /* c */ {
        int f;
    }
    static final class H368
        extends Base<T>
        implements Comparable<H368>
    { int f;
    }
    static final class H369
        extends Base<T>
        implements Comparable<H369> {

        int f;
    }
    static final class H370
        extends Base<T>
        implements Comparable<H370>

    {
        int f;
    }
    static final class H371
        extends Base<T>
        implements Comparable<H371> {  
        int f;
    }
    static final class H372
        extends Base<T>
        implements Comparable<H372> { /* c */
        int f;
    }
    static final class H373
        extends Base<T>
        implements Comparable<H373>	{
        int f;
    }
    static final class H374
        extends Base<T>
        implements Comparable<H374> {int f;
    }
    static final class H375
        extends Base<T>
        implements Comparable<H375>
    {}
    static final class H376
        extends Base<T>
        implements Comparable<H376>
    { }
    static final class H377
        extends Base<T>
        implements Comparable<H377> {
    }
    static final class H378
        extends Base<T>
        implements Comparable<H378> {  // c
    }

    public static <A,
            B> Map<A,
            B> r379(int a) {
        x();
    }
    public static <A,
            B> Map<A,
            B> r380(int a)
    {
        x();
    }
    public static <A,
            B> Map<A,
            B> r381(int a) { x(); }
    public static <A,
            B> Map<A,
            B> r382(int a) {}
    public static <A,
            B> Map<A,
            B> r383(int a) { }
    public static <A,
            B> Map<A,
            B> r384(int a) {
    }
    public static <A,
            B> Map<A,
            B> r385(int a) { // c
        x();
    }
    public static <A,
            B> Map<A,
            B> r386(int a)
    // c
    {
        x();
    }
    public static <A,
            B> Map<A,
            B> r387(int a) { x();
    }
    public static <A,
            B> Map<A,
            B> r388(int a) /* c */ {
        x();
    }
    public static <A,
            B> Map<A,
            B> r389(int a)
    { x();
    }
    public static <A,
            B> Map<A,
            B> r390(int a) {

        x();
    }
    public static <A,
            B> Map<A,
            B> r391(int a)

    {
        x();
    }
    public static <A,
            B> Map<A,
            B> r392(int a) {  
        x();
    }
    public static <A,
            B> Map<A,
            B> r393(int a) { /* c */
        x();
    }
    public static <A,
            B> Map<A,
            B> r394(int a)	{
        x();
    }
    public static <A,
            B> Map<A,
            B> r395(int a) {x();
    }
    public static <A,
            B> Map<A,
            B> r396(int a)
    {}
    public static <A,
            B> Map<A,
            B> r397(int a)
    { }
    public static <A,
            B> Map<A,
            B> r398(int a) {
    }
    public static <A,
            B> Map<A,
            B> r399(int a) {  // c
    }

    @Anno
    @Anno2(1)
    static void s400() {
        x();
    }
    @Anno
    @Anno2(1)
    static void s401()
    {
        x();
    }
    @Anno
    @Anno2(1)
    static void s402() { x(); }
    @Anno
    @Anno2(1)
    static void s403() {}
    @Anno
    @Anno2(1)
    static void s404() { }
    @Anno
    @Anno2(1)
    static void s405() {
    }
    @Anno
    @Anno2(1)
    static void s406() { // c
        x();
    }
    @Anno
    @Anno2(1)
    static void s407()
    // c
    {
        x();
    }
    @Anno
    @Anno2(1)
    static void s408() { x();
    }
    @Anno
    @Anno2(1)
    static void s409() /* c */ {
        x();
    }
    @Anno
    @Anno2(1)
    static void s410()
    { x();
    }
    @Anno
    @Anno2(1)
    static void s411() {

        x();
    }
    @Anno
    @Anno2(1)
    static void s412()

    {
        x();
    }
    @Anno
    @Anno2(1)
    static void s413() {  
        x();
    }
    @Anno
    @Anno2(1)
    static void s414() { /* c */
        x();
    }
    @Anno
    @Anno2(1)
    static void s415()	{
        x();
    }
    @Anno
    @Anno2(1)
    static void s416() {x();
    }
    @Anno
    @Anno2(1)
    static void s417()
    {}
    @Anno
    @Anno2(1)
    static void s418()
    { }
    @Anno
    @Anno2(1)
    static void s419() {
    }
    @Anno
    @Anno2(1)
    static void s420() {  // c
    }

    @Anno static
    void t421(int a,
            int b) {
        x();
    }
    @Anno static
    void t422(int a,
            int b)
    {
        x();
    }
    @Anno static
    void t423(int a,
            int b) { x(); }
    @Anno static
    void t424(int a,
            int b) {}
    @Anno static
    void t425(int a,
            int b) { }
    @Anno static
    void t426(int a,
            int b) {
    }
    @Anno static
    void t427(int a,
            int b) { // c
        x();
    }
    @Anno static
    void t428(int a,
            int b)
    // c
    {
        x();
    }
    @Anno static
    void t429(int a,
            int b) { x();
    }
    @Anno static
    void t430(int a,
            int b) /* c */ {
        x();
    }
    @Anno static
    void t431(int a,
            int b)
    { x();
    }
    @Anno static
    void t432(int a,
            int b) {

        x();
    }
    @Anno static
    void t433(int a,
            int b)

    {
        x();
    }
    @Anno static
    void t434(int a,
            int b) {  
        x();
    }
    @Anno static
    void t435(int a,
            int b) { /* c */
        x();
    }
    @Anno static
    void t436(int a,
            int b)	{
        x();
    }
    @Anno static
    void t437(int a,
            int b) {x();
    }
    @Anno static
    void t438(int a,
            int b)
    {}
    @Anno static
    void t439(int a,
            int b)
    { }
    @Anno static
    void t440(int a,
            int b) {
    }
    @Anno static
    void t441(int a,
            int b) {  // c
    }

    @Anno
    @Deprecated enum FE442 {
        X;
    }
    @Anno
    @Deprecated enum FE443
    {
        X;
    }
    @Anno
    @Deprecated enum FE444 { X; }
    @Anno
    @Deprecated enum FE445 {}
    @Anno
    @Deprecated enum FE446 { }
    @Anno
    @Deprecated enum FE447 {
    }
    @Anno
    @Deprecated enum FE448 { // c
        X;
    }
    @Anno
    @Deprecated enum FE449
    // c
    {
        X;
    }
    @Anno
    @Deprecated enum FE450 { X;
    }
    @Anno
    @Deprecated enum FE451 /* c */ {
        X;
    }
    @Anno
    @Deprecated enum FE452
    { X;
    }
    @Anno
    @Deprecated enum FE453 {

        X;
    }
    @Anno
    @Deprecated enum FE454

    {
        X;
    }
    @Anno
    @Deprecated enum FE455 {  
        X;
    }
    @Anno
    @Deprecated enum FE456 { /* c */
        X;
    }
    @Anno
    @Deprecated enum FE457	{
        X;
    }
    @Anno
    @Deprecated enum FE458 {X;
    }
    @Anno
    @Deprecated enum FE459
    {}
    @Anno
    @Deprecated enum FE460
    { }
    @Anno
    @Deprecated enum FE461 {
    }
    @Anno
    @Deprecated enum FE462 {  // c
    }

    interface J463
        extends Runnable {
        void m();
    }
    interface J464
        extends Runnable
    {
        void m();
    }
    interface J465
        extends Runnable { void m(); }
    interface J466
        extends Runnable {}
    interface J467
        extends Runnable { }
    interface J468
        extends Runnable {
    }
    interface J469
        extends Runnable { // c
        void m();
    }
    interface J470
        extends Runnable
    // c
    {
        void m();
    }
    interface J471
        extends Runnable { void m();
    }
    interface J472
        extends Runnable /* c */ {
        void m();
    }
    interface J473
        extends Runnable
    { void m();
    }
    interface J474
        extends Runnable {

        void m();
    }
    interface J475
        extends Runnable

    {
        void m();
    }
    interface J476
        extends Runnable {  
        void m();
    }
    interface J477
        extends Runnable { /* c */
        void m();
    }
    interface J478
        extends Runnable	{
        void m();
    }
    interface J479
        extends Runnable {void m();
    }
    interface J480
        extends Runnable
    {}
    interface J481
        extends Runnable
    { }
    interface J482
        extends Runnable {
    }
    interface J483
        extends Runnable {  // c
    }

    record RR484(int a,
        int b) {
        int g() { return 1; }
    }
    record RR485(int a,
        int b)
    {
        int g() { return 1; }
    }
    record RR486(int a,
        int b) { int g() { return 1; } }
    record RR487(int a,
        int b) {}
    record RR488(int a,
        int b) { }
    record RR489(int a,
        int b) {
    }
    record RR490(int a,
        int b) { // c
        int g() { return 1; }
    }
    record RR491(int a,
        int b)
    // c
    {
        int g() { return 1; }
    }
    record RR492(int a,
        int b) { int g() { return 1; }
    }
    record RR493(int a,
        int b) /* c */ {
        int g() { return 1; }
    }
    record RR494(int a,
        int b)
    { int g() { return 1; }
    }
    record RR495(int a,
        int b) {

        int g() { return 1; }
    }
    record RR496(int a,
        int b)

    {
        int g() { return 1; }
    }
    record RR497(int a,
        int b) {  
        int g() { return 1; }
    }
    record RR498(int a,
        int b) { /* c */
        int g() { return 1; }
    }
    record RR499(int a,
        int b)	{
        int g() { return 1; }
    }
    record RR500(int a,
        int b) {int g() { return 1; }
    }
    record RR501(int a,
        int b)
    {}
    record RR502(int a,
        int b)
    { }
    record RR503(int a,
        int b) {
    }
    record RR504(int a,
        int b) {  // c
    }

    @Anno
    LcProbe(int a,
            int b) {
        x();
    }
    @Anno
    LcProbe(int a,
            int b)
    {
        x();
    }
    @Anno
    LcProbe(int a,
            int b) { x(); }
    @Anno
    LcProbe(int a,
            int b) {}
    @Anno
    LcProbe(int a,
            int b) { }
    @Anno
    LcProbe(int a,
            int b) {
    }
    @Anno
    LcProbe(int a,
            int b) { // c
        x();
    }
    @Anno
    LcProbe(int a,
            int b)
    // c
    {
        x();
    }
    @Anno
    LcProbe(int a,
            int b) { x();
    }
    @Anno
    LcProbe(int a,
            int b) /* c */ {
        x();
    }
    @Anno
    LcProbe(int a,
            int b)
    { x();
    }
    @Anno
    LcProbe(int a,
            int b) {

        x();
    }
    @Anno
    LcProbe(int a,
            int b)

    {
        x();
    }
    @Anno
    LcProbe(int a,
            int b) {  
        x();
    }
    @Anno
    LcProbe(int a,
            int b) { /* c */
        x();
    }
    @Anno
    LcProbe(int a,
            int b)	{
        x();
    }
    @Anno
    LcProbe(int a,
            int b) {x();
    }
    @Anno
    LcProbe(int a,
            int b)
    {}
    @Anno
    LcProbe(int a,
            int b)
    { }
    @Anno
    LcProbe(int a,
            int b) {
    }
    @Anno
    LcProbe(int a,
            int b) {  // c
    }

    {
        x();
    }
    { x(); }

    record RC(int a) {
        RC {
            x();
        }
        RC
        {
            x();
        }
        RC { x(); }
        RC {}
        RC { }
        RC {
        }
        RC { // c
            x();
        }
        RC
        // c
        {
            x();
        }
        RC { x();
        }
        RC /* c */ {
            x();
        }
        RC
        { x();
        }
        RC {

            x();
        }
        RC

        {
            x();
        }
        RC {  
            x();
        }
        RC { /* c */
            x();
        }
        RC	{
            x();
        }
        RC {x();
        }
        RC
        {}
        RC
        { }
        RC {
        }
        RC {  // c
        }
    }

    enum EC547 {
        A {
            int z;
        },
        B;
    }
    enum EC548 {
        A
        {
            int z;
        },
        B;
    }
    enum EC549 {
        A { int z; },
        B;
    }
    enum EC550 {
        A {},
        B;
    }
    enum EC551 {
        A { },
        B;
    }
    enum EC552 {
        A {
        },
        B;
    }
    enum EC553 {
        A { // c
            int z;
        },
        B;
    }
    enum EC554 {
        A
        // c
        {
            int z;
        },
        B;
    }
    enum EC555 {
        A { int z;
        },
        B;
    }
    enum EC556 {
        A /* c */ {
            int z;
        },
        B;
    }
    enum EC557 {
        A
        { int z;
        },
        B;
    }
    enum EC558 {
        A {

            int z;
        },
        B;
    }
    enum EC559 {
        A

        {
            int z;
        },
        B;
    }
    enum EC560 {
        A {  
            int z;
        },
        B;
    }
    enum EC561 {
        A { /* c */
            int z;
        },
        B;
    }
    enum EC562 {
        A	{
            int z;
        },
        B;
    }
    enum EC563 {
        A {int z;
        },
        B;
    }
    enum EC564 {
        A
        {},
        B;
    }
    enum EC565 {
        A
        { },
        B;
    }
    enum EC566 {
        A {
        },
        B;
    }
    enum EC567 {
        A {  // c
        },
        B;
    }

    void body(boolean a, boolean b, int[] qs, Object o) throws Exception {
        if (a) {
            x();
        }
        if (a)
        {
            x();
        }
        if (a) { x(); }
        if (a) {}
        if (a) { }
        if (a) {
        }
        if (a) { // c
            x();
        }
        if (a)
        // c
        {
            x();
        }
        if (a) { x();
        }
        if (a) /* c */ {
            x();
        }
        if (a)
        { x();
        }
        if (a) {

            x();
        }
        if (a)

        {
            x();
        }
        if (a) {  
            x();
        }
        if (a) { /* c */
            x();
        }
        if (a)	{
            x();
        }
        if (a) {x();
        }
        if (a)
        {}
        if (a)
        { }
        if (a) {
        }
        if (a) {  // c
        }

        if (a) x(); else {
            y();
        }
        if (a) x(); else
        {
            y();
        }
        if (a) x(); else { y(); }
        if (a) x(); else {}
        if (a) x(); else { }
        if (a) x(); else {
        }
        if (a) x(); else { // c
            y();
        }
        if (a) x(); else
        // c
        {
            y();
        }
        if (a) x(); else { y();
        }
        if (a) x(); else /* c */ {
            y();
        }
        if (a) x(); else
        { y();
        }
        if (a) x(); else {

            y();
        }
        if (a) x(); else

        {
            y();
        }
        if (a) x(); else {  
            y();
        }
        if (a) x(); else { /* c */
            y();
        }
        if (a) x(); else	{
            y();
        }
        if (a) x(); else {y();
        }
        if (a) x(); else
        {}
        if (a) x(); else
        { }
        if (a) x(); else {
        }
        if (a) x(); else {  // c
        }

        if (a) { x(); } else if (b) {
            y();
        }
        if (a) { x(); } else if (b)
        {
            y();
        }
        if (a) { x(); } else if (b) { y(); }
        if (a) { x(); } else if (b) {}
        if (a) { x(); } else if (b) { }
        if (a) { x(); } else if (b) {
        }
        if (a) { x(); } else if (b) { // c
            y();
        }
        if (a) { x(); } else if (b)
        // c
        {
            y();
        }
        if (a) { x(); } else if (b) { y();
        }
        if (a) { x(); } else if (b) /* c */ {
            y();
        }
        if (a) { x(); } else if (b)
        { y();
        }
        if (a) { x(); } else if (b) {

            y();
        }
        if (a) { x(); } else if (b)

        {
            y();
        }
        if (a) { x(); } else if (b) {  
            y();
        }
        if (a) { x(); } else if (b) { /* c */
            y();
        }
        if (a) { x(); } else if (b)	{
            y();
        }
        if (a) { x(); } else if (b) {y();
        }
        if (a) { x(); } else if (b)
        {}
        if (a) { x(); } else if (b)
        { }
        if (a) { x(); } else if (b) {
        }
        if (a) { x(); } else if (b) {  // c
        }

        if (a) { x(); } else if (b) { y(); } else {
            z();
        }
        if (a) { x(); } else if (b) { y(); } else
        {
            z();
        }
        if (a) { x(); } else if (b) { y(); } else { z(); }
        if (a) { x(); } else if (b) { y(); } else {}
        if (a) { x(); } else if (b) { y(); } else { }
        if (a) { x(); } else if (b) { y(); } else {
        }
        if (a) { x(); } else if (b) { y(); } else { // c
            z();
        }
        if (a) { x(); } else if (b) { y(); } else
        // c
        {
            z();
        }
        if (a) { x(); } else if (b) { y(); } else { z();
        }
        if (a) { x(); } else if (b) { y(); } else /* c */ {
            z();
        }
        if (a) { x(); } else if (b) { y(); } else
        { z();
        }
        if (a) { x(); } else if (b) { y(); } else {

            z();
        }
        if (a) { x(); } else if (b) { y(); } else

        {
            z();
        }
        if (a) { x(); } else if (b) { y(); } else {  
            z();
        }
        if (a) { x(); } else if (b) { y(); } else { /* c */
            z();
        }
        if (a) { x(); } else if (b) { y(); } else	{
            z();
        }
        if (a) { x(); } else if (b) { y(); } else {z();
        }
        if (a) { x(); } else if (b) { y(); } else
        {}
        if (a) { x(); } else if (b) { y(); } else
        { }
        if (a) { x(); } else if (b) { y(); } else {
        }
        if (a) { x(); } else if (b) { y(); } else {  // c
        }

        while (a) {
            x();
        }
        while (a)
        {
            x();
        }
        while (a) { x(); }
        while (a) {}
        while (a) { }
        while (a) {
        }
        while (a) { // c
            x();
        }
        while (a)
        // c
        {
            x();
        }
        while (a) { x();
        }
        while (a) /* c */ {
            x();
        }
        while (a)
        { x();
        }
        while (a) {

            x();
        }
        while (a)

        {
            x();
        }
        while (a) {  
            x();
        }
        while (a) { /* c */
            x();
        }
        while (a)	{
            x();
        }
        while (a) {x();
        }
        while (a)
        {}
        while (a)
        { }
        while (a) {
        }
        while (a) {  // c
        }

        for (;;) {
            x();
        }
        for (;;)
        {
            x();
        }
        for (;;) { x(); }
        for (;;) {}
        for (;;) { }
        for (;;) {
        }
        for (;;) { // c
            x();
        }
        for (;;)
        // c
        {
            x();
        }
        for (;;) { x();
        }
        for (;;) /* c */ {
            x();
        }
        for (;;)
        { x();
        }
        for (;;) {

            x();
        }
        for (;;)

        {
            x();
        }
        for (;;) {  
            x();
        }
        for (;;) { /* c */
            x();
        }
        for (;;)	{
            x();
        }
        for (;;) {x();
        }
        for (;;)
        {}
        for (;;)
        { }
        for (;;) {
        }
        for (;;) {  // c
        }

        for (int q : qs) {
            x();
        }
        for (int q : qs)
        {
            x();
        }
        for (int q : qs) { x(); }
        for (int q : qs) {}
        for (int q : qs) { }
        for (int q : qs) {
        }
        for (int q : qs) { // c
            x();
        }
        for (int q : qs)
        // c
        {
            x();
        }
        for (int q : qs) { x();
        }
        for (int q : qs) /* c */ {
            x();
        }
        for (int q : qs)
        { x();
        }
        for (int q : qs) {

            x();
        }
        for (int q : qs)

        {
            x();
        }
        for (int q : qs) {  
            x();
        }
        for (int q : qs) { /* c */
            x();
        }
        for (int q : qs)	{
            x();
        }
        for (int q : qs) {x();
        }
        for (int q : qs)
        {}
        for (int q : qs)
        { }
        for (int q : qs) {
        }
        for (int q : qs) {  // c
        }

        for (int q = 0;
                 q < 1; q++) {
            x();
        }
        for (int q = 0;
                 q < 1; q++)
        {
            x();
        }
        for (int q = 0;
                 q < 1; q++) { x(); }
        for (int q = 0;
                 q < 1; q++) {}
        for (int q = 0;
                 q < 1; q++) { }
        for (int q = 0;
                 q < 1; q++) {
        }
        for (int q = 0;
                 q < 1; q++) { // c
            x();
        }
        for (int q = 0;
                 q < 1; q++)
        // c
        {
            x();
        }
        for (int q = 0;
                 q < 1; q++) { x();
        }
        for (int q = 0;
                 q < 1; q++) /* c */ {
            x();
        }
        for (int q = 0;
                 q < 1; q++)
        { x();
        }
        for (int q = 0;
                 q < 1; q++) {

            x();
        }
        for (int q = 0;
                 q < 1; q++)

        {
            x();
        }
        for (int q = 0;
                 q < 1; q++) {  
            x();
        }
        for (int q = 0;
                 q < 1; q++) { /* c */
            x();
        }
        for (int q = 0;
                 q < 1; q++)	{
            x();
        }
        for (int q = 0;
                 q < 1; q++) {x();
        }
        for (int q = 0;
                 q < 1; q++)
        {}
        for (int q = 0;
                 q < 1; q++)
        { }
        for (int q = 0;
                 q < 1; q++) {
        }
        for (int q = 0;
                 q < 1; q++) {  // c
        }

        do {
            x();
        } while (a);
        do
        {
            x();
        } while (a);
        do { x(); } while (a);
        do {} while (a);
        do { } while (a);
        do {
        } while (a);
        do { // c
            x();
        } while (a);
        do
        // c
        {
            x();
        } while (a);
        do { x();
        } while (a);
        do /* c */ {
            x();
        } while (a);
        do
        { x();
        } while (a);
        do {

            x();
        } while (a);
        do

        {
            x();
        } while (a);
        do {  
            x();
        } while (a);
        do { /* c */
            x();
        } while (a);
        do	{
            x();
        } while (a);
        do {x();
        } while (a);
        do
        {} while (a);
        do
        { } while (a);
        do {
        } while (a);
        do {  // c
        } while (a);

        try {
            x();
        } finally { z(); }
        try
        {
            x();
        } finally { z(); }
        try { x(); } finally { z(); }
        try {} finally { z(); }
        try { } finally { z(); }
        try {
        } finally { z(); }
        try { // c
            x();
        } finally { z(); }
        try
        // c
        {
            x();
        } finally { z(); }
        try { x();
        } finally { z(); }
        try /* c */ {
            x();
        } finally { z(); }
        try
        { x();
        } finally { z(); }
        try {

            x();
        } finally { z(); }
        try

        {
            x();
        } finally { z(); }
        try {  
            x();
        } finally { z(); }
        try { /* c */
            x();
        } finally { z(); }
        try	{
            x();
        } finally { z(); }
        try {x();
        } finally { z(); }
        try
        {} finally { z(); }
        try
        { } finally { z(); }
        try {
        } finally { z(); }
        try {  // c
        } finally { z(); }

        try (Res r = open()) {
            x();
        } finally { z(); }
        try (Res r = open())
        {
            x();
        } finally { z(); }
        try (Res r = open()) { x(); } finally { z(); }
        try (Res r = open()) {} finally { z(); }
        try (Res r = open()) { } finally { z(); }
        try (Res r = open()) {
        } finally { z(); }
        try (Res r = open()) { // c
            x();
        } finally { z(); }
        try (Res r = open())
        // c
        {
            x();
        } finally { z(); }
        try (Res r = open()) { x();
        } finally { z(); }
        try (Res r = open()) /* c */ {
            x();
        } finally { z(); }
        try (Res r = open())
        { x();
        } finally { z(); }
        try (Res r = open()) {

            x();
        } finally { z(); }
        try (Res r = open())

        {
            x();
        } finally { z(); }
        try (Res r = open()) {  
            x();
        } finally { z(); }
        try (Res r = open()) { /* c */
            x();
        } finally { z(); }
        try (Res r = open())	{
            x();
        } finally { z(); }
        try (Res r = open()) {x();
        } finally { z(); }
        try (Res r = open())
        {} finally { z(); }
        try (Res r = open())
        { } finally { z(); }
        try (Res r = open()) {
        } finally { z(); }
        try (Res r = open()) {  // c
        } finally { z(); }

        try { x(); } catch (Exception e) {
            x();
        }
        try { x(); } catch (Exception e)
        {
            x();
        }
        try { x(); } catch (Exception e) { x(); }
        try { x(); } catch (Exception e) {}
        try { x(); } catch (Exception e) { }
        try { x(); } catch (Exception e) {
        }
        try { x(); } catch (Exception e) { // c
            x();
        }
        try { x(); } catch (Exception e)
        // c
        {
            x();
        }
        try { x(); } catch (Exception e) { x();
        }
        try { x(); } catch (Exception e) /* c */ {
            x();
        }
        try { x(); } catch (Exception e)
        { x();
        }
        try { x(); } catch (Exception e) {

            x();
        }
        try { x(); } catch (Exception e)

        {
            x();
        }
        try { x(); } catch (Exception e) {  
            x();
        }
        try { x(); } catch (Exception e) { /* c */
            x();
        }
        try { x(); } catch (Exception e)	{
            x();
        }
        try { x(); } catch (Exception e) {x();
        }
        try { x(); } catch (Exception e)
        {}
        try { x(); } catch (Exception e)
        { }
        try { x(); } catch (Exception e) {
        }
        try { x(); } catch (Exception e) {  // c
        }

        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)
        {
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) { x(); }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {}
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) { }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) { // c
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)
        // c
        {
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) { x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) /* c */ {
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)
        { x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {

            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)

        {
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {  
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) { /* c */
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)	{
            x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {x();
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)
        {}
        try { x(); } catch (IllegalStateException | IllegalArgumentException e)
        { }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {
        }
        try { x(); } catch (IllegalStateException | IllegalArgumentException e) {  // c
        }

        try { x(); } finally {
            x();
        }
        try { x(); } finally
        {
            x();
        }
        try { x(); } finally { x(); }
        try { x(); } finally {}
        try { x(); } finally { }
        try { x(); } finally {
        }
        try { x(); } finally { // c
            x();
        }
        try { x(); } finally
        // c
        {
            x();
        }
        try { x(); } finally { x();
        }
        try { x(); } finally /* c */ {
            x();
        }
        try { x(); } finally
        { x();
        }
        try { x(); } finally {

            x();
        }
        try { x(); } finally

        {
            x();
        }
        try { x(); } finally {  
            x();
        }
        try { x(); } finally { /* c */
            x();
        }
        try { x(); } finally	{
            x();
        }
        try { x(); } finally {x();
        }
        try { x(); } finally
        {}
        try { x(); } finally
        { }
        try { x(); } finally {
        }
        try { x(); } finally {  // c
        }

        synchronized (o) {
            x();
        }
        synchronized (o)
        {
            x();
        }
        synchronized (o) { x(); }
        synchronized (o) {}
        synchronized (o) { }
        synchronized (o) {
        }
        synchronized (o) { // c
            x();
        }
        synchronized (o)
        // c
        {
            x();
        }
        synchronized (o) { x();
        }
        synchronized (o) /* c */ {
            x();
        }
        synchronized (o)
        { x();
        }
        synchronized (o) {

            x();
        }
        synchronized (o)

        {
            x();
        }
        synchronized (o) {  
            x();
        }
        synchronized (o) { /* c */
            x();
        }
        synchronized (o)	{
            x();
        }
        synchronized (o) {x();
        }
        synchronized (o)
        {}
        synchronized (o)
        { }
        synchronized (o) {
        }
        synchronized (o) {  // c
        }

        switch (qs.length) {
            case 1: x(); break;
        }
        switch (qs.length)
        {
            case 1: x(); break;
        }
        switch (qs.length) { case 1: x(); break; }
        switch (qs.length) {}
        switch (qs.length) { }
        switch (qs.length) {
        }
        switch (qs.length) { // c
            case 1: x(); break;
        }
        switch (qs.length)
        // c
        {
            case 1: x(); break;
        }
        switch (qs.length) { case 1: x(); break;
        }
        switch (qs.length) /* c */ {
            case 1: x(); break;
        }
        switch (qs.length)
        { case 1: x(); break;
        }
        switch (qs.length) {

            case 1: x(); break;
        }
        switch (qs.length)

        {
            case 1: x(); break;
        }
        switch (qs.length) {  
            case 1: x(); break;
        }
        switch (qs.length) { /* c */
            case 1: x(); break;
        }
        switch (qs.length)	{
            case 1: x(); break;
        }
        switch (qs.length) {case 1: x(); break;
        }
        switch (qs.length)
        {}
        switch (qs.length)
        { }
        switch (qs.length) {
        }
        switch (qs.length) {  // c
        }

        int sv = switch (qs.length) {
            default -> 1;
        };
        int sv = switch (qs.length)
        {
            default -> 1;
        };
        int sv = switch (qs.length) { default -> 1; };
        int sv = switch (qs.length) {};
        int sv = switch (qs.length) { };
        int sv = switch (qs.length) {
        };
        int sv = switch (qs.length) { // c
            default -> 1;
        };
        int sv = switch (qs.length)
        // c
        {
            default -> 1;
        };
        int sv = switch (qs.length) { default -> 1;
        };
        int sv = switch (qs.length) /* c */ {
            default -> 1;
        };
        int sv = switch (qs.length)
        { default -> 1;
        };
        int sv = switch (qs.length) {

            default -> 1;
        };
        int sv = switch (qs.length)

        {
            default -> 1;
        };
        int sv = switch (qs.length) {  
            default -> 1;
        };
        int sv = switch (qs.length) { /* c */
            default -> 1;
        };
        int sv = switch (qs.length)	{
            default -> 1;
        };
        int sv = switch (qs.length) {default -> 1;
        };
        int sv = switch (qs.length)
        {};
        int sv = switch (qs.length)
        { };
        int sv = switch (qs.length) {
        };
        int sv = switch (qs.length) {  // c
        };

        Runnable r = () -> {
            x();
        };
        Runnable r = () ->
        {
            x();
        };
        Runnable r = () -> { x(); };
        Runnable r = () -> {};
        Runnable r = () -> { };
        Runnable r = () -> {
        };
        Runnable r = () -> { // c
            x();
        };
        Runnable r = () ->
        // c
        {
            x();
        };
        Runnable r = () -> { x();
        };
        Runnable r = () -> /* c */ {
            x();
        };
        Runnable r = () ->
        { x();
        };
        Runnable r = () -> {

            x();
        };
        Runnable r = () ->

        {
            x();
        };
        Runnable r = () -> {  
            x();
        };
        Runnable r = () -> { /* c */
            x();
        };
        Runnable r = () ->	{
            x();
        };
        Runnable r = () -> {x();
        };
        Runnable r = () ->
        {};
        Runnable r = () ->
        { };
        Runnable r = () -> {
        };
        Runnable r = () -> {  // c
        };

        Runnable r2 = (a, b) -> {
            x();
        };
        Runnable r2 = (a, b) ->
        {
            x();
        };
        Runnable r2 = (a, b) -> { x(); };
        Runnable r2 = (a, b) -> {};
        Runnable r2 = (a, b) -> { };
        Runnable r2 = (a, b) -> {
        };
        Runnable r2 = (a, b) -> { // c
            x();
        };
        Runnable r2 = (a, b) ->
        // c
        {
            x();
        };
        Runnable r2 = (a, b) -> { x();
        };
        Runnable r2 = (a, b) -> /* c */ {
            x();
        };
        Runnable r2 = (a, b) ->
        { x();
        };
        Runnable r2 = (a, b) -> {

            x();
        };
        Runnable r2 = (a, b) ->

        {
            x();
        };
        Runnable r2 = (a, b) -> {  
            x();
        };
        Runnable r2 = (a, b) -> { /* c */
            x();
        };
        Runnable r2 = (a, b) ->	{
            x();
        };
        Runnable r2 = (a, b) -> {x();
        };
        Runnable r2 = (a, b) ->
        {};
        Runnable r2 = (a, b) ->
        { };
        Runnable r2 = (a, b) -> {
        };
        Runnable r2 = (a, b) -> {  // c
        };

        Object an = new Object() {
            int z;
        };
        Object an = new Object()
        {
            int z;
        };
        Object an = new Object() { int z; };
        Object an = new Object() {};
        Object an = new Object() { };
        Object an = new Object() {
        };
        Object an = new Object() { // c
            int z;
        };
        Object an = new Object()
        // c
        {
            int z;
        };
        Object an = new Object() { int z;
        };
        Object an = new Object() /* c */ {
            int z;
        };
        Object an = new Object()
        { int z;
        };
        Object an = new Object() {

            int z;
        };
        Object an = new Object()

        {
            int z;
        };
        Object an = new Object() {  
            int z;
        };
        Object an = new Object() { /* c */
            int z;
        };
        Object an = new Object()	{
            int z;
        };
        Object an = new Object() {int z;
        };
        Object an = new Object()
        {};
        Object an = new Object()
        { };
        Object an = new Object() {
        };
        Object an = new Object() {  // c
        };

        Object an2 = new Base<String>(1,
            2) {
            int z;
        };
        Object an2 = new Base<String>(1,
            2)
        {
            int z;
        };
        Object an2 = new Base<String>(1,
            2) { int z; };
        Object an2 = new Base<String>(1,
            2) {};
        Object an2 = new Base<String>(1,
            2) { };
        Object an2 = new Base<String>(1,
            2) {
        };
        Object an2 = new Base<String>(1,
            2) { // c
            int z;
        };
        Object an2 = new Base<String>(1,
            2)
        // c
        {
            int z;
        };
        Object an2 = new Base<String>(1,
            2) { int z;
        };
        Object an2 = new Base<String>(1,
            2) /* c */ {
            int z;
        };
        Object an2 = new Base<String>(1,
            2)
        { int z;
        };
        Object an2 = new Base<String>(1,
            2) {

            int z;
        };
        Object an2 = new Base<String>(1,
            2)

        {
            int z;
        };
        Object an2 = new Base<String>(1,
            2) {  
            int z;
        };
        Object an2 = new Base<String>(1,
            2) { /* c */
            int z;
        };
        Object an2 = new Base<String>(1,
            2)	{
            int z;
        };
        Object an2 = new Base<String>(1,
            2) {int z;
        };
        Object an2 = new Base<String>(1,
            2)
        {};
        Object an2 = new Base<String>(1,
            2)
        { };
        Object an2 = new Base<String>(1,
            2) {
        };
        Object an2 = new Base<String>(1,
            2) {  // c
        };

        Object an3 = new Object() {
            { x(); }
        };
        Object an3 = new Object()
        {
            { x(); }
        };
        Object an3 = new Object() { { x(); } };
        Object an3 = new Object() {};
        Object an3 = new Object() { };
        Object an3 = new Object() {
        };
        Object an3 = new Object() { // c
            { x(); }
        };
        Object an3 = new Object()
        // c
        {
            { x(); }
        };
        Object an3 = new Object() { { x(); }
        };
        Object an3 = new Object() /* c */ {
            { x(); }
        };
        Object an3 = new Object()
        { { x(); }
        };
        Object an3 = new Object() {

            { x(); }
        };
        Object an3 = new Object()

        {
            { x(); }
        };
        Object an3 = new Object() {  
            { x(); }
        };
        Object an3 = new Object() { /* c */
            { x(); }
        };
        Object an3 = new Object()	{
            { x(); }
        };
        Object an3 = new Object() {{ x(); }
        };
        Object an3 = new Object()
        {};
        Object an3 = new Object()
        { };
        Object an3 = new Object() {
        };
        Object an3 = new Object() {  // c
        };

        Object an4 = new Object() {
            int z; { x(); }
        };
        Object an4 = new Object()
        {
            int z; { x(); }
        };
        Object an4 = new Object() { int z; { x(); } };
        Object an4 = new Object() {};
        Object an4 = new Object() { };
        Object an4 = new Object() {
        };
        Object an4 = new Object() { // c
            int z; { x(); }
        };
        Object an4 = new Object()
        // c
        {
            int z; { x(); }
        };
        Object an4 = new Object() { int z; { x(); }
        };
        Object an4 = new Object() /* c */ {
            int z; { x(); }
        };
        Object an4 = new Object()
        { int z; { x(); }
        };
        Object an4 = new Object() {

            int z; { x(); }
        };
        Object an4 = new Object()

        {
            int z; { x(); }
        };
        Object an4 = new Object() {  
            int z; { x(); }
        };
        Object an4 = new Object() { /* c */
            int z; { x(); }
        };
        Object an4 = new Object()	{
            int z; { x(); }
        };
        Object an4 = new Object() {int z; { x(); }
        };
        Object an4 = new Object()
        {};
        Object an4 = new Object()
        { };
        Object an4 = new Object() {
        };
        Object an4 = new Object() {  // c
        };

        if (a
                && b) {
            x();
        }
        if (a
                && b)
        {
            x();
        }
        if (a
                && b) { x(); }
        if (a
                && b) {}
        if (a
                && b) { }
        if (a
                && b) {
        }
        if (a
                && b) { // c
            x();
        }
        if (a
                && b)
        // c
        {
            x();
        }
        if (a
                && b) { x();
        }
        if (a
                && b) /* c */ {
            x();
        }
        if (a
                && b)
        { x();
        }
        if (a
                && b) {

            x();
        }
        if (a
                && b)

        {
            x();
        }
        if (a
                && b) {  
            x();
        }
        if (a
                && b) { /* c */
            x();
        }
        if (a
                && b)	{
            x();
        }
        if (a
                && b) {x();
        }
        if (a
                && b)
        {}
        if (a
                && b)
        { }
        if (a
                && b) {
        }
        if (a
                && b) {  // c
        }

        while (a &&
                b) {
            x();
        }
        while (a &&
                b)
        {
            x();
        }
        while (a &&
                b) { x(); }
        while (a &&
                b) {}
        while (a &&
                b) { }
        while (a &&
                b) {
        }
        while (a &&
                b) { // c
            x();
        }
        while (a &&
                b)
        // c
        {
            x();
        }
        while (a &&
                b) { x();
        }
        while (a &&
                b) /* c */ {
            x();
        }
        while (a &&
                b)
        { x();
        }
        while (a &&
                b) {

            x();
        }
        while (a &&
                b)

        {
            x();
        }
        while (a &&
                b) {  
            x();
        }
        while (a &&
                b) { /* c */
            x();
        }
        while (a &&
                b)	{
            x();
        }
        while (a &&
                b) {x();
        }
        while (a &&
                b)
        {}
        while (a &&
                b)
        { }
        while (a &&
                b) {
        }
        while (a &&
                b) {  // c
        }

        for (int q : qs) {
            x();
        }
        for (int q : qs)
        {
            x();
        }
        for (int q : qs) { x(); }
        for (int q : qs) {}
        for (int q : qs) { }
        for (int q : qs) {
        }
        for (int q : qs) { // c
            x();
        }
        for (int q : qs)
        // c
        {
            x();
        }
        for (int q : qs) { x();
        }
        for (int q : qs) /* c */ {
            x();
        }
        for (int q : qs)
        { x();
        }
        for (int q : qs) {

            x();
        }
        for (int q : qs)

        {
            x();
        }
        for (int q : qs) {  
            x();
        }
        for (int q : qs) { /* c */
            x();
        }
        for (int q : qs)	{
            x();
        }
        for (int q : qs) {x();
        }
        for (int q : qs)
        {}
        for (int q : qs)
        { }
        for (int q : qs) {
        }
        for (int q : qs) {  // c
        }

        switch (qs.length
                ) {
            default: x();
        }
        switch (qs.length
                )
        {
            default: x();
        }
        switch (qs.length
                ) { default: x(); }
        switch (qs.length
                ) {}
        switch (qs.length
                ) { }
        switch (qs.length
                ) {
        }
        switch (qs.length
                ) { // c
            default: x();
        }
        switch (qs.length
                )
        // c
        {
            default: x();
        }
        switch (qs.length
                ) { default: x();
        }
        switch (qs.length
                ) /* c */ {
            default: x();
        }
        switch (qs.length
                )
        { default: x();
        }
        switch (qs.length
                ) {

            default: x();
        }
        switch (qs.length
                )

        {
            default: x();
        }
        switch (qs.length
                ) {  
            default: x();
        }
        switch (qs.length
                ) { /* c */
            default: x();
        }
        switch (qs.length
                )	{
            default: x();
        }
        switch (qs.length
                ) {default: x();
        }
        switch (qs.length
                )
        {}
        switch (qs.length
                )
        { }
        switch (qs.length
                ) {
        }
        switch (qs.length
                ) {  // c
        }

        Runnable r3 = (a,
                b) -> {
            x();
        };
        Runnable r3 = (a,
                b) ->
        {
            x();
        };
        Runnable r3 = (a,
                b) -> { x(); };
        Runnable r3 = (a,
                b) -> {};
        Runnable r3 = (a,
                b) -> { };
        Runnable r3 = (a,
                b) -> {
        };
        Runnable r3 = (a,
                b) -> { // c
            x();
        };
        Runnable r3 = (a,
                b) ->
        // c
        {
            x();
        };
        Runnable r3 = (a,
                b) -> { x();
        };
        Runnable r3 = (a,
                b) -> /* c */ {
            x();
        };
        Runnable r3 = (a,
                b) ->
        { x();
        };
        Runnable r3 = (a,
                b) -> {

            x();
        };
        Runnable r3 = (a,
                b) ->

        {
            x();
        };
        Runnable r3 = (a,
                b) -> {  
            x();
        };
        Runnable r3 = (a,
                b) -> { /* c */
            x();
        };
        Runnable r3 = (a,
                b) ->	{
            x();
        };
        Runnable r3 = (a,
                b) -> {x();
        };
        Runnable r3 = (a,
                b) ->
        {};
        Runnable r3 = (a,
                b) ->
        { };
        Runnable r3 = (a,
                b) -> {
        };
        Runnable r3 = (a,
                b) -> {  // c
        };

        if (a) x(); else {
            y();
        }
        if (a) x(); else
        {
            y();
        }
        if (a) x(); else { y(); }
        if (a) x(); else {}
        if (a) x(); else { }
        if (a) x(); else {
        }
        if (a) x(); else { // c
            y();
        }
        if (a) x(); else
        // c
        {
            y();
        }
        if (a) x(); else { y();
        }
        if (a) x(); else /* c */ {
            y();
        }
        if (a) x(); else
        { y();
        }
        if (a) x(); else {

            y();
        }
        if (a) x(); else

        {
            y();
        }
        if (a) x(); else {  
            y();
        }
        if (a) x(); else { /* c */
            y();
        }
        if (a) x(); else	{
            y();
        }
        if (a) x(); else {y();
        }
        if (a) x(); else
        {}
        if (a) x(); else
        { }
        if (a) x(); else {
        }
        if (a) x(); else {  // c
        }

        try (Res r = open();
                Res q = open()) {
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open())
        {
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) { x(); } finally { z(); }
        try (Res r = open();
                Res q = open()) {} finally { z(); }
        try (Res r = open();
                Res q = open()) { } finally { z(); }
        try (Res r = open();
                Res q = open()) {
        } finally { z(); }
        try (Res r = open();
                Res q = open()) { // c
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open())
        // c
        {
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) { x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) /* c */ {
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open())
        { x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) {

            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open())

        {
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) {  
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) { /* c */
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open())	{
            x();
        } finally { z(); }
        try (Res r = open();
                Res q = open()) {x();
        } finally { z(); }
        try (Res r = open();
                Res q = open())
        {} finally { z(); }
        try (Res r = open();
                Res q = open())
        { } finally { z(); }
        try (Res r = open();
                Res q = open()) {
        } finally { z(); }
        try (Res r = open();
                Res q = open()) {  // c
        } finally { z(); }

        synchronized (o
                ) {
            x();
        }
        synchronized (o
                )
        {
            x();
        }
        synchronized (o
                ) { x(); }
        synchronized (o
                ) {}
        synchronized (o
                ) { }
        synchronized (o
                ) {
        }
        synchronized (o
                ) { // c
            x();
        }
        synchronized (o
                )
        // c
        {
            x();
        }
        synchronized (o
                ) { x();
        }
        synchronized (o
                ) /* c */ {
            x();
        }
        synchronized (o
                )
        { x();
        }
        synchronized (o
                ) {

            x();
        }
        synchronized (o
                )

        {
            x();
        }
        synchronized (o
                ) {  
            x();
        }
        synchronized (o
                ) { /* c */
            x();
        }
        synchronized (o
                )	{
            x();
        }
        synchronized (o
                ) {x();
        }
        synchronized (o
                )
        {}
        synchronized (o
                )
        { }
        synchronized (o
                ) {
        }
        synchronized (o
                ) {  // c
        }

         {
            x();
        }
                {
            x();
        }
         { x(); }
         {}
         { }
         {
        }
         { // c
            x();
        }
                // c
        {
            x();
        }
         { x();
        }
         /* c */ {
            x();
        }
                { x();
        }
         {

            x();
        }
                {
            x();
        }
         {  
            x();
        }
         { /* c */
            x();
        }
        	{
            x();
        }
         {x();
        }
                {}
                { }
         {
        }
         {  // c
        }

        switch (qs.length) {
            case 1: {
                x();
            }
        }
        switch (qs.length) {
            case 1:
            {
                x();
            }
        }
        switch (qs.length) {
            case 1: { x(); }
        }
        switch (qs.length) {
            case 1: {}
        }
        switch (qs.length) {
            case 1: { }
        }
        switch (qs.length) {
            case 1: {
            }
        }
        switch (qs.length) {
            case 1: { // c
                x();
            }
        }
        switch (qs.length) {
            case 1:
            // c
            {
                x();
            }
        }
        switch (qs.length) {
            case 1: { x();
            }
        }
        switch (qs.length) {
            case 1: /* c */ {
                x();
            }
        }
        switch (qs.length) {
            case 1:
            { x();
            }
        }
        switch (qs.length) {
            case 1: {

                x();
            }
        }
        switch (qs.length) {
            case 1:

            {
                x();
            }
        }
        switch (qs.length) {
            case 1: {  
                x();
            }
        }
        switch (qs.length) {
            case 1: { /* c */
                x();
            }
        }
        switch (qs.length) {
            case 1:	{
                x();
            }
        }
        switch (qs.length) {
            case 1: {x();
            }
        }
        switch (qs.length) {
            case 1:
            {}
        }
        switch (qs.length) {
            case 1:
            { }
        }
        switch (qs.length) {
            case 1: {
            }
        }
        switch (qs.length) {
            case 1: {  // c
            }
        }

        switch (qs.length) {
            default: {
                x();
            }
        }
        switch (qs.length) {
            default:
            {
                x();
            }
        }
        switch (qs.length) {
            default: { x(); }
        }
        switch (qs.length) {
            default: {}
        }
        switch (qs.length) {
            default: { }
        }
        switch (qs.length) {
            default: {
            }
        }
        switch (qs.length) {
            default: { // c
                x();
            }
        }
        switch (qs.length) {
            default:
            // c
            {
                x();
            }
        }
        switch (qs.length) {
            default: { x();
            }
        }
        switch (qs.length) {
            default: /* c */ {
                x();
            }
        }
        switch (qs.length) {
            default:
            { x();
            }
        }
        switch (qs.length) {
            default: {

                x();
            }
        }
        switch (qs.length) {
            default:

            {
                x();
            }
        }
        switch (qs.length) {
            default: {  
                x();
            }
        }
        switch (qs.length) {
            default: { /* c */
                x();
            }
        }
        switch (qs.length) {
            default:	{
                x();
            }
        }
        switch (qs.length) {
            default: {x();
            }
        }
        switch (qs.length) {
            default:
            {}
        }
        switch (qs.length) {
            default:
            { }
        }
        switch (qs.length) {
            default: {
            }
        }
        switch (qs.length) {
            default: {  // c
            }
        }

        switch (qs.length) {
            case 1 -> {
                x();
            }
        }
        switch (qs.length) {
            case 1 ->
            {
                x();
            }
        }
        switch (qs.length) {
            case 1 -> { x(); }
        }
        switch (qs.length) {
            case 1 -> {}
        }
        switch (qs.length) {
            case 1 -> { }
        }
        switch (qs.length) {
            case 1 -> {
            }
        }
        switch (qs.length) {
            case 1 -> { // c
                x();
            }
        }
        switch (qs.length) {
            case 1 ->
            // c
            {
                x();
            }
        }
        switch (qs.length) {
            case 1 -> { x();
            }
        }
        switch (qs.length) {
            case 1 -> /* c */ {
                x();
            }
        }
        switch (qs.length) {
            case 1 ->
            { x();
            }
        }
        switch (qs.length) {
            case 1 -> {

                x();
            }
        }
        switch (qs.length) {
            case 1 ->

            {
                x();
            }
        }
        switch (qs.length) {
            case 1 -> {  
                x();
            }
        }
        switch (qs.length) {
            case 1 -> { /* c */
                x();
            }
        }
        switch (qs.length) {
            case 1 ->	{
                x();
            }
        }
        switch (qs.length) {
            case 1 -> {x();
            }
        }
        switch (qs.length) {
            case 1 ->
            {}
        }
        switch (qs.length) {
            case 1 ->
            { }
        }
        switch (qs.length) {
            case 1 -> {
            }
        }
        switch (qs.length) {
            case 1 -> {  // c
            }
        }

        switch (qs.length) {
            default -> {
                x();
            }
        }
        switch (qs.length) {
            default ->
            {
                x();
            }
        }
        switch (qs.length) {
            default -> { x(); }
        }
        switch (qs.length) {
            default -> {}
        }
        switch (qs.length) {
            default -> { }
        }
        switch (qs.length) {
            default -> {
            }
        }
        switch (qs.length) {
            default -> { // c
                x();
            }
        }
        switch (qs.length) {
            default ->
            // c
            {
                x();
            }
        }
        switch (qs.length) {
            default -> { x();
            }
        }
        switch (qs.length) {
            default -> /* c */ {
                x();
            }
        }
        switch (qs.length) {
            default ->
            { x();
            }
        }
        switch (qs.length) {
            default -> {

                x();
            }
        }
        switch (qs.length) {
            default ->

            {
                x();
            }
        }
        switch (qs.length) {
            default -> {  
                x();
            }
        }
        switch (qs.length) {
            default -> { /* c */
                x();
            }
        }
        switch (qs.length) {
            default ->	{
                x();
            }
        }
        switch (qs.length) {
            default -> {x();
            }
        }
        switch (qs.length) {
            default ->
            {}
        }
        switch (qs.length) {
            default ->
            { }
        }
        switch (qs.length) {
            default -> {
            }
        }
        switch (qs.length) {
            default -> {  // c
            }
        }

        switch (qs.length) {
            case 1:
            case 2: {
                x();
            }
            case 3:
            case 4:
            {
                x();
            }
            case 5: x(); { y(); }
            case 6: y();
            {
                x();
            }
            default:
            {
                x();
            }
        }
        switch (qs.length) {
            case 1, 2 -> {
                x();
            }
            case 3, 4 ->
            {
                x();
            }
            case 5 -> x();
            default ->
            {
                x();
            }
        }
        switch (qs.length)
        {
            default: x();
        }
        switch (qs.length) {}
        switch (qs.length) { }
        switch (qs.length) { default: x(); }
        switch (qs.length) {
        }
        { x(); }
        {
            x();
        }
        label: { x(); }
        label2:
        {
            x();
        }
    }
}
