set -e

# download W3C test suite
BUILD_DIR="$PWD"
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


