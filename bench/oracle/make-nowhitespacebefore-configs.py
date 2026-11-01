#!/usr/bin/env python3
"""Write the NoWhitespaceBefore configurations: defaults, allowLineBreaks, and one config per token.

usage: make-nowhitespacebefore-configs.py   (writes next to this script; prints the config names)
"""
import os

TOKENS = "COMMA SEMI POST_INC POST_DEC ELLIPSIS LABELED_STAT DOT METHOD_REF".split()
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="NoWhitespaceBefore">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


names = ["nowhitespacebefore", "nowhitespacebefore-linebreaks"]
write(names[0], [])
write(names[1], [("allowLineBreaks", "true")])
for t in TOKENS:
    n = "nowhitespacebefore-" + t.lower().replace("_", "")
    write(n, [("tokens", t)])
    write(n + "-linebreaks", [("tokens", t), ("allowLineBreaks", "true")])
    names += [n, n + "-linebreaks"]
print("\n".join(names))
