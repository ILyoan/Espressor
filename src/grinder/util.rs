use std::char;
use ast;
use token;

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
pub fn is_ident_name(token: &token::Token) -> bool {
    match *token {
        token::IDENT(_) => true,
        token::LITERAL(ref lit) => match *lit {
            token::LIT_BOOL(_) => true,
            _ => false,
        },
        _ => false,
    }
}

#[inline(always)]
pub fn is_dec_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

#[inline(always)]
pub fn is_assignment_operator(token: &token::Token) -> bool {
    match *token {
        token::ASSIGN => true,
        token::BINOPEQ(_) => true,
        _ => false
    }
}

pub fn convert_literal(literal: token::Literal) -> ast::LiteralValue {
    match literal {
        token::LIT_NULL => ast::LV_Null,
        token::LIT_BOOL(v) => ast::LV_Boolean(v),
        token::LIT_NUMERIC(v) => ast::LV_Number(from_str(v).unwrap()),
        token::LIT_STRING(v) => ast::LV_String(v),
        token::LIT_REGEXP(v) => ast::LV_RegExp(v),                    
    }
}

pub fn token_to_assignment_operator(token: token::Token) -> ast::AssignmentOperator {
    match token {
        token::ASSIGN => ast::AO_ASSIGN,
        token::BINOPEQ(token::PLUS) => ast::AO_PLUS,
        token::BINOPEQ(token::MINUS) => ast::AO_MINUS,
        token::BINOPEQ(token::MUL) => ast::AO_MUL,
        token::BINOPEQ(token::DIV) => ast::AO_DIV,
        token::BINOPEQ(token::MOD) => ast::AO_MOD,
        token::BINOPEQ(token::LSH) => ast::AO_LSH,
        token::BINOPEQ(token::RSH) => ast::AO_RSH,
        token::BINOPEQ(token::URSH) => ast::AO_URSH,
        token::BINOPEQ(token::BITWISE_OR) => ast::AO_BITWISE_OR,
        token::BINOPEQ(token::BITWISE_XOR) => ast::AO_BITWISE_XOR,
        token::BINOPEQ(token::BITWISE_AND) => ast::AO_BITWISE_AND,
        _ => fail!("{:?} is not a assignment operator", token)
    }
}

pub fn token_to_binary_operator(token: token::Token) -> ast::BinaryOperator {
    match token {
        token::EQ => ast::BO_EQ,
        token::STRICT_EQ => ast::BO_STRICT_EQ,
        token::NE => ast::BO_NE,
        token::STRICT_NE => ast::BO_STRICT_NE,
        token::LT => ast::BO_LT,
        token::LE => ast::BO_LE,
        token::GE => ast::BO_GE,
        token::GT => ast::BO_GT,
        token::BINOP(token::LSH) => ast::BO_LSH,
        token::BINOP(token::RSH) => ast::BO_RSH,
        token::BINOP(token::URSH) => ast::BO_URSH,
        token::BINOP(token::PLUS) => ast::BO_PLUS,
        token::BINOP(token::MINUS) => ast::BO_MINUS,
        token::BINOP(token::MUL) => ast::BO_MUL,
        token::BINOP(token::DIV) => ast::BO_DIV,
        token::BINOP(token::MOD) => ast::BO_MOD,
        token::BINOP(token::BITWISE_AND) => ast::BO_BITWISE_AND,
        token::BINOP(token::BITWISE_OR) => ast::BO_BITWISE_OR,
        token::BINOP(token::BITWISE_XOR) => ast::BO_BITWISE_XOR,
        token::IDENT(ref ident) => {
            match ident.as_slice() {
                "in" => ast::BO_IN,
                "instanceof" => ast::BO_INSTANCEOF,
                ".." => ast::BO_DOTDOT,
                _ => fail!("{:?} is not a binray operator", token)
            }
        }
        _ => fail!("{:?} is not a binary operator", token)
    }
}

pub fn token_to_unary_operator(token: token::Token) -> ast::UnaryOperator {
    match token {
        token::BINOP(token::PLUS) => ast::UO_PLUS,
        token::BINOP(token::MINUS) => ast::UO_MINUS,
        token::NOT => ast::UO_NOT,
        token::BITWISE_NOT => ast::UO_BITWISE_NOT,
        token::IDENT(ref ident) => {
            match ident.as_slice() {
                "void" => ast::UO_VOID,
                "typeof" => ast::UO_TYPEOF,
                "delete" => ast::UO_DELETE,
                _ => fail!("{:?} is not an unary operator", token)
            }
        }
        _ => fail!("{:?} is not an unary operator", token)
    }
}

pub fn token_to_update_operator(token: token::Token) -> ast::UpdateOperator {
    match token {
        token::INCREMENT => ast::UO_INCREASE,
        token::DECREMENT => ast::UO_DECREASE,
        _ => fail!("{:?} is not an update operator", token)
    }
}
