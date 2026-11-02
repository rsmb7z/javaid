class CiProbe6 {
    void m() {
        foo();
          // reached twice in the tree
        int a = 1, b = 2;
        foo();
          /* block twice */
        int c = 1, d = 2, e = 3;
    }

    int x, y;
      // before field pair
    int z, w;

    void foo() { }
}
