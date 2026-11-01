package ok.pkg;

public class TabProbe {
	int a;
    int b;	
    int c = 1;	// comment after a tab
    String s = "a	b";
    /* block 	 comment */
    /**
     *	javadoc tab
     */
    String t = """
        text	block
        """;
    void f() {}		
}
