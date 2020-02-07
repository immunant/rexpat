use libc::c_uint;
use num_derive::{ToPrimitive, FromPrimitive};

pub type C2RustUnnamed_2 = c_uint;

#[derive(Clone, Copy, PartialEq, ToPrimitive, FromPrimitive)]
pub enum ByteType {
  NONXML,   /* e.g. noncharacter-FFFF */
  MALFORM,  /* illegal, with regard to encoding */
  LT,       /* less than = "<" */
  AMP,      /* ampersand = "&" */
  RSQB,     /* right square bracket = "[" */
  LEAD2,    /* lead byte of a 2-byte UTF-8 character */
  LEAD3,    /* lead byte of a 3-byte UTF-8 character */
  LEAD4,    /* lead byte of a 4-byte UTF-8 character */
  TRAIL,    /* trailing unit, e.g. second 16-bit unit of a 4-byte char. */
  CR,       /* carriage return = "\r" */
  LF,       /* line feed = "\n" */
  GT,       /* greater than = ">" */
  QUOT,     /* quotation character = "\"" */
  APOS,     /* aposthrophe = "'" */
  EQUALS,   /* equal sign = "=" */
  QUEST,    /* question mark = "?" */
  EXCL,     /* exclamation mark = "!" */
  SOL,      /* solidus, slash = "/" */
  SEMI,     /* semicolon = ";" */
  NUM,      /* number sign = "#" */
  LSQB,     /* left square bracket = "[" */
  S,        /* white space, e.g. "\t", " "[, "\r"] */
  NMSTRT,   /* non-hex name start letter = "G".."Z" + "g".."z" + "_" */
  COLON,    /* colon = ":" */
  HEX,      /* hex letter = "A".."F" + "a".."f" */
  DIGIT,    /* digit = "0".."9" */
  NAME,     /* dot and middle dot = "." + chr(0xb7) */
  MINUS,    /* minus = "-" */
  OTHER,    /* known not to be a name or name start character */
  NONASCII, /* might be a name or name start character */
  PERCNT,   /* percent sign = "%" */
  LPAR,     /* left parenthesis = "(" */
  RPAR,     /* right parenthesis = "(" */
  AST,      /* asterisk = "*" */
  PLUS,     /* plus sign = "+" */
  COMMA,    /* comma = "," */
  VERBAR    /* vertical bar = "|" */
}