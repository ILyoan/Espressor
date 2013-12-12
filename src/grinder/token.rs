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

    KEYWORD(Keyword),
    STRICT_KEYWORD(StrictKeyword),

    EOF,
}

#[deriving(Clone, Eq, IterBytes)]
pub enum Literal {
    LIT_NULL,
    LIT_BOOL(bool),
    // TODO: Need to be implemented more efficiently.
    LIT_NUMERIC(~str),
    // TODO: Interning string literals.
    LIT_STRING(~str),
    // TODO: Interning regular expression literals.
    LIT_REGEXP(~str),
}

#[deriving(Clone, Eq, IterBytes)]
pub enum Keyword {
    BREAK,
    CASE,
    CATCH,    
    CONTINUE,
    DEBUGGER,
    DEFAULT,
    DELETE,
    DO,
    ELSE,
    FINALLY,
    FOR,
    FUNCTION,
    IF,
    IN,
    INSTANCEOF,
    NEW,
    RETURN,
    SWITCH,
    THIS,
    THROW,
    TRY,
    TYPEOF,
    VAR,
    VOID,
    WHILE,
    WITH,
    // FUTURE RESERVED KEWORDS.
    CLASS,
    CONST,
    ENUM,
    EXPORT,
    EXTENDS,
    IMPORT,
    SUPER,
}

#[deriving(Clone, Eq, IterBytes)]
pub enum StrictKeyword {
    IMPLEMENTS,
    INTERFACE,
    LET,
    PACKAGE,
    PRIVATE,
    PROTECTED,
    PUBLIC,
    STATIC,
    YIELD,
}

impl ToStr for Token {
    fn to_str(&self) -> ~str {
        ~""
    }
}


pub fn reserved_or_idnetifier(string: &str) -> Token {
    match maybe_null_literal(string) {
        Some(lit) => LITERAL(lit),
        None => {
            match maybe_bool_literal(string) {
                Some(lit) => LITERAL(lit),
                None => {
                    match maybe_keyword(string) {
                        Some(keyword) => KEYWORD(keyword),
                        None => {
                            match maybe_strict_keyword(string) {
                                Some(keyword) => STRICT_KEYWORD(keyword),
                                None => IDENT(string.to_owned()),
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn maybe_null_literal(string: &str) -> Option<Literal> {
    match string {
        "null" => Some(LIT_NULL),
        _ => None,
    }
}

pub fn maybe_bool_literal(string: &str) -> Option<Literal> {
    match string {
        "true" => Some(LIT_BOOL(true)),
        "false" => Some(LIT_BOOL(false)),
        _ => None,
    }
}

pub fn maybe_keyword(string: &str) -> Option<Keyword> {
    match string {
        "break" => Some(BREAK),
        "case" => Some(CASE),
        "catch" => Some(CATCH),
        "continue" => Some(CONTINUE),
        "debugger" => Some(DEBUGGER),
        "default" => Some(DEFAULT),
        "delete" => Some(DELETE),
        "do" => Some(DO),
        "else" => Some(ELSE),
        "finally" => Some(FINALLY),
        "for" => Some(FOR),
        "function" => Some(FUNCTION),
        "if" => Some(IF),
        "in" => Some(IN),
        "instanceof" => Some(INSTANCEOF),
        "new" => Some(NEW),
        "return" => Some(RETURN),
        "switch" => Some(SWITCH),
        "this" => Some(THIS),
        "throw" => Some(THROW),
        "try" => Some(TRY),
        "typeof" => Some(TYPEOF),
        "var" => Some(VAR),
        "void" => Some(VOID),
        "while" => Some(WHILE),
        "with" => Some(WITH),
        // Future reserved kewords.
        "class" => Some(CLASS),
        "const" => Some(CONST),
        "enum" => Some(ENUM),
        "export" => Some(EXPORT),
        "extends" => Some(EXTENDS),
        "import" => Some(IMPORT),
        "super" => Some(SUPER),
        _ => None,
    }
}

pub fn maybe_strict_keyword(string: &str) -> Option<StrictKeyword> {
    match string {
        "implements" => Some(IMPLEMENTS),
        "inteface" => Some(INTERFACE),
        "let" => Some(LET),
        "package" => Some(PACKAGE),
        "private" => Some(PRIVATE),
        "protected" => Some(PROTECTED),
        "public" => Some(PUBLIC),
        "static" => Some(STATIC),
        "yield" => Some(YIELD),
        _ => None,
    }
}