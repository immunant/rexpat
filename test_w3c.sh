#! /usr/bin/env bash
set -e

SCRIPT_DIR="$(dirname "$0")"
EXP_OUTPUT="$SCRIPT_DIR/upstream/expat/tests/xmltest.log.expected"

[ -f "$EXP_OUTPUT" ] || {
    echo "$EXP_OUTPUT: No such file; did you checkout submodule?"; exit 1
}

XMLWF="$SCRIPT_DIR/target/debug/xmlwf"
[ -f "$XMLWF" ] || {
    echo "$XMLWF: No such file; did you \`cargo build\`?"; exit 1
}

# download W3C test suite
TEST_TMP=/tmp/libexpat
TEST_SUITE=/tmp/libexpat/xml-test-suite
ARCHIVE="$TEST_TMP/xmlts20020606.tar" 
if [ ! -d "$TEST_SUITE" ]; then
    mkdir -p "$TEST_TMP"
    curl http://www.w3.org/XML/Test/xmlts20020606.tar --silent --output "$ARCHIVE"
    tar -C "$TEST_TMP"  -xf "$ARCHIVE"
fi

# run xmltest.sh and check against expected output
"$PWD/src/tests/xmltest.sh" > "$TEST_TMP/xmltest.log" 2>&1
diff "$TEST_TMP/xmltest.log" "$PWD/upstream/expat/tests/xmltest.log.expected" && echo "PASS"


