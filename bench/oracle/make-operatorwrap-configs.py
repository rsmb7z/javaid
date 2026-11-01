#!/usr/bin/env python3
"""Write the OperatorWrap configurations: the defaults under each option, and one config per token
under each option.

usage: make-operatorwrap-configs.py   (writes next to this script; prints the config names)
"""
import os

TOKENS = """QUESTION COLON EQUAL NOT_EQUAL DIV PLUS MINUS STAR MOD SR BSR GE GT SL LE LT BXOR BOR LOR BAND LAND TYPE_EXTENSION_AND
LITERAL_INSTANCEOF ASSIGN DIV_ASSIGN PLUS_ASSIGN MINUS_ASSIGN STAR_ASSIGN MOD_ASSIGN SR_ASSIGN BSR_ASSIGN SL_ASSIGN BXOR_ASSIGN
BOR_ASSIGN BAND_ASSIGN METHOD_REF LAMBDA""".split()
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="OperatorWrap">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


names = ["operatorwrap", "operatorwrap-eol"]
write("operatorwrap", [])
write("operatorwrap-eol", [("option", "eol")])
for t in TOKENS:
    for option in ("nl", "eol"):
        n = "operatorwrap-" + t.lower().replace("_", "") + ("" if option == "nl" else "-eol")
        write(n, [("tokens", t), ("option", option)])
        names.append(n)
print("\n".join(names))
