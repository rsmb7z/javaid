#!/usr/bin/env python3
"""Write the MethodParamPad configurations: the options, and one config per token.

usage: make-methodparampad-configs.py   (writes next to this script; prints the CASES lines)
"""
import os

TOKENS = "CTOR_CALL RECORD_PATTERN_DEF CTOR_DEF LITERAL_NEW METHOD_CALL METHOD_DEF SUPER_CTOR_CALL ENUM_CONSTANT_DEF RECORD_DEF".split()
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="MethodParamPad">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


names = ["methodparampad", "methodparampad-linebreaks", "methodparampad-space", "methodparampad-space-linebreaks"]
write(names[0], [])
write(names[1], [("allowLineBreaks", "true")])
write(names[2], [("option", "space")])
write(names[3], [("option", "space"), ("allowLineBreaks", "true")])
for t in TOKENS:
    n = "methodparampad-" + t.lower().replace("_", "")
    write(n, [("tokens", t)])
    names.append(n)
print("\n".join(names))
