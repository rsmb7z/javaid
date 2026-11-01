#!/usr/bin/env python3
"""Write the WhitespaceAround configurations: the defaults, one config per accepted token, and one per option.

usage: make-whitespacearound-configs.py   (writes next to this script; prints the config names)
"""
import os

# The acceptable tokens of WhitespaceAround in Checkstyle 14.4.0 (getAcceptableTokens).
TOKENS = """ASSIGN ARRAY_INIT BAND BAND_ASSIGN BOR BOR_ASSIGN BSR BSR_ASSIGN BXOR BXOR_ASSIGN COLON DIV DIV_ASSIGN DO_WHILE
EQUAL GE GT LAMBDA LAND LCURLY LE LITERAL_CATCH LITERAL_DO LITERAL_ELSE LITERAL_FINALLY LITERAL_FOR LITERAL_IF
LITERAL_RETURN LITERAL_SWITCH LITERAL_SYNCHRONIZED LITERAL_TRY LITERAL_WHILE LOR LT MINUS MINUS_ASSIGN MOD MOD_ASSIGN
NOT_EQUAL PLUS PLUS_ASSIGN QUESTION RCURLY SL SLIST SL_ASSIGN SR SR_ASSIGN STAR STAR_ASSIGN LITERAL_ASSERT
TYPE_EXTENSION_AND WILDCARD_TYPE GENERIC_START GENERIC_END ELLIPSIS LITERAL_WHEN""".split()
OPTIONS = ["allowEmptyConstructors", "allowEmptyMethods", "allowEmptyTypes", "allowEmptyLoops", "allowEmptyLambdas",
           "allowEmptyCatches", "allowEmptySwitchBlockStatements"]
HEAD = """<?xml version="1.0"?>
<!DOCTYPE module PUBLIC "-//Checkstyle//DTD Checkstyle Configuration 1.3//EN" "https://checkstyle.org/dtds/configuration_1_3.dtd">
<module name="Checker">
  <property name="charset" value="UTF-8"/>
  <property name="severity" value="warning"/>
  <property name="fileExtensions" value="java"/>
  <module name="TreeWalker"><module name="WhitespaceAround">%s</module></module>
</module>
"""
here = os.path.dirname(os.path.abspath(__file__))


def write(name, props):
    body = "".join(f'<property name="{k}" value="{v}"/>' for k, v in props)
    with open(os.path.join(here, name + ".xml"), "w") as fh:
        fh.write(HEAD % body)


names = ["whitespacearound"]
write(names[0], [])
for t in TOKENS:
    n = "whitespacearound-" + t.lower().replace("_", "")
    write(n, [("tokens", t)])
    names.append(n)
for o in OPTIONS:
    n = "whitespacearound-" + o[len("allow"):].lower()
    write(n, [(o, "true")])
    names.append(n)
write("whitespacearound-foreach", [("ignoreEnhancedForColon", "false")])
names.append("whitespacearound-foreach")
write("whitespacearound-allempty", [(o, "true") for o in OPTIONS])
names.append("whitespacearound-allempty")
print("\n".join(names))
