use std::char;

use reader::Reader;
use token;
use util;

#[deriving(Clone, Eq)]
pub enum LexMessageType {
    LMT_None,
    LMT_Illegal,
}

#[deriving(Clone, Eq)]
pub enum LexMessage {
    UnexpectedToken,
    UnexpectedNumber,
    UnexpectedString,
    NotImplemented,
}

impl ToStr for LexMessageType {
    #[inline]
    fn to_str(&self) -> ~str {
        match *self {
            LMT_None => ~"",
            LMT_Illegal => ~"ILLEGAL",
        }
    }
}

pub struct Lexer {
    priv reader: Reader,
}

impl Lexer {
    pub fn new(src: ~str) -> Lexer {
        Lexer {
            reader: Reader::new(src),
        }
    }

    pub fn next_token(&mut self) -> Option<token::Token> {
        if !self.reader.is_eof() {
            let token = next_token(&mut self.reader);
            Some(token)
        } else {
            None
        }
    }

    pub fn is_eof(&self) -> bool {
        self.reader.is_eof()
    }
}

fn lex_error(reader: &Reader, msg: LexMessage, mtype: LexMessageType) -> ! {
    if reader.is_eof() {
        fail!("'SyntasError: {:?} {}", msg, mtype.to_str())
    } else {
        fail!("'SyntasError: {:?} {} at row:{}, col:{}",
            msg, mtype.to_str(), reader.curr_pos().row, reader.curr_pos().col)
    }   
}

// Return the next token.
// Move cursor behind the token.
fn next_token(reader: &mut Reader) -> token::Token {
    reader.consume_whitespace_and_comments();

    let c = reader.curr();

    if util::is_ident_start(c) {
        scan_ident(reader)
    } else if util::is_dec_digit(c) {
        scan_number(reader)
    } else if util::is_quote(c) {
        scan_string(reader)
    } else if util::is_newline(c) {
        scan_newline(reader)
    } else {
        scan_operator_or_structure(reader)  
    }
}

// Scan identifier.
fn scan_ident(reader: &mut Reader) -> token::Token {
    let start_idx = reader.curr_pos_idx();
    while util::is_ident_continue(reader.curr()) {
        reader.bump();
    }
    do reader.with_str_from(start_idx) |ident| {
        token::reserved_or_idnetifier(ident)
    }
}

// Scan hexadecimal digits and return the string of it.
fn scan_digits(reader: &mut Reader, radix: uint) -> ~str {
    let mut res = ~"";
    while !reader.is_eof() {
        let c = reader.curr();
        match char::to_digit(c, radix) {
            Some(_) => {
                res.push_char(c);
                reader.bump();
            }
            None => break
        }
    };
    res
}

// Scan exponent part of a number from current position.
fn scan_exponent(reader: &mut Reader) -> Option<~str> {
    let mut res = ~"";
    let mut c = reader.curr();
    if c == 'e' || c == 'E' {
        res.push_char(c);
        reader.bump();

        c = reader.curr();
        if c == '+' || c == '-' {
            res.push_char(c);
            reader.bump();
        }
        let exponent = scan_digits(reader, 10u);
        if exponent.len() > 0u {
            return Some(res + exponent);
        } else {
            lex_error(reader, UnexpectedToken, LMT_Illegal);
        }
    } else {
        return None;
    }
}

// Scan number from current position.
fn scan_number(reader: &mut Reader) -> token::Token {
    let mut num_str;
    let mut base = 10u;

    // Check if the number is a hex.
    // TODO: manage oct numbers.
    if reader.is_curr('0') && reader.is_next('x') {
        reader.bump();
        reader.bump();
        base = 16u;
    }

    // Scan number.
    num_str = scan_digits(reader, base);

    // Scan float part.
    //let mut is_float = false;
    // '.' may be followed by an idntifie. in that case, the number is not a float.
    if reader.is_curr('.') && !util::is_ident_start(reader.next()) {
        // Hex number could not have float part.
        if base == 16u {
            lex_error(reader, UnexpectedNumber, LMT_Illegal);
        }
        //is_float = true;
        reader.bump();
        num_str.push_char('.');
        num_str.push_str(scan_digits(reader, 10u));
    }

    // Scan exponent part if it's exist.
    match scan_exponent(reader) {
        Some(ref exp) => {
            //is_float = true;
            num_str.push_str(*exp);
        }
        None => ()
    }
    token::LITERAL(token::LIT_NUMERIC(num_str))
}

// Scan string literal.
fn scan_string(reader: &mut Reader) -> token::Token {
    assert!(reader.is_curr('\'') || reader.is_curr('"'));

    let mut string = ~"";
    let quote = reader.bump_curr();
    while !reader.bump_if(quote) {
        if reader.is_eof() {
            lex_error(reader, UnexpectedToken, LMT_Illegal);
        }
        if reader.bump_if('\\') {
            match reader.curr() {
                'n' => { reader.bump(); string.push_char('\n'); }
                'r' => { reader.bump(); string.push_char('\r'); }
                't' => { reader.bump(); string.push_char('\t'); }
                'v' => { reader.bump(); string.push_char('\x0B'); }
                // TODO: Handle {\u, \x, \b, \f}.
                _ => { string.push_char(reader.bump_curr()); }
            }
        } else if util::is_newline(reader.curr()) {
            lex_error(reader, UnexpectedToken, LMT_Illegal);
        } else {
            string.push_char(reader.bump_curr());
        }
    }
    token::LITERAL(token::LIT_STRING(string))
}

// Scan newline.
fn scan_newline(reader: &mut Reader) -> token::Token {
    lex_error(reader, NotImplemented, LMT_None)
}

// Scan regular expression.
fn scan_regexp(reader: &mut Reader) -> token::Token {
    lex_error(reader, NotImplemented, LMT_None)
}

// Scan operators or sturctural symbols.
fn scan_operator_or_structure(reader: &mut Reader) -> token::Token {
    // Check if a binary operation is a form of an assignment.
    fn binop(reader: &mut Reader, op: token::Binop) -> token::Token {
        reader.bump();
        if reader.bump_if('=') {
            return token::BINOPEQ(op);
        } else {
            return token::BINOP(op);
        }
    }

    match reader.curr() {
        '=' => {
            reader.bump();
            if reader.bump_if('=') {
                if reader.bump_if('=') {
                    token::STRICT_EQ // "==="
                } else {
                    token::EQ // "=="
                }
            } else {
                token::ASSIGN // "="
            }
        }
        '!' => {
            reader.bump();
            if reader.bump_if('=') {
                if reader.bump_if('=') {
                    token::STRICT_NE // "!=="
                } else {
                    token::NE // "!="
                }
            } else {
                token::NOT // "!"
            }
        }
        '<' => {
            reader.bump();
            match reader.curr() {
                '=' => { reader.bump(); token::LE } // "<="
                '<' => { reader.bump(); binop(reader, token::LSH) } // "<<" or "<<="
                _ => { token::LT } // "<"
            }
        }
        '>' => {
            reader.bump();
            match reader.curr() {
                '=' => { reader.bump(); token::GE } // ">="
                '>' => { 
                    reader.bump();
                    if reader.bump_if('>') {
                        binop(reader, token::URSH) // ">>>" or ">>>="
                    } else {
                        binop(reader, token::RSH) // ">>" or ">>="
                    }
                }
                _ => { token::GT } // ">"
            }
        }
        '+' => {
            reader.bump();
            if reader.bump_if('+') {
                token::INCREMENT // "++"
            } else {
                binop(reader, token::PLUS) // "+" or "+="
            }
        }
        '-' => {
            reader.bump();
            if reader.bump_if('-') {
                token::DECREMENT // "--"
            } else {
                binop(reader, token::MINUS) // "-" or "-="
            }
        }
        '|' => {
            reader.bump();
            if reader.bump_if('|') {
                token::OR // "||"
            } else {
                binop(reader, token::BITWISE_OR) // "|" or "|="
            }
        }
        '&' => {
            reader.bump();
            if reader.is_curr('&') {
                reader.bump();
                token::AND //'&&'
            } else {
                binop(reader, token::BITWISE_AND) // '&' or '&&'
            }
        }
        '*' => { reader.bump(); binop(reader, token::MUL) } // "*" or "*="
        '/' => { reader.bump(); binop(reader, token::DIV) } // "/" or "/="
        '^' => { reader.bump(); binop(reader, token::BITWISE_XOR) } // "^" or "^="

        '~' => { reader.bump(); token::BITWISE_NOT } // "~"
        ';' => { reader.bump(); token::SEMICOLON }
        ',' => { reader.bump(); token::COMMA }
        '?' => { reader.bump(); token::HOOK }
        ':' => { reader.bump(); token::COLON }
        '.' => { reader.bump(); token::DOT }
        '[' => { reader.bump(); token::LBRACKET }
        ']' => { reader.bump(); token::RBRACKET }
        '{' => { reader.bump(); token::LBRACE }
        '}' => { reader.bump(); token::RBRACE }
        '(' => { reader.bump(); token::LPAREN }
        ')' => { reader.bump(); token::RPAREN }
        _=> {
            lex_error(reader, UnexpectedToken, LMT_Illegal)
        }
    }
}


#[cfg(test)]
mod test {  
    use super::Lexer;
    use super::super::token;

    #[test]
    fn simple_test() {
        let src = ~"var a = 1; var b=2; a+ b;";
        let mut lexer = Lexer::new(src);
        assert_eq!(lexer.next_token(), Some(token::IDENT(~"var")));
        assert_eq!(lexer.next_token(), Some(token::IDENT(~"a")));
        assert_eq!(lexer.next_token(), Some(token::ASSIGN));
        assert_eq!(lexer.next_token(), Some(token::LITERAL(token::LIT_NUMERIC(~"1"))));
        assert_eq!(lexer.next_token(), Some(token::SEMICOLON));
        assert_eq!(lexer.next_token(), Some(token::IDENT(~"var")));
        assert_eq!(lexer.next_token(), Some(token::IDENT(~"b")));
        assert_eq!(lexer.next_token(), Some(token::ASSIGN));
        assert_eq!(lexer.next_token(), Some(token::LITERAL(token::LIT_NUMERIC(~"2"))));
        assert_eq!(lexer.next_token(), Some(token::SEMICOLON));
        assert_eq!(lexer.next_token(), Some(token::IDENT(~"a")));
        assert_eq!(lexer.next_token(), Some(token::BINOP(token::PLUS)));
        assert_eq!(lexer.next_token(), Some(token::IDENT(~"b")));
        assert_eq!(lexer.next_token(), Some(token::SEMICOLON));
        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn string_literal() {
        let src = ~"'simple string token1'";
        let mut lexer = Lexer::new(src);
        assert_eq!(lexer.next_token(), Some(token::LITERAL(token::LIT_STRING(~"simple string token1"))));
        assert_eq!(lexer.next_token(), None);

        let src = ~"\"'simple string token2'\"";
        let mut lexer = Lexer::new(src);
        assert_eq!(lexer.next_token(), Some(token::LITERAL(token::LIT_STRING(~"'simple string token2'"))));
        assert_eq!(lexer.next_token(), None);
    }
}