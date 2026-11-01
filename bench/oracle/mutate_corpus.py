#!/usr/bin/env python3
"""Make a mutated copy of a Java source tree, to stress modules the original code never violates.

usage: mutate_corpus.py <source-dir> <destination-dir> [--seed N] [--rate 0.05]

Guava and Spring are tidy: at Checkstyle's defaults they contain one fall-through, two statements
sharing a line, no star imports and no lowercase `l` literal. A module that never fires on a
corpus has not been tested by it. So this copies every .java file and, at a fraction of the places
where it is possible, makes a small edit that stays valid Java:

  * UpperEll                       `10L`            -> `10l`
  * AvoidStarImport                `import a.b.C;`  -> `import a.b.*;`
  * ModifierOrder                  `public static`  -> `static public`, and the like
  * ArrayTypeStyle                 `int[] xs`       -> `int xs[]`
  * OneStatementPerLine,           two simple one-line statements are joined onto one line
    MultipleVariableDeclarations
  * FallThrough                    a `break;` on a line of its own is deleted
  * MissingSwitchDefault           a `default:` line is turned into a `case 0:` line
  * WhitespaceAround (only with --around)  the blank before, after, or on both sides of a binary operator,
    a `?` and `:`, and of a brace is removed
  * WhitespaceAfter (only with --after-punct)  the blank after `,` `;` `->` `...`, a cast, and `if` `for`
    `while` `catch` `switch` `try` `return` `synchronized` `else` `do` is removed
  * NoWhitespaceBefore (only with --before-punct)  a blank, or a line break, put before `,` `;` `.`
    `::` `++` `--` `...`
  * GenericWhitespace (only with --generics)  a blank put inside, before or after the angle brackets of a
    type or a type argument, a `>>` split, the blank removed after a method's type parameters, or one
    put before `.<`
  * OperatorWrap (only with --wrap)  an operator at the end of a line is moved to the start of the next, or
    one that starts a line to the end of the one before
  * SeparatorWrap (only with --separators)  a `,` `.` `::` or `...` at the end of a line is moved to the
    start of the next, or one that starts a line to the end of the one before
  * LeftCurly (only with --curly)  a `{` at the end of a line is moved to a line of its own, one on a line
    of its own is joined to the line before, or the statement after it is joined to its line
  * WhitespaceAfter on comments and annotations (only with --comments)  the blank after `//` is removed,
    or the one after an annotation with arguments that opens a type argument
  * MethodParamPad (only with --before-paren)  `foo(x)` -> `foo (x)`, or the `(` moved to a line of its own
  * NoWhitespaceAfter (only with --after-token)  a blank, or a line break, put after a `.`, `!`, unary `-`, prefix
    `++`, `@`, an array initialiser's `{`, or before a `[`
  * SimplifyBooleanExpression, SimplifyBooleanReturn (only with --bool)  a condition or return gets `== true`,
    `&& true` or `? true : false`, and `if (c) { return true; } return false;` becomes an if-else
  * RedundantImport, IllegalImport (only with --imports)  an import is repeated, or one from `java.lang`, the file's own
    package, `sun` or `java.io` is added
  * FinalClass, HideUtilityClassConstructor (only with --classes)  `final class` loses `final`, `static class` loses
    `static`, `abstract class` loses `abstract`, and a public or protected constructor becomes private or package-private
  * RightCurly (only with --rcurly)  `} else {` (or `} catch`, `} finally`) is split after the brace, or a `}` that
    starts a line is joined to the statement before it
  * EmptyBlock (only with --empty)  the only statement of a block is deleted, or replaced by a comment
  * NewlineAtEndOfFile (only with --eof)  a file's final line break is removed, doubled, or changed to CR or CRLF,
    or the whole file is converted to CRLF
  * InnerAssignment (only with --inner)  `x = y;` becomes `x = q = y;`, a call or `return` takes an assignment as
    its argument or value, and a condition is turned into a comparison of an assignment
  * AvoidNestedBlocks (only with --nest)  a statement that ends a line is wrapped in braces
  * EmptyStatement (only with --stray)  a second `;` put after a statement that ends a line
  * EmptyForIteratorPad (only with --for-pad)  a blank added after, or taken from, the `;` that ends a `for`
    header with no update
  * ParenPad (only with --parens)  `(x)` -> `( x )` on code lines, `foo(x)` -> `foo( x)`, `foo(x )`

Checkstyle and javaid are then run on the *same* mutated tree, so whatever they disagree about is a
real difference in how the two read code, found where it happens in real programs. Nothing here
needs to understand Java: a mutation is a regular-expression edit of one or two lines, deterministic
for a given seed, and a file Checkstyle cannot parse afterwards simply produces no output from it.
"""
import argparse
import os
import random
import re
import sys

MODIFIER_SWAPS = [
    (re.compile(r"\bpublic static\b"), "static public"),
    (re.compile(r"\bprivate static\b"), "static private"),
    (re.compile(r"\bprotected static\b"), "static protected"),
    (re.compile(r"\bpublic final\b"), "final public"),
    (re.compile(r"\bprivate final\b"), "final private"),
    (re.compile(r"\bpublic abstract\b"), "abstract public"),
    (re.compile(r"\bstatic final\b"), "final static"),
    (re.compile(r"\bpublic synchronized\b"), "synchronized public"),
]
ELL = re.compile(r"\b((?:0[xX][0-9a-fA-F_]+|\d[\d_]*))L\b")
STAR = re.compile(r"^(import (?!static )[\w.]+)\.[A-Z]\w*;\s*$")
ARRAY = re.compile(r"^(\s*(?:(?:final|private|public|protected|static|transient|volatile)\s+)*[A-Z]\w*|\s*(?:int|long|byte|char|short|double|float|boolean))\[\] (\w+)(\s*(?:=|;))")
SIMPLE_STATEMENT = re.compile(r"^\s+[\w.\[\]()<>\", +*/!&|?:=-]+;\s*$")
BREAK = re.compile(r"^\s*break;\s*$")
DEFAULT = re.compile(r"^(\s*)default:\s*$")


OPEN = re.compile(r"\((?=[^\s)])")
CLOSE = re.compile(r"(?<=[^\s(])\)")


def pad_parens(text, rng, rate):
    """Pad parentheses on code lines: sometimes both sides, sometimes one, at `rate` of the lines."""
    out = []
    for line in text.split("\n"):
        # A char literal such as ')' would turn into the invalid ' )', and Checkstyle abandons the whole run.
        if not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and rng.random() < rate * 4:
            how = rng.randrange(3)
            if how in (0, 1):
                line = OPEN.sub("( ", line)
            if how in (0, 2):
                line = CLOSE.sub(" )", line)
        out.append(line)
    return "\n".join(out)


CALL = re.compile(r"(\w)\(")


def pad_before_paren(text, rng, rate):
    """Put a blank, or a line break, between a name and the `(` after it, on `rate` of the code lines."""
    out = []
    for line in text.split("\n"):
        # A line break inside a trailing comment or a string literal would break the file, so those lines are left alone.
        if not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and "//" not in line and "/*" not in line and '"' not in line and rng.random() < rate * 4:
            indent = line[: len(line) - len(line.lstrip())]
            how = rng.randrange(3)
            if how == 0:
                line = CALL.sub(r"\1 (", line, count=1)
            elif how == 1:
                line = CALL.sub(lambda m: m.group(1) + "\n" + indent + "    (", line, count=1)
            else:
                line = CALL.sub(r"\1\t(", line)
        out.append(line)
    return "\n".join(out)


PUNCT = [re.compile(p) for p in (r"(?<=\S),", r"(?<=\S);", r"(?<=[A-Za-z_)\]])\.(?=[A-Za-z_])", r"(?<=\w)::", r"(?<=\w)\+\+", r"(?<=\w)--", r"(?<=\w)\.\.\.")]


def pad_before_punct(text, rng, rate):
    """Put a blank, or a line break, before a punctuation token, on `rate` of the code lines."""
    out = []
    for line in text.split("\n"):
        if (not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and "//" not in line and "/*" not in line
                and '"' not in line and rng.random() < rate * 4):
            indent = line[: len(line) - len(line.lstrip())]
            pattern = rng.choice(PUNCT)
            if rng.random() < 0.6:
                line = pattern.sub(lambda m: " " + m.group(0), line, count=1)
            else:
                line = pattern.sub(lambda m: "\n" + indent + "    " + m.group(0), line, count=1)
        out.append(line)
    return "\n".join(out)


AFTER = [
    (re.compile(r", (?=\S)"), ","),
    (re.compile(r"; (?=\S)"), ";"),
    (re.compile(r" -> (?=\S)"), " ->"),
    (re.compile(r"(?<=\w)\.\.\. "), "..."),
    (re.compile(r"(\((?:int|long|double|float|char|byte|short|String|Object|[A-Z]\w*)\)) (?=[\w(])"), r"\1"),
    (re.compile(r"\b(if|for|while|catch|switch|synchronized|try) \("), r"\1("),
    (re.compile(r"\b(return) (?=\()"), r"\1"),
    (re.compile(r"\belse \{"), "else{"),
    (re.compile(r"\bdo \{"), "do{"),
    (re.compile(r"\bfinally \{"), "finally{"),
]


def pad_after_punct(text, rng, rate):
    """Remove the blank after a token that should be followed by one, on `rate` of the code lines."""
    out = []
    for line in text.split("\n"):
        if (not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and "//" not in line and "/*" not in line
                and '"' not in line and rng.random() < rate * 4):
            pattern, repl = rng.choice(AFTER)
            line = pattern.sub(repl, line, count=1)
        out.append(line)
    return "\n".join(out)


_OPS = r"(?:<<=|>>>=|>>=|>>>|<<|>>|&&|\|\||==|!=|<=|>=|[-+*/%&|^]=|=|[-+*/%&|^<>])"
AROUND = [
    (re.compile(r"(?<=[\w)\]]) (" + _OPS + r") (?=[\w(])"), r"\1 "),
    (re.compile(r"(?<=[\w)\]]) (" + _OPS + r") (?=[\w(])"), r" \1"),
    (re.compile(r"(?<=[\w)\]]) (" + _OPS + r") (?=[\w(])"), r"\1"),
    (re.compile(r"\) \{"), "){"),
    (re.compile(r"\{ (?=\S)"), "{"),
    (re.compile(r"(?<=\S) \}"), "}"),
    (re.compile(r"\} (?=else|catch|finally|while)"), "}"),
    (re.compile(r"\b(else|try|finally|do) \{"), r"\1{"),
]
TERNARY = re.compile(r" \? (.*?) : ")


def pad_around(text, rng, rate):
    """Take away a blank next to an operator or a brace, on `rate` of the code lines."""
    out = []
    for line in text.split("\n"):
        if (not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and "//" not in line and "/*" not in line
                and '"' not in line and rng.random() < rate * 4):
            if TERNARY.search(line) and rng.random() < 0.3:
                line = TERNARY.sub(lambda m: rng.choice(["?%s : ", " ? %s:", "?%s:"]) % m.group(1), line, count=1)
            else:
                pattern, repl = rng.choice(AROUND)
                line = pattern.sub(repl, line, count=1)
        out.append(line)
    return "\n".join(out)


GENERICS = [
    (re.compile(r"\b([A-Z]\w*)<(?=[\w?@])"), r"\1 <"),
    (re.compile(r"\b([A-Z]\w*)<(?=[\w?@])"), r"\1< "),
    (re.compile(r"(?<=\w)>(?=[ ,)(.;\[])"), " >"),
    (re.compile(r"(?<=\w)> (?=\w)"), ">"),
    (re.compile(r"(?<=\w)>>(?=[ )(,;.])"), "> >"),
    (re.compile(r"\b(public|private|protected|static|final|default|abstract|synchronized) <"), r"\1<"),
    (re.compile(r"(?<=[ (])<(?=[A-Z]\w*[ ,>])"), "< "),
    (re.compile(r"\.<"), ". <"),
    (re.compile(r"\bnew (\w+)<>"), r"new \1< >"),
]


def pad_generics(text, rng, rate):
    """Put a blank in, or take one out, next to an angle bracket, on `rate` of the code lines."""
    out = []
    for line in text.split("\n"):
        if (not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and "//" not in line and "/*" not in line
                and '"' not in line and rng.random() < rate * 4):
            pattern, repl = rng.choice(GENERICS)
            line = pattern.sub(repl, line, count=1)
        out.append(line)
    return "\n".join(out)


_WRAP_OPS = (r"instanceof|->|::|&&|\|\||==|!=|<=|>=|[-+*/%&|^]=|<<=|>>=|>>>=|>>>|<<|>>|=|[-+*/%&|^<>?:]")
WRAP_END = re.compile(r"^(?P<head>\s*\S.*[\w)\]]) (?P<op>" + _WRAP_OPS + r")$")
WRAP_START = re.compile(r"^(?P<indent>\s+)(?P<op>" + _WRAP_OPS + r") (?=\S)")
WRAP_KEYWORD_END = re.compile(r"\b(return|else|do|try|finally|case|default|throw|new|yield|assert)$")
WRAP_OPERAND_END = re.compile(r"[\w)\]]$")


def _wrap_code(line):
    s = line.strip()
    return s != "" and not s.startswith(("*", "//", "/*", "@", "}")) and not any(c in line for c in "'\"") and "//" not in line and "/*" not in line and "*/" not in line


def wrap_operators(text, rng, rate):
    """Move an operator across a line break, on `rate` of the places where one can go."""
    lines = text.split("\n")
    in_block = False
    skip = []
    for line in lines:
        # A line inside a block comment, or one that opens or closes one, is left alone.
        opens = line.count("/*")
        closes = line.count("*/")
        skip.append(in_block or opens > 0 or closes > 0)
        in_block = (in_block or opens > 0) and not (closes > 0 and line.rfind("*/") > line.rfind("/*"))
    for i in range(len(lines) - 1):
        a, b = lines[i], lines[i + 1]
        if skip[i] or skip[i + 1] or not _wrap_code(a) or not _wrap_code(b) or rng.random() >= rate * 4:
            continue
        m = WRAP_END.match(a)
        if m and not WRAP_KEYWORD_END.search(m.group("head")):
            lines[i] = m.group("head")
            lines[i + 1] = re.sub(r"^(\s*)", lambda g: g.group(1) + m.group("op") + " ", b, count=1)
            continue
        m = WRAP_START.match(b)
        if m and WRAP_OPERAND_END.search(a) and not WRAP_KEYWORD_END.search(a):
            lines[i] = a + " " + m.group("op")
            lines[i + 1] = m.group("indent") + b[m.end():]
    return "\n".join(lines)


SEP_END = re.compile(r"^(?P<head>\s*\S.*[\w)\]>])(?P<op>,|\.|::)$")
SEP_START = re.compile(r"^(?P<indent>\s+)(?P<op>,|::|\.(?![.\d]))(?=\S|$)")


def wrap_separators(text, rng, rate):
    """Move a separator across a line break, on `rate` of the places where one can go."""
    lines = text.split("\n")
    in_block = False
    skip = []
    for line in lines:
        opens = line.count("/*")
        closes = line.count("*/")
        skip.append(in_block or opens > 0 or closes > 0)
        in_block = (in_block or opens > 0) and not (closes > 0 and line.rfind("*/") > line.rfind("/*"))
    for i in range(len(lines) - 1):
        a, b = lines[i], lines[i + 1]
        if skip[i] or skip[i + 1] or not _wrap_code(a) or not _wrap_code(b) or rng.random() >= rate * 4:
            continue
        m = SEP_END.match(a)
        if m and not WRAP_KEYWORD_END.search(m.group("head")):
            lines[i] = m.group("head")
            lines[i + 1] = re.sub(r"^(\s*)", lambda g: g.group(1) + m.group("op") + (" " if m.group("op") == "," else ""), b, count=1)
            continue
        m = SEP_START.match(b)
        if m and WRAP_OPERAND_END.search(a) and not WRAP_KEYWORD_END.search(a):
            lines[i] = a + m.group("op")
            lines[i + 1] = m.group("indent") + b[m.end():].lstrip()
    return "\n".join(lines)


def wrap_curlies(text, rng, rate):
    """Move a `{` to or from a line of its own, or join the first statement to its line."""
    lines = text.split("\n")
    in_block = False
    skip = []
    for line in lines:
        opens = line.count("/*")
        closes = line.count("*/")
        skip.append(in_block or opens > 0 or closes > 0)
        in_block = (in_block or opens > 0) and not (closes > 0 and line.rfind("*/") > line.rfind("/*"))
    out = []
    i = 0
    while i < len(lines):
        a = lines[i]
        b = lines[i + 1] if i + 1 < len(lines) else None
        if skip[i] or not _wrap_code(a) or rng.random() >= rate * 4:
            out.append(a)
            i += 1
            continue
        indent = a[: len(a) - len(a.lstrip())]
        if a.endswith(" {") and not a.lstrip().startswith(("{", "case ", "default")) and rng.random() < 0.5:
            out.append(a[:-2].rstrip())
            out.append(indent + "{")
        elif a.strip() == "{" and out and _wrap_code(out[-1]) and not skip[i - 1] and not out[-1].rstrip().endswith((",", ";", "{", "}")):
            out[-1] = out[-1].rstrip() + " {"
        elif a.endswith(" {") and b is not None and not skip[i + 1] and _wrap_code(b) and b.strip().endswith(";") and not b.strip().startswith("}"):
            out.append(a + " " + b.strip())
            i += 1
        else:
            out.append(a)
        i += 1
    return "\n".join(out)


LINE_COMMENT = re.compile(r"(?<![:\"'])//(/*) (?=\S)")
TYPE_ANNOTATION = re.compile(r"(<|, )(@\w+\([^()]*\)) (?=[A-Z])")


def tighten_comments(text, rng, rate):
    """Take the blank out after `//`, and after an annotation with arguments inside type arguments."""
    out = []
    for line in text.split("\n"):
        if rng.random() < rate * 4:
            if LINE_COMMENT.search(line) and '"' not in line:
                line = LINE_COMMENT.sub(lambda m: "//" + m.group(1), line, count=1)
            elif TYPE_ANNOTATION.search(line):
                line = TYPE_ANNOTATION.sub(lambda m: m.group(1) + m.group(2), line, count=1)
        out.append(line)
    return "\n".join(out)


STRAY_END = re.compile(r"^(?P<head>\s+[^;]*[\w)\]\"']);$")


AFTER_TOKEN = [
    re.compile(r"(?<=[A-Za-z_)\]])\.(?=[A-Za-z_])"),
    re.compile(r"(?<![\w)\]=!])!(?=[\w(])"),
    re.compile(r"(?<=[=(,] )-(?=[\w(])"),
    re.compile(r"(?<![\w)\]+])\+\+(?=\w)"),
    re.compile(r"@(?=[A-Z])"),
    re.compile(r"(?<== )\{(?=\S)"),
    re.compile(r"(?<=\w)(?=\[)"),
]


def pad_after_tokens(text, rng, rate):
    """Put a blank, or a line break, after a token that should have none, on `rate * 4` of the code lines."""
    out = []
    for line in text.split("\n"):
        if (not line.lstrip().startswith(("*", "//", "/*")) and "'" not in line and "//" not in line and "/*" not in line
                and '"' not in line and rng.random() < rate * 4):
            indent = line[: len(line) - len(line.lstrip())]
            pattern = rng.choice(AFTER_TOKEN)
            how = rng.random()
            gap = " " if how < 0.6 else ("  " if how < 0.75 else "\n" + indent + "    ")
            line = pattern.sub(lambda m: m.group(0) + gap, line, count=1)
        out.append(line)
    return "\n".join(out)


FOR_NO_UPDATE = re.compile(r"(\bfor \([^()]*;)( ?)\)")


def pad_for_iterators(text, rng, rate):
    """Toggle the blank in `for (...; ...; )` headers, on `rate * 8` of the lines that have one."""
    out = []
    for line in text.split("\n"):
        if FOR_NO_UPDATE.search(line) and '"' not in line and "'" not in line and rng.random() < rate * 8:
            blank = rng.choice(["", " ", "  ", "\t"])
            line = FOR_NO_UPDATE.sub(lambda m: m.group(1) + blank + ")", line, count=1)
        out.append(line)
    return "\n".join(out)


MEMBER_START = re.compile(r"^\s*(?:@|(?:public|private|protected|static|final|abstract|default|native|synchronized|transient|volatile)\b|(?:super|this)\(|(?:throws|extends|implements|permits|when|while)\b)")
DECLARATION = re.compile(r"^\s*[\w<>\[\].,? ]+\s\w+\(.*\);\s*$")


TYPE_HEADER = re.compile(r"\b(?:class|interface|enum|record)\b|\bnew\s+[\w.<>,? @]+\([^;]*\)\s*$")


def code_lines(text):
    """For each line, whether it starts inside a method or initialiser body (so that it holds statements),
    not directly in a type body. A crude scan: strings, chars and comments are skipped, and a `{` whose
    header names a class, interface, enum, record or `new X(...)` opens a type body."""
    stack = []  # True for a block of statements, False for a type body
    header = ""
    state = []
    i, n = 0, len(text)
    in_block = False
    state.append(False)
    while i < n:
        c = text[i]
        if c == "\n":
            state.append(bool(stack) and stack[-1])
            header += " "
            i += 1
            continue
        if in_block:
            if text.startswith("*/", i):
                in_block = False
                i += 2
            else:
                i += 1
            continue
        if text.startswith("/*", i):
            in_block = True
            i += 2
        elif text.startswith("//", i):
            while i < n and text[i] != "\n":
                i += 1
        elif c in "\"'":
            quote = c
            i += 1
            while i < n and text[i] != quote and text[i] != "\n":
                i += 2 if text[i] == "\\" else 1
            i += 1
            header += "x"
        elif c == "{":
            stack.append(False if TYPE_HEADER.search(header) else True)
            header = ""
            i += 1
        elif c == "}":
            if stack:
                stack.pop()
            header = ""
            i += 1
        elif c == ";":
            header = ""
            i += 1
        else:
            header += c
            i += 1
    return state


def _statement_lines(text):
    """The indices of the lines that are safe to edit as statements: not in a comment, not inside a header
    that spans lines, not part of a `case x -> y;` rule (or a file that has rules, or a module declaration),
    and not a `for`, `case` or `default` line. Returns the lines too, and whether each starts in a method body."""
    if re.search(r"^\s*(?:open\s+)?module\s", text, re.M):
        return [], [], []  # a module declaration takes no statements
    if re.search(r"^\s*(?:case .*|default)\s*->", text, re.M):
        return [], [], []  # a switch with rules: a multi-line rule body is picky
    lines = text.split("\n")
    in_code = code_lines(text)
    in_block = False
    depth = 0  # parentheses open at the start of the line: a line inside a header cannot take an edit
    chosen = []
    for i, line in enumerate(lines):
        opens = line.count("/*")
        closes = line.count("*/")
        skip = in_block or opens > 0 or closes > 0
        in_block = (in_block or opens > 0) and not (closes > 0 and line.rfind("*/") > line.rfind("/*"))
        inside = depth > 0
        if not skip and _wrap_code(line):
            depth = max(0, depth + line.count("(") - line.count(")"))
        if inside or skip or not _wrap_code(line) or "->" in line or (i > 0 and lines[i - 1].rstrip().endswith("->")) or line.count("(") != line.count(")") or re.search(r"\bfor\b|\bcase\b|\bdefault\b", line):
            continue
        chosen.append(i)
    return chosen, lines, in_code


def add_strays(text, rng, rate, nest=False):
    """Put an empty statement after a statement that ends a line (or, with `nest`, wrap it in braces), on
    `rate` of the places."""
    chosen, lines, in_code = _statement_lines(text)
    for i in chosen:
        line = lines[i]
        if STRAY_END.match(line) and rng.random() < rate * 4:
            if not nest:
                lines[i] = line + ";"
            elif (i < len(in_code) and in_code[i] and not MEMBER_START.match(line) and not DECLARATION.match(line) and line.strip()[0].isalnum()
                  and i > 0 and re.search(r"[;{}]$", lines[i - 1].rstrip())
                  and not lines[i - 1].lstrip().startswith("@")):
                indent = line[: len(line) - len(line.lstrip())]
                lines[i] = indent + "{ " + line.strip() + " }"
    return "\n".join(lines)


BOOL_RETURN = re.compile(r"^(\s*)return (?:true|false);$")
IF_OPEN = re.compile(r"^(\s*)if \(.*\) \{$")
RETURN_VALUE = re.compile(r"^(\s+)return (.+);$")


def add_booleans(text, rng, rate):
    """Give SimplifyBooleanExpression and SimplifyBooleanReturn something to find: a condition gets `== true`,
    `!false &&`, `|| false` or `? true : false`; a return gets `== false` or `&& true`; and the
    `if (c) { return true; } return false;` idiom becomes an if-else."""
    chosen, lines, in_code = _statement_lines(text)
    allowed = set(chosen)
    for i in chosen:
        if not (i < len(in_code) and in_code[i]) or rng.random() >= rate * 4:
            continue
        line = lines[i]
        m = CONDITION.match(line)
        if m:
            cond = m.group(2)
            new = rng.choice([f"{cond} == true", f"!false && ({cond})", f"({cond}) || false", f"({cond}) ? true : false", f"true != !({cond})", f"({cond}) == (true)"])
            lines[i] = f"{m.group(1)} ({new}) {{"
            continue
        m = RETURN_VALUE.match(line)
        if m and not m.group(2).strip() in ("true", "false") and line.strip()[0].isalnum() and "->" not in line:
            new = rng.choice([f"({m.group(2)}) == false", f"({m.group(2)}) && true", f"!!({m.group(2)})", f"({m.group(2)}) ? true : false"])
            lines[i] = f"{m.group(1)}return {new};"
    out = list(lines)
    for i in range(len(lines) - 3):
        a, b, c, d = lines[i:i + 4]
        if (IF_OPEN.match(a) and BOOL_RETURN.match(b) and c.strip() == "}" and BOOL_RETURN.match(d)
                and i in allowed and rng.random() < min(1.0, rate * 40)):
            indent = a[: len(a) - len(a.lstrip())]
            out[i + 2] = indent + "} else {"
            out[i + 3] = d
            out.insert(i + 4, "")  # placeholder, fixed below
    result = []
    for line in out:
        result.append(line)
    # the placeholder rows carry the closing brace, positioned after the second return
    fixed = []
    k = 0
    while k < len(result):
        if result[k] == "" and k > 0 and result[k - 1].strip().startswith("return") and k >= 2 and result[k - 2].strip() == "} else {":
            fixed.append(result[k - 2][: len(result[k - 2]) - len(result[k - 2].lstrip())] + "}")
        else:
            fixed.append(result[k])
        k += 1
    return "\n".join(fixed)


ASSIGN_STATEMENT = re.compile(r"^(\s+)(\w+) = (.*);$")
CALL_STATEMENT = re.compile(r"^(\s+)(\w+(?:\.\w+)*)\(([^()]+)\);$")
RETURN_STATEMENT = re.compile(r"^(\s+)return (.+);$")
CONDITION = re.compile(r"^(\s*(?:\} else )?(?:if|while)) \((.*)\) \{$")


def add_inner_assignments(text, rng, rate):
    """Turn a statement into one that holds an assignment: `x = y;` -> `x = x = y;`, `f(a);` -> `f(q = a);`,
    `return a;` -> `return q = a;`, and a condition into a comparison of an assignment with null."""
    chosen, lines, in_code = _statement_lines(text)
    for i in chosen:
        line = lines[i]
        if not (i < len(in_code) and in_code[i]) or MEMBER_START.match(line) or rng.random() >= rate * 4:
            continue
        m = CONDITION.match(line)
        if m:
            how = rng.randrange(3)
            cond = ["(q = %s) != null", "(q = %s) > 0", "q = %s"][how] % m.group(2)
            lines[i] = f"{m.group(1)} ({cond}) {{"
            continue
        if not line.strip()[0].isalnum() or DECLARATION.match(line):
            continue
        m = RETURN_STATEMENT.match(line)
        if m:
            lines[i] = f"{m.group(1)}return q = {m.group(2)};"
            continue
        m = ASSIGN_STATEMENT.match(line)
        if m and not line.strip().startswith(("final ", "var ")) and not re.match(r"^\s+[\w<>\[\].?, ]+ \w+ = ", line):
            lines[i] = f"{m.group(1)}{m.group(2)} = q {rng.choice(['=', '+='])} {m.group(3)};"
            continue
        m = CALL_STATEMENT.match(line)
        if m and not re.match(r"^\s+(?:super|this)\(", line):
            lines[i] = f"{m.group(1)}{m.group(2)}(q = {m.group(3)});"
    return "\n".join(lines)


CLASS_EDITS = [
    (re.compile(r"\bfinal (class\b)"), r"\1"),
    (re.compile(r"\bstatic (class\b)"), r"\1"),
    (re.compile(r"\babstract (class\b)"), r"\1"),
    (re.compile(r"^(\s*)(?:public|protected) ([A-Z]\w*\()"), r"\1private \2"),
    (re.compile(r"^(\s*)(?:public|protected) ([A-Z]\w*\()"), r"\1\2"),
]


PACKAGE_LINE = re.compile(r"^package ([\w.]+);\s*$")
IMPORT_LINE = re.compile(r"^import (?:static )?[\w.]+(?:\.\*)?;\s*$")


def add_imports(text, rng, rate):
    """Repeat an import, or add one that is redundant or illegal, on `rate * 8` of the import lines."""
    lines = text.split("\n")
    package = next((m.group(1) for l in lines for m in [PACKAGE_LINE.match(l)] if m), "")
    out = []
    for line in lines:
        out.append(line)
        if IMPORT_LINE.match(line) and rng.random() < rate * 8:
            extra = rng.choice([line, "import java.lang.Object;", "import java.lang.*;", "import sun.misc.Unsafe;",
                                "import java.io.File;", "import static java.lang.Math.abs;", "import static java.lang.Math.abs;",
                                f"import {package}.Other;" if package else line, f"import {package}.*;" if package else line])
            out.append(extra)
    return "\n".join(out)


def edit_classes(text, rng, rate):
    """Change class and constructor modifiers, on `rate * 8` of the lines that have one."""
    out = []
    for line in text.split("\n"):
        if not line.lstrip().startswith(("*", "//", "/*")) and rng.random() < rate * 8:
            pattern, repl = rng.choice(CLASS_EDITS)
            line = pattern.sub(repl, line, count=1)
        out.append(line)
    return "\n".join(out)


CHAINED = re.compile(r"^(\s*)\} (else\b.*|catch\b.*|finally\b.*)$")


def move_right_curlies(text, rng, rate):
    """Split `} else {` after its brace, or join a lone `}` to the line above, on `rate * 4` of the places."""
    lines = text.split("\n")
    out = []
    in_block = False
    for i, line in enumerate(lines):
        opens, closes = line.count("/*"), line.count("*/")
        skip = in_block or opens > 0 or closes > 0
        in_block = (in_block or opens > 0) and not (closes > 0 and line.rfind("*/") > line.rfind("/*"))
        if skip or '"' in line or "'" in line or "//" in line or rng.random() >= rate * 4:
            out.append(line)
            continue
        m = CHAINED.match(line)
        if m:
            out.append(m.group(1) + "}")
            out.append(m.group(1) + m.group(2))
        elif (line.strip() == "}" and out and out[-1].rstrip().endswith(";") and "//" not in out[-1] and "*/" not in out[-1]
              and not out[-1].strip().startswith(("*", "/*"))):
            out[-1] = out[-1].rstrip() + " }"
        else:
            out.append(line)
    return "\n".join(out)


def empty_blocks(text, rng, rate):
    """Empty the blocks that hold one simple statement: delete it, or put a comment in its place."""
    chosen, lines, in_code = _statement_lines(text)
    drop = set()
    for i in chosen:
        if i == 0 or i + 1 >= len(lines) or not (i < len(in_code) and in_code[i]):
            continue
        line = lines[i]
        if (STRAY_END.match(line) and lines[i - 1].rstrip().endswith("{") and lines[i + 1].strip().startswith("}")
                and not MEMBER_START.match(line) and rng.random() < rate * 8):
            if rng.random() < 0.5:
                drop.add(i)
            else:
                indent = line[: len(line) - len(line.lstrip())]
                lines[i] = indent + rng.choice(["// nothing", "/* nothing */"])
    return "\n".join(l for i, l in enumerate(lines) if i not in drop)


def change_endings(text, rng, rate):
    """Change how a file ends, on `rate * 4` of the files."""
    if rng.random() >= rate * 4:
        return text
    body = text.rstrip("\r\n")
    how = rng.randrange(7)
    if how == 0:
        return body
    if how == 1:
        return body + "\n\n"
    if how == 2:
        return body + "\r"
    if how == 3:
        return body + "\r\n"
    if how == 4:
        return text.replace("\r\n", "\n").replace("\n", "\r\n")
    if how == 5:
        return body + "\r\r"
    return body + "  "


def mutate(text, rng, rate):
    lines = text.split("\n")
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.lstrip()
        in_comment = stripped.startswith(("*", "//", "/*"))
        if not in_comment:
            if ELL.search(line) and rng.random() < rate * 4:
                line = ELL.sub(lambda m: m.group(1) + "l", line)
            m = STAR.match(line)
            if m and rng.random() < rate * 2:
                line = m.group(1) + ".*;"
            for pattern, replacement in MODIFIER_SWAPS:
                if pattern.search(line) and rng.random() < rate:
                    line = pattern.sub(replacement, line, count=1)
                    break
            m = ARRAY.match(line)
            if m and rng.random() < rate:
                line = ARRAY.sub(lambda m: f"{m.group(1)} {m.group(2)}[]{m.group(3)}", line, count=1)
            if BREAK.match(line) and rng.random() < rate * 3:
                i += 1
                continue
            m = DEFAULT.match(line)
            if m and rng.random() < rate * 3:
                line = f"{m.group(1)}case 0:"
            if SIMPLE_STATEMENT.match(line) and i + 1 < len(lines) and SIMPLE_STATEMENT.match(lines[i + 1]) and rng.random() < rate * 2:
                line = line.rstrip() + " " + lines[i + 1].strip()
                i += 1
        out.append(line)
        i += 1
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("source")
    ap.add_argument("destination")
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--rate", type=float, default=0.05)
    ap.add_argument("--around", action="store_true", help="only remove blanks around operators and braces (a separate pass)")
    ap.add_argument("--comments", action="store_true", help="only tighten comments and type annotations (a separate pass)")
    ap.add_argument("--curly", action="store_true", help="only move left curly braces (a separate pass)")
    ap.add_argument("--separators", action="store_true", help="only move separators across line breaks (a separate pass)")
    ap.add_argument("--after-token", action="store_true", help="only pad after tokens that take no blank (a separate pass)")
    ap.add_argument("--for-pad", action="store_true", help="only pad the end of for headers (a separate pass)")
    ap.add_argument("--imports", action="store_true", help="only add redundant and illegal imports (a separate pass)")
    ap.add_argument("--classes", action="store_true", help="only change class and constructor modifiers (a separate pass)")
    ap.add_argument("--rcurly", action="store_true", help="only move closing braces (a separate pass)")
    ap.add_argument("--empty", action="store_true", help="only empty blocks (a separate pass)")
    ap.add_argument("--eof", action="store_true", help="only change how files end (a separate pass)")
    ap.add_argument("--bool", action="store_true", help="only add redundant boolean expressions and returns (a separate pass)")
    ap.add_argument("--inner", action="store_true", help="only add assignments inside statements and conditions (a separate pass)")
    ap.add_argument("--nest", action="store_true", help="only wrap statements in braces (a separate pass)")
    ap.add_argument("--stray", action="store_true", help="only add empty statements (a separate pass)")
    ap.add_argument("--wrap", action="store_true", help="only move operators across line breaks (a separate pass)")
    ap.add_argument("--generics", action="store_true", help="only pad angle brackets (a separate pass)")
    ap.add_argument("--after-punct", action="store_true", help="only remove blanks after tokens (a separate pass)")
    ap.add_argument("--before-punct", action="store_true", help="only pad before punctuation (a separate pass)")
    ap.add_argument("--before-paren", action="store_true", help="only pad before parentheses (a separate pass)")
    ap.add_argument("--parens", action="store_true", help="only pad parentheses (a separate pass, so other seeds are unchanged)")
    a = ap.parse_args()
    rng = random.Random(a.seed)
    files = changed = 0
    for root, _, names in os.walk(a.source):
        for name in sorted(names):
            if not name.endswith(".java"):
                continue
            src = os.path.join(root, name)
            dst = os.path.join(a.destination, os.path.relpath(src, a.source))
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            with open(src, encoding="utf-8", errors="surrogateescape", newline="") as fh:
                text = fh.read()
            if a.parens:
                new = pad_parens(text, rng, a.rate)
            elif a.after_token:
                new = pad_after_tokens(text, rng, a.rate)
            elif a.for_pad:
                new = pad_for_iterators(text, rng, a.rate)
            elif a.imports:
                new = add_imports(text, rng, a.rate)
            elif a.classes:
                new = edit_classes(text, rng, a.rate)
            elif a.rcurly:
                new = move_right_curlies(text, rng, a.rate)
            elif a.empty:
                new = empty_blocks(text, rng, a.rate)
            elif a.eof:
                new = change_endings(text, rng, a.rate)
            elif a.bool:
                new = add_booleans(text, rng, a.rate)
            elif a.inner:
                new = add_inner_assignments(text, rng, a.rate)
            elif a.nest:
                new = add_strays(text, rng, a.rate, nest=True)
            elif a.stray:
                new = add_strays(text, rng, a.rate)
            elif a.comments:
                new = tighten_comments(text, rng, a.rate)
            elif a.curly:
                new = wrap_curlies(text, rng, a.rate)
            elif a.separators:
                new = wrap_separators(text, rng, a.rate)
            elif a.wrap:
                new = wrap_operators(text, rng, a.rate)
            elif a.generics:
                new = pad_generics(text, rng, a.rate)
            elif a.around:
                new = pad_around(text, rng, a.rate)
            elif a.after_punct:
                new = pad_after_punct(text, rng, a.rate)
            elif a.before_punct:
                new = pad_before_punct(text, rng, a.rate)
            elif a.before_paren:
                new = pad_before_paren(text, rng, a.rate)
            else:
                new = mutate(text, rng, a.rate)
            with open(dst, "w", encoding="utf-8", errors="surrogateescape", newline="") as fh:
                fh.write(new)
            files += 1
            changed += new != text
    print(f"{files} files copied to {a.destination}, {changed} of them mutated", file=sys.stderr)


if __name__ == "__main__":
    main()
