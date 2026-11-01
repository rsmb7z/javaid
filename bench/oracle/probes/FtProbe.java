package probes;

public class FtProbe {
    // CHECKSTYLE.SUPPRESS: FileTabCharacter for +2 lines
	int a = 1;   
	int b = 2;   
	int c = 3;   
    // CHECKSTYLE.SUPPRESS: LineLength for -1 lines
    String s1 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    String s2 = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    // SUPPRESS CHECKSTYLE LineLength
    String s3 = "cccccccccccccccccccccccccccccccccccccccccccccccccc";
    String s4 = "dddddddddddddddddddddddddddddddddddddddddddddddddd"; // SUPPRESS CHECKSTYLE LineLength
    // IGNORE UpperEll 1 Should
    long l1 = 1l;
    long l2 = 2l;
    long l3 = 3l;
    // IGNOREID noTrailingWhitespace
    int w1 = 1;   
    int w2 = 2;   
    int w3 = 3;   

    // OFF LineLength
    String s5 = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
    // ON LineLength
    String s6 = "ffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    // MSGOFF
    int x1 = 1;   
    // MSGON
    int x2 = 1;   
    // CHECKSTYLE:OFF
    long m1 = 1l;
    // CHECKSTYLE:ON
    long m2 = 1l;
    void Bad_Method() { int Bad_Local = 1; ; }
    String t1 = """
        ggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg
        hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh
        """;
    String t2 = "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii";
    // OFF UpperEll
    long n1 = 1l;
    // ON UpperEll
    // OFF LineLength
    // OFF UpperEll
    long n2 = 1l;
    // ON LineLength
    long n3 = 1l;
    String t3 = "jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj";
}
