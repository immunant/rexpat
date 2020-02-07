use libc::c_uint;
use num_derive::{ToPrimitive, FromPrimitive};

pub type C2RustUnnamed_2 = c_uint;
// pub const BT_NONXML: ByteType = 0;
// pub const BT_MALFORM: ByteType = 1;
// pub const BT_LT: ByteType = 2;
// pub const BT_AMP: ByteType = 3;
// pub const BT_RSQB: ByteType = 4;
// pub const BT_LEAD2: ByteType = 5;
// pub const BT_LEAD3: ByteType = 6;
// pub const BT_LEAD4: ByteType = 7;
// pub const BT_TRAIL: ByteType = 8;
// pub const BT_CR: ByteType = 9;
// pub const BT_LF: ByteType = 10;
// pub const BT_GT: ByteType = 11;
// pub const BT_QUOT: ByteType = 12;
// pub const BT_APOS: ByteType = 13;
// pub const BT_EQUALS: ByteType = 14;
// pub const BT_QUEST: ByteType = 15;
// pub const BT_EXCL: ByteType = 16;
// pub const BT_SOL: ByteType = 17;
// pub const BT_SEMI: ByteType = 18;
// pub const BT_NUM: ByteType = 19;
// pub const BT_LSQB: ByteType = 20;
// pub const BT_S: ByteType = 21;
// pub const BT_NMSTRT: ByteType = 22;
// pub const BT_COLON: ByteType = 23;
// pub const BT_HEX: ByteType = 24;
// pub const BT_DIGIT: ByteType = 25;
// pub const BT_NAME: ByteType = 26;
// pub const BT_MINUS: ByteType = 27;
// pub const BT_OTHER: ByteType = 28;
// pub const BT_NONASCII: ByteType = 29;
// pub const BT_PERCNT: ByteType = 30;
// pub const BT_LPAR: ByteType = 31;
// pub const BT_RPAR: ByteType = 32;
// pub const BT_AST: ByteType = 33;
// pub const BT_PLUS: ByteType = 34;
// pub const BT_COMMA: ByteType = 35;
// pub const BT_VERBAR: ByteType = 36;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
#[derive(ToPrimitive)]
#[derive(FromPrimitive)]
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