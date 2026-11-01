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
