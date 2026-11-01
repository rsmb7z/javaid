#!/usr/bin/env python3
"""Write the WhitespaceAfter configurations: the defaults, one config per token, and Google's selection.

usage: make-whitespaceafter-configs.py   (writes next to this script; prints the config names)
"""
import os

TOKENS = """COMMA SEMI TYPECAST LITERAL_IF LITERAL_ELSE LITERAL_WHILE LITERAL_DO LITERAL_FOR LITERAL_FINALLY LITERAL_RETURN
LITERAL_YIELD LITERAL_CATCH DO_WHILE ELLIPSIS LITERAL_SWITCH LITERAL_SYNCHRONIZED LITERAL_TRY LITERAL_CASE LAMBDA
LITERAL_WHEN ANNOTATIONS SINGLE_LINE_COMMENT""".split()
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="WhitespaceAfter">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


names = ["whitespaceafter"]
write(names[0], [])
for t in TOKENS:
    n = "whitespaceafter-" + t.lower().replace("_", "")
    write(n, [("tokens", t)])
    names.append(n)
# Google's own selection, which takes the two tokens the defaults leave out.
write("whitespaceafter-google", [("tokens", "COMMA, SEMI, TYPECAST, ELLIPSIS, LITERAL_YIELD, LITERAL_CASE, ANNOTATIONS, SINGLE_LINE_COMMENT")])
names.append("whitespaceafter-google")
print("\n".join(names))
