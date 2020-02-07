#! /usr/bin/env bash

#   EXPAT TEST SCRIPT FOR W3C XML TEST SUITE

# This script can be used to exercise Expat against the
# 2013 w3c.org xml test suite, available from
# https://www.w3.org/XML/Test/xmlts20130923.tar.gz.

# To run this script, first set XMLWF below so that xmlwf can be
# found, then set the output directory with OUTPUT.

# The script lists all test cases where Expat shows a discrepancy
# from the expected result. Test cases where only the canonical
# output differs are prefixed with "Output differs:", and a diff file
# is generated in the appropriate subdirectory under $OUTPUT.

# If there are output files provided, the script will use
# output from xmlwf and compare the desired output against it.
# However, one has to take into account that the canonical output
# produced by xmlwf conforms to an older definition of canonical XML
# and does not generate notation declarations.

shopt -s nullglob

# Note: OUTPUT must terminate with the directory separator.

TS="/tmp/libexpat/xml-test-suite-2013"
OUTPUT="/tmp/libexpat/tests/out/"

MYDIR="`dirname \"$0\"`"
cd "$MYDIR"
MYDIR="`pwd`"
# test with ported Rust binary
XMLWF="${1:-`dirname $MYDIR`/../target/debug/xmlwf}"
# test with original C binary
# XMLWF="${1:-`dirname $MYDIR`/../upstream/expat/xmlwf/xmlwf}"
# Unicode-aware diff utility
DIFF="${MYDIR}/../../upstream/expat/tests/udiffer.py"


# RunXmlwfNotWF file reldir
# reldir includes trailing slash
RunXmlwfNotWF() {
  file="$1"
  reldir="$2"
  if $XMLWF -p "$file" > /dev/null; then
      echo "Expected not well-formed: $reldir$file"
      return 1
  else
      return 0
  fi 
}

# RunXmlwfWF file reldir
# reldir includes trailing slash
RunXmlwfWF() {
  file="$1"
  reldir="$2"
  $XMLWF -p -N -d "$OUTPUT$reldir" "$file" > outfile || return $?
  read outdata < outfile 
  if test "$outdata" = "" ; then 
      if [ -f "out/$file" ] ; then 
          $DIFF "$OUTPUT$reldir$file" "out/$file" > outfile 
          if [ -s outfile ] ; then 
              cp outfile "$OUTPUT$reldir$file.diff"
              echo "Output differs: $reldir$file"
              return 1
          fi 
      fi 
      return 0
  else 
      echo "In $reldir: $outdata"
      return 1
  fi 
}

SUCCESS=0
ERROR=0

UpdateStatus() {
  if [ "$1" -eq 0 ] ; then
    SUCCESS=`expr $SUCCESS + 1`
  else
    ERROR=`expr $ERROR + 1`
  fi
}

##########################
# well-formed test cases #
##########################

cd "$TS/xmlconf"
for xmldir in ibm/valid/P* \
              ibm/invalid/P* \
              ibm/xml-1.1/valid/P* \
              ibm/xml-1.1/invalid/P* \
              xmltest/valid/ext-sa \
              xmltest/valid/not-sa \
              xmltest/invalid \
              xmltest/invalid/not-sa \
              xmltest/valid/sa \
              sun/valid \
              sun/invalid ; do
  cd "$TS/xmlconf/$xmldir"
  mkdir -p "$OUTPUT$xmldir"
  for xmlfile in $(ls -1 *.xml | sort -d) ; do
      [[ -f "$xmlfile" ]] || continue
      RunXmlwfWF "$xmlfile" "$xmldir/"
      UpdateStatus $?
  done
  rm -f outfile
done

cd "$TS/xmlconf/oasis"
mkdir -p "$OUTPUT"oasis
for xmlfile in *pass*.xml ; do
    RunXmlwfWF "$xmlfile" "oasis/"
    UpdateStatus $?
done
rm outfile

##############################
# not well-formed test cases #
##############################

cd "$TS/xmlconf"
for xmldir in ibm/not-wf/P* \
              ibm/not-wf/p28a \
              ibm/not-wf/misc \
              ibm/xml-1.1/not-wf/P* \
              xmltest/not-wf/ext-sa \
              xmltest/not-wf/not-sa \
              xmltest/not-wf/sa \
              sun/not-wf ; do
  cd "$TS/xmlconf/$xmldir"
  for xmlfile in *.xml ; do
      RunXmlwfNotWF "$xmlfile" "$xmldir/"
      UpdateStatus $?
  done
done

cd "$TS/xmlconf/oasis"
for xmlfile in *fail*.xml ; do
    RunXmlwfNotWF "$xmlfile" "oasis/"
    UpdateStatus $?
done

echo "Passed: $SUCCESS"
echo "Failed: $ERROR"
