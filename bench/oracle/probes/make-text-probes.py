#!/usr/bin/env python3
"""Generate the probe files for the text-level modules.

usage: make-text-probes.py [output-directory]    (default: the directory of this script)

Tabs, trailing spaces, form feeds and carriage returns are exactly what these probes test, and
editors and version control like to rewrite them, so the files are generated here and the generated
files are committed. Each file has one distinctive case per line; Checkstyle's answers for them are
recorded next to the file by `bench/oracle/record-golden.py`.
"""
import os
import sys

out = sys.argv[1] if len(sys.argv) > 1 else os.path.dirname(os.path.abspath(__file__))
T = "\t"


def write(name, text):
    with open(os.path.join(out, name), "w", encoding="utf-8", newline="") as fh:
        fh.write(text)


def lines(rows):
    return "\n".join(rows)


# --- FileTabCharacter: where does a tab count? ------------------------------------------------
write("TabProbe.java", lines([
    "package ok.pkg;",                                    # 1
    "",                                                    # 2
    "public class TabProbe {",                             # 3
    f"{T}int a;",                                          # 4  leading tab
    f"    int b;{T}",                                      # 5  trailing tab
    f"    int c = 1;{T}// comment after a tab",            # 6
    f'    String s = "a{T}b";',                            # 7  tab inside a string literal
    f"    /* block {T} comment */",                        # 8  tab inside a block comment
    "    /**",                                             # 9
    f"     *{T}javadoc tab",                               # 10
    "     */",                                              # 11
    '    String t = """',                                   # 12
    f"        text{T}block",                               # 13 tab inside a text block
    '        """;',                                         # 14
    f"    void f() {{}}{T}{T}",                            # 15 two tabs on one line
    "}",                                                    # 16
]) + "\n")

# --- LineLength: how is a line measured? (run with max=40) ----------------------------------
rows = [
    "package ok.pkg;",                                     # 1
    "",                                                     # 2
    "public class LineProbe {",                             # 3
    "    // " + "a" * 33,                                   # 4  exactly 40: fine
    "    // " + "a" * 34,                                   # 5  41
    "    // " + "a" * 33 + "   ",                           # 6  trailing spaces count
    T * 4 + "int x;",                                       # 7  4 tabs = 32, +6 = 38: fine
    T * 5 + "int x;",                                       # 8  5 tabs = 40, +6 = 46
    "x" * 33 + T,                                           # 9  a tab runs to the next multiple of the width: 40
    "x" * 33 + T + "y",                                     # 10 41
    "x" * 34 + T,                                           # 11 40
    "x" * 40 + T,                                           # 12 48
    "    // " + "\U0001F600" * 2 + "a" * 31,                 # 13 40 code points (42 UTF-16 units): fine
    "    // " + "\U0001F600" * 3 + "a" * 31,                 # 14 41 code points
    "    // " + "é" * 17,                              # 15 combining marks count: 41
    "    // " + "中" * 33,                               # 16 40
    "    // " + "中" * 34,                               # 17 41
    "package ok.pkg.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;",   # 18 ignored by the default pattern
    "import java.util.concurrent.atomic.AtomicReference;",  # 19 ignored
    "import static java.util.concurrent.TimeUnit.SECONDS;", # 20 ignored
    "importx java.util.concurrent.atomic.AtomicReference;", # 21 no space after the keyword: not ignored
    "  import java.util.concurrent.atomic.AtomicReference;", # 22 indented: not ignored
    "// import java.util.concurrent.atomic.AtomicReference;", # 23 a comment: not ignored
    "packagex ok.pkg.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;",  # 24
    "    // see http://example.com/a/very/long/url/x",       # 25 a URL (Google's pattern ignores it)
    '    String s = "' + "a" * 47 + '";',                    # 26
    "     * " + "word " * 12,                                # 27
    '    String t = """',                                    # 28
    "        " + "w" * 40,                                   # 29 inside a text block
    '        """;',                                          # 30
    "}",                                                     # 31 last line, no newline after it
]
write("LineProbe.java", lines(rows))

# --- RegexpSingleline: trailing whitespace and counting ---------------------------------------
write("TrailProbe.java", lines([
    "package ok.pkg;",                                     # 1
    "",                                                     # 2
    "public class TrailProbe {",                            # 3
    "    int a;  ",                                         # 4  trailing spaces
    f"    int b;{T}",                                       # 5  trailing tab
    "    int c;\f",                                         # 6  form feed is \s, and not a line break
    "    int d;\v",                                         # 7  so is vertical tab
    "    int e; ",                                     # 8  no-break space: Java's \s is ASCII only
    "    int f; ",                                     # 9  neither is an em space
    "      ",                                               # 10 a line of only spaces
    T,                                                      # 11 a line of only a tab
    "",                                                     # 12 an empty line
    "    int g;",                                           # 13 clean
    "    // note  ",                                        # 14 inside a comment
    '    String s = """',                                   # 15
    "        hello  ",                                      # 16 inside a text block
    '        """;',                                         # 17
    "    // TODO later",                                    # 18
    "    // todo later",                                    # 19
    "    // Todo later",                                    # 20
    "}  ",                                                  # 21 last line, no newline after it
]))

# --- TodoComment: which text of a comment does the pattern see? -----------------------------------
write("TodoProbe.java", lines([
    "package ok.pkg;",                                     # 1
    "",                                                     # 2
    "public class TodoProbe {",                             # 3
    "    // TODO: line comment with space",                 # 4
    "    //TODO: line comment no space",                    # 5
    "    // todo: lower case",                              # 6
    "    // TODO no colon",                                 # 7
    "    // Fix this TODO: mid-line",                       # 8
    "    int a; // TODO: trailing after code",              # 9
    "    /* TODO: block one-liner */",                      # 10
    "    /*TODO: block no space*/",                         # 11
    "    /** TODO: javadoc one-liner */",                   # 12
    "    /*",                                               # 13
    "     * TODO: block, star prefix",                      # 14
    "     */",                                              # 15
    "    /*",                                               # 16
    "       todo plain text line",                          # 17
    "         TODO: indented plain",                        # 18
    "     */",                                              # 19
    "    /**",                                              # 20
    "     * first line",                                    # 21
    "     * @param x TODO: tag line",                       # 22
    "     * todo: lower in javadoc",                        # 23
    "     */",                                              # 24
    '    String s = "TODO: inside a string";',              # 25
    "    char c = 'x'; /* TODO: after code */ int d;",      # 26
    "    /* TODO: a */ /* TODO: b */",                      # 27
    "    // TODO: first TODO: second",                      # 28
    "    /*",                                               # 29
    "     * TODO: one",                                     # 30
    "     * TODO: two",                                     # 31
    "     */",                                              # 32
    "    // =========== box line",                          # 33
    "    //===========",                                    # 34
    "    /*********",                                       # 35
    "     * box star",                                      # 36
    "     *********/",                                      # 37
    "    /* ========= */",                                  # 38
    "    // ######### hash",                                # 39
    "    // ********* stars",                               # 40
    "    // ========",                                      # 41
    "    // ==========",                                    # 42
    "    // one",                                           # 43
    "    /*",                                               # 44
    "     * TODO: x",                                       # 45
    "     * y",                                             # 46
    "     */",                                              # 47
    "}",                                                    # 48
]) + "\n")

# --- line terminators: which characters end a line? -----------------------------------------------
body = ["package ok.pkg;", "class {name} {{", f"{T}int a;", "    // TODO: x  ", "    /*", "     * TODO:", "     */", "}}"]


def with_terminator(name, term):
    return term.join(row.format(name=name) if "{name}" in row else row.replace("{{", "{").replace("}}", "}") for row in body) + term


write("CrlfProbe.java", with_terminator("CrlfProbe", "\r\n"))
write("CrProbe.java", with_terminator("CrProbe", "\r"))
write("FormFeedProbe.java", with_terminator("FormFeedProbe", "\f"))   # a form feed is not a line break
write("MixedProbe.java", "package ok.pkg;\r\nclass MixedProbe {\n" + f"{T}int a;\r" + "    int b;  \n}\r\n")

# --- non-Java files, which a Checker-level fileExtensions can pull in ------------------------------
write("Props.properties", lines([
    "# a properties file",                                  # 1
    "key=value  ",                                          # 2  trailing spaces
    f"other={T}tabbed",                                     # 3  tab
    "clean=yes",                                            # 4
    "very.long.key.that.is.well.over.the.forty.character.limit=value",   # 5  LineLength is limited to java
]) + "\n")
write("Doc.xml", lines([
    '<?xml version="1.0"?>',                                # 1
    "<root>  ",                                             # 2
    f"{T}<child/>",                                         # 3
    "</root>",                                              # 4
]) + "\n")
