"""reindent.py SRCDIR OUTDIR SEED SHIFT WRAP [COUNT]: a re-indented copy of (COUNT random files of) a Java tree.

The token stream is unchanged; only the white space between tokens differs:
  SHIFT  probability that a line's leading white space is replaced (shifted by a random amount, spaces or tabs)
  WRAP   probability that a blank between two tokens on one line becomes a line break plus a random indentation
Block comments and text blocks are single tokens and keep their text. Line comments are kept as they are.
Files are copied to OUTDIR under their relative paths.
"""
import os, random, re, sys

src, out, seed, shift, wrap = sys.argv[1], sys.argv[2], int(sys.argv[3]), float(sys.argv[4]), float(sys.argv[5])
count = int(sys.argv[6]) if len(sys.argv) > 6 else 10**9
rnd = random.Random(seed)

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


def rand_indent(base):
    n = max(0, base + rnd.choice([-8, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 4, 6, 8, 12]))
    style = rnd.random()
    if style < 0.6:
        return " " * n
    if style < 0.8:
        return "\t" * (n // 4) + " " * (n % 4)
    return "\t" * (n // 8) + " " * (n % 8)


def mutate(text):
    pos = 0
    parts = []
    line_indent = 0
    first_on_line = True
    pending_ws = ""
    while pos < len(text):
        m = TOKEN.match(text, pos)
        if not m:
            return None
        pos = m.end()
        k = m.lastgroup
        t = m.group(0)
        if k == "ws":
            pending_ws = t
            continue
        ws = pending_ws
        pending_ws = ""
        if "\n" in ws:
            head, _, tail = ws.rpartition("\n")
            if rnd.random() < shift:
                tail = rand_indent(len(tail.expandtabs(4)))
            ws = head + "\n" + tail
            line_indent = len(tail.expandtabs(4))
        elif ws and parts and rnd.random() < wrap and not parts[-1].startswith("//"):
            ws = "\n" + rand_indent(line_indent + 4)
        parts.append(ws + t)
    parts.append(pending_ws)
    return "".join(parts)


files = []
for root, _, names in os.walk(src):
    for n in names:
        if n.endswith(".java"):
            files.append(os.path.join(root, n))
files.sort()
rnd.shuffle(files)
done = 0
for f in files:
    if done >= count:
        break
    with open(f, encoding="utf-8", errors="surrogateescape", newline="") as fh:
        text = fh.read()
    res = mutate(text)
    if res is None:
        continue
    dst = os.path.join(out, os.path.relpath(f, src))
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    with open(dst, "w", encoding="utf-8", errors="surrogateescape", newline="") as fh:
        fh.write(res)
    done += 1
print(done, "files")
