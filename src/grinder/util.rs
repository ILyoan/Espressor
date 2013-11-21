use std::char;

#[inline(always)]
pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

#[inline(always)]
pub fn is_newline(ch: char) -> bool {
    ch == '\n' || ch == '\r'
}

#[inline(always)]
pub fn is_quote(ch: char) -> bool {
    ch == '\'' || ch == '"'
}

// TODO: TO make sure the followings comply with ECMA spec. (7.6)
#[inline(always)]
pub fn is_ident_start(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z')
        || (ch >= 'A' && ch <= 'Z')
        || ch == '_'
        || ch == '$'
        || (ch > '\x7f' && char::is_XID_continue(ch))
}

#[inline(always)]
pub fn is_ident_continue(ch: char) -> bool {
    is_ident_start(ch)
        || (ch >= '0' && ch <= '9')
}

#[inline(always)]
pub fn is_dec_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

