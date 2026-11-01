#!/usr/bin/env python3
"""Write the ParenPad configurations: the defaults, option=space, and one config per token.

One config per token is how the probe attributes each finding to the token that owns it, instead of
guessing which Checkstyle token a parenthesis belongs to.
usage: make-parenpad-configs.py   (run from anywhere; writes next to this script)
"""
import os

TOKENS = """ANNOTATION ANNOTATION_FIELD_DEF CTOR_CALL CTOR_DEF DOT ENUM_CONSTANT_DEF EXPR LITERAL_CATCH
LITERAL_DO LITERAL_FOR LITERAL_IF LITERAL_NEW LITERAL_SWITCH LITERAL_SYNCHRONIZED LITERAL_WHILE
METHOD_CALL METHOD_DEF QUESTION RESOURCE_SPECIFICATION SUPER_CTOR_CALL LAMBDA RECORD_DEF
RECORD_PATTERN_DEF""".split()

HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="ParenPad">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))
def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)
write("parenpad", [])
write("parenpad-space", [("option", "space")])
for t in TOKENS:
    write("parenpad-" + t.lower().replace("_", ""), [("tokens", t)])
print("\n".join(f'    ("PadProbe.java", "parenpad-{t.lower().replace("_", "")}"),' for t in TOKENS))
