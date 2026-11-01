#!/usr/bin/env python3
"""Write the SeparatorWrap configurations: the defaults (whose option is eol) and under each option
explicitly, and one config per token under each option.

usage: make-separatorwrap-configs.py   (writes next to this script; prints the config names)
"""
import os

TOKENS = "DOT COMMA SEMI ELLIPSIS ARRAY_DECLARATOR RBRACK METHOD_REF LPAREN RPAREN".split()
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="SeparatorWrap">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


# SeparatorWrap's own default option is eol, the opposite of OperatorWrap's.
names = ["separatorwrap", "separatorwrap-nl", "separatorwrap-eol"]
write("separatorwrap", [])
write("separatorwrap-nl", [("option", "nl")])
write("separatorwrap-eol", [("option", "eol")])
for t in TOKENS:
    for option in ("nl", "eol"):
        n = "separatorwrap-" + t.lower().replace("_", "") + "-" + option
        write(n, [("tokens", t), ("option", option)])
        names.append(n)
print("\n".join(names))
