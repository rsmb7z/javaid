"""mutate.py OUTDIR LISTFILE COUNT SEED: re-lay-out COUNT random files from LISTFILE in three ways.

The token stream is unchanged; only whitespace (and line comments, turned into block comments) differs:
  j  all on one line, single spaces
  s  one token per line, random tab/space indentation
  r  random spacing: spaces, tabs, occasional line breaks
"""
import os, random, re, sys

out, lst, count, seed = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
rnd = random.Random(seed)
files = [l for l in open(lst).read().split("\n") if l]
rnd.shuffle(files)

TOKEN = re.compile(r'''
    (?P<ws>\s+)
  | (?P<lc>//[^\n]*)
  | (?P<bc>/\*.*?\*/)
  | (?P<tb>"""(?:[^"\\]|\\.|"(?!""))*""")
  | (?P<str>"(?:[^"\\\n]|\\.)*")
  | (?P<chr>'(?:[^'\\\n]|\\.)+')
  | (?P<num>(?:0[bB][01_]+|0[xX][0-9a-fA-F_]*\.?[0-9a-fA-F_]*(?:[pP][+-]?[0-9]+)?|\.?[0-9][0-9_]*(?:\.[0-9_]*)?(?:[eE][+-]?[0-9]+)?)[lLfFdD]?)
  | (?P<id>non-sealed|[^\W\d]\w*|\$\w*)
  | (?P<op>>>>=|<<=|>>=|>>>|>>|<<|\.\.\.|->|::|\+\+|--|&&|\|\||==|!=|<=|>=|\+=|-=|\*=|/=|%=|&=|\|=|\^=|[^\s\w])
''', re.X | re.S)

def tokenize(src):
    pos = 0
    toks = []
    while pos < len(src):
        m = TOKEN.match(src, pos)
        if not m:
            return None
        pos = m.end()
        k = m.lastgroup
        if k == "ws":
            continue
        t = m.group(0)
        if k == "lc":
            body = t[2:]
            if "*/" in body:
                continue
            t = "/*" + body + "*/"
        toks.append((k, t))
    return toks

def join(toks, style):
    parts = []
    for i, (k, t) in enumerate(toks):
        if style == "j":
            parts.append(t)
            parts.append(" ")
        elif style == "s":
            parts.append(rnd.choice(["", " ", "\t", "  ", "\t\t", "    ", "\t "]) * 1)
            parts.append(t)
            parts.append("\n")
        else:
            parts.append(t)
            parts.append(rnd.choice([" ", " ", "  ", "\t", " \t", "\n", "\n  ", "\r\n"]))
    return "".join(parts)

os.makedirs(out, exist_ok=True)
names = []
n = 0
for f in files:
    if n >= count:
        break
    try:
        src = open(f, encoding="utf-8").read()
    except Exception:
        continue
    toks = tokenize(src)
    if not toks or len(toks) > 20000:
        continue
    n += 1
    for style in "jsr":
        p = "%s/m%04d%s.java" % (out, n, style)
        with open(p, "w", encoding="utf-8", newline="") as fh:
            fh.write(join(toks, style))
        names.append(p)
open(out + "/list", "w").write("\n".join(names) + "\n")
print(len(names), "files")
