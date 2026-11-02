class ColumnProbe {
	// Tabs run to the next multiple of eight; an emoji is one column (two UTF-16 units); é is one.
	String s = "😀😀" ;	int x = 1;
	int y = 1; // 😀😀	 trailing tab
	String t = "😀😀"; int FooBar_baz = 2;
	String café = "é"; int Ünder_score = 3;
	void m() {
		if (x > 0) { x = "😀".length(); }
		if (x > 0)
		{
			x = 2; }
		int[] arr = { 1, 2 }; int z = arr [0];
	}
	java.util.List<String> qualified;
	Map.Entry<String, String> entry;
}
