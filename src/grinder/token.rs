#[deriving(Clone, Eq, IterBytes)]
pub enum Binop {
    LSH,
    RSH,
    URSH,
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    BITWISE_AND,
    BITWISE_OR,
    BITWISE_XOR,
}

#[deriving(Clone, Eq, IterBytes)]
pub enum Token {
    // Experssion-operator symbols.
    ASSIGN,
    EQ, 
    STRICT_EQ,
    NE,
    STRICT_NE,
    LE,
    LT,
    GE,
    GT,
    INCREMENT,
    DECREMENT,
    NOT,
    OR,
    AND,
    BITWISE_NOT,
    BINOP(Binop),
    BINOPEQ(Binop), 

    // Structural symbols.
    NEWLINE,
    SEMICOLON,
    COMMA,
    HOOK,
    COLON,
    DOT,
    LBRACKET,
    RBRACKET,
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,

    // Literals
    LITERAL(Literal),

    // Idents
    // TODO: Interning idents.
    IDENT(~str),

    EOF,
}

#[deriving(Clone, Eq, IterBytes)]
pub enum Literal {
    LIT_BOOL(bool),
    // TODO: Need to be implemented more efficiently.
    LIT_NUMERIC(~str),
    // TODO: Interning string literals.
    LIT_STRING(~str),
    // TODO: Interning regular expression literals.
    LIT_REGEXP(~str),
}

// TODO: Handling future reserved words. (7.6.1.2)
#[deriving(Clone, Eq, IterBytes)]
pub enum Keyword {
    Break,
    Case,
    Catch,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Export,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,
}

