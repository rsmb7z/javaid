#!/usr/bin/env python3
"""Write the LeftCurly configurations: the defaults, the four options, ignoreEnums off, and one config
per token under each option.

usage: make-leftcurly-configs.py   (writes next to this script; prints the config names)
"""
import os

TOKENS = """ANNOTATION_DEF CLASS_DEF CTOR_DEF ENUM_CONSTANT_DEF ENUM_DEF INTERFACE_DEF LAMBDA LITERAL_CASE LITERAL_CATCH LITERAL_DEFAULT
LITERAL_DO LITERAL_ELSE LITERAL_FINALLY LITERAL_FOR LITERAL_IF LITERAL_SWITCH LITERAL_SYNCHRONIZED LITERAL_TRY LITERAL_WHILE METHOD_DEF
OBJBLOCK STATIC_INIT RECORD_DEF COMPACT_CTOR_DEF SWITCH_RULE""".split()
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="LeftCurly">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


names = ["leftcurly", "leftcurly-nl", "leftcurly-eol", "leftcurly-noignoreenums", "leftcurly-nl-noignoreenums", "leftcurly-nlow", "leftcurly-nlorsingleline"]
write("leftcurly-nlow", [("option", "nlow")])
write("leftcurly-nlorsingleline", [("option", "nl_or_singleline")])
write("leftcurly", [])
write("leftcurly-nl", [("option", "nl")])
write("leftcurly-eol", [("option", "eol")])
write("leftcurly-noignoreenums", [("ignoreEnums", "false")])
write("leftcurly-nl-noignoreenums", [("option", "nl"), ("ignoreEnums", "false")])
for t in TOKENS:
    for option in ("nl", "eol", "nlow", "nl_or_singleline"):
        n = "leftcurly-" + t.lower().replace("_", "") + "-" + option.replace("_", "")
        write(n, [("tokens", t), ("option", option)])
        names.append(n)
        if t in ("ENUM_DEF", "OBJBLOCK", "ENUM_CONSTANT_DEF", "CLASS_DEF") and option == "eol":
            n = n + "-noignoreenums"
            write(n, [("tokens", t), ("option", option), ("ignoreEnums", "false")])
            names.append(n)
print("\n".join(names))
