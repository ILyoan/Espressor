#[link(name = "ast",
       package_id = "ast",
       vers = "0.1-pre")];

#[crate_type = "lib"];


pub use newable::Newable;
pub use node_type::NodeType;

pub mod newable;
pub mod node_type;

// Node structures.

pub struct Node<T> {
    loc: SourceLocation,
    body: T,
}

pub struct SourceLocation {
    start: Position,
    end: Position,
}

pub struct Position {
    line: u32,
    column: u32,
}


// Program.

pub struct Program {
    body: ~[Statement],
}


// Functions.

pub enum Function {
    // From Declarations.
    FnFunctionDeclaration(~FunctionDeclaration),
    // From Expressions.
    FnFunctionExperssion(~FunctionExpression),
    FnArrowExpression(~ArrowExpression),
}

pub struct FunctionBody {
    id: Option<Identifier>,
    params: ~[Pattern],
    defaults: ~[Expression],
    rest: Option<Identifier>,
    body: Either<BlockStatement, Expression>,
    generator: bool,
    expression: bool,
}


// Statements.

pub enum Statement {
    StmtEmpty(~EmptyStatement),
    StmtBlock(~BlockStatement),
    StmtExpression(~ExpressionStatement),
    StmtIf(~IfStatement),
    StmtLabled(~LabledStatement),
    StmtBreak(~BreakStatement),
    StmtContinue(~ContinueStatement),
    StmtWith(~WithStatement),
    StmtSwitch(~SwitchStatement),
    StmtReturn(~ReturnStatement),
    StmtThrow(~ThrowStatement),
    StmtTry(~TryStatement),
    StmtWhile(~WhileStatement),
    StmtDoWhile(~DoWhileStatement),
    StmtFor(~ForStatement),
    StmtForIn(~ForInStatement),
    StmtForOf(~ForOfStatement),
    // From Declarations.
    StmtDeclaration(~Declaration),
}

pub struct EmptyStatement;

pub struct BlockStatement {
    body: ~[Statement],
}

pub struct ExpressionStatement {
    expression: Expression,
}

pub struct IfStatement {
    test: Expression,
    consquent: Statement,
    alternate: Option<Statement>,
}

pub struct LabledStatement {
    lable: Identifier,
    body: Statement,
}

pub struct BreakStatement {
    lable: Option<Identifier>,
}

pub struct ContinueStatement {
    lable: Option<Identifier>,
}

pub struct WithStatement {
    object: Expression,
    body: Statement,
}

pub struct SwitchStatement {
    discriminant: Expression,
    cases: ~[SwitchCase],
    lexical: bool,
}

pub struct ReturnStatement {
    argument: Option<Expression>,
}

pub struct ThrowStatement {
    argument: ~Expression,
}

pub struct TryStatement {
    block: BlockStatement,
    handler: Option<CatchClause>,
    guardedHandlers: ~[CatchClause],
    finalizer: Option<BlockStatement>,
}

pub struct WhileStatement {
    test: Expression,
    body: Statement,
}

pub struct DoWhileStatement {
    body: Statement,
    test: Expression,
}

pub struct ForStatement {
    init: Option<Either<VariableDeclaration, Expression>>,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Statement,
}

pub struct ForInStatement {
    left: Either<VariableDeclaration, Expression>,
    right: Expression,
    body: Statement,
    each: bool,
}

pub struct ForOfStatement {
    left: Either<VariableDeclaration, Expression>,
    right: Expression,
    body: Statement,    
}


// Declarations.

pub enum Declaration {
    DeclFunction(~FunctionDeclaration),
    DeclVariable(~VariableDeclaration),
}

pub enum DeclarationKind {
    Var,
    Let,
    Const,
}

pub struct FunctionDeclaration {
    function: FunctionBody,
}

pub struct VariableDeclaration {
    declarations: ~[VariableDeclarator],
    kind: DeclarationKind,
}

pub struct VariableDeclarator {
    id: Pattern,
    init: Option<Expression>,
}


// Patterns

pub enum Pattern {
    PtrnObject(~ObjectPattern),
    PtrnArray(~ArrayPattern),
    // From Expressions.
    PtrnExpression(~Expression),
    // From Miscellaneous.
    PtrnIdentifier(~Identifier),
}

pub struct ObjectPatternProperty {
    key: Either<Literal, Identifier>,
    value: Pattern,
}

pub struct ObjectPattern {
    properties: ObjectPatternProperty,
}

pub struct ArrayPattern {
    elements: ~[Option<Pattern>],
}


// Expressions

pub enum Expression {
    ExprThis(~ThisExpression),
    ExprArray(~ArrayExpression),
    ExprObject(~ObjectExpression),
    ExprFunction(~FunctionExpression),
    ExprArrow(~ArrowExpression),
    ExprSequence(~SequenceExpression),
    ExprUnary(~UnaryExpression),
    ExprBinary(~BinaryExpression),
    ExprAssignment(~AssignmentExpression),
    ExprUpdate(~UpdateExpression),
    ExprLogical(~LogicalExpression),
    ExprConditional(~CondionalExpression),
    ExprNew(~NewExpression),
    ExprCall(~CallExpression),
    ExprMember(~MemberExpression),
    // From Miscellaneous.
    ExprIdentifier(~Identifier),
}

pub struct ThisExpression;

pub struct ArrayExpression {
    elements: ~[Option<Expression>],
}

pub enum ObjectExpressionPropertyKind {
    Init,
    Get,
    Set,
}

pub struct ObjectExpressionProperty {
    key: Either<Literal, Identifier>,
    value: Expression,
    kind: ObjectExpressionPropertyKind,
}

pub struct ObjectExpression {
    properties: ~[ObjectExpressionProperty],
}

pub struct FunctionExpression {
    function: FunctionBody,
}

pub struct ArrowExpression {
    // Remember that ArrowExpression does not have 'id' field.
    function: FunctionBody,
}

pub struct SequenceExpression {
    expression: ~[Expression],
}

pub struct UnaryExpression {
    operator: UnaryOperator,
    prefix: bool,
    argument: Expression,
}

pub struct BinaryExpression {
    operator: BinaryOperator,
    left: Expression,
    right: Expression,
}

pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: Expression,
    right: Expression,
}

pub struct UpdateExpression {
    operator: UpdateOperator,
    argument: Expression,
    prefix: bool,
}

pub struct LogicalExpression {
    operator: LogicalOperator,
    left: Expression,
    right: Expression,
}

pub struct CondionalExpression {
    test: Expression,
    alternate: Expression,
    consequent: Expression,
}

pub struct NewExpression {
    callee: Expression,
    arguments: ~[Expression],
}

pub struct CallExpression {
    callee: Expression,
    arguments: ~[Expression],
}

pub struct MemberExpression {
    object: Expression,
    property: Either<Identifier, Expression>,
    computed: bool,
}


// Clauses

pub struct SwitchCase {
    test: Option<Expression>,
    consequent: ~[Statement],
}

pub struct CatchClause {
    param: Pattern,
    body: BlockStatement,
}


// Miscellaneous

pub struct Identifier {
    name: ~str,
}

pub enum LiteralValue {
    LV_String(~str),
    LV_Boolean(bool),
    LV_Null,
    // TODO
    LV_Number(f64),
    LV_RegExp(~str),
}

pub struct Literal {
    value: LiteralValue,
}

pub enum UnaryOperator {
    UO_PLUS, // "+"
    UO_MINUS, // "-"    
    UO_NOT, // "!"
    UO_BITWISE_NOT, // "~"
    UO_TYPEOF, // "typeof"
    UO_VOID, // "void"
    UO_DELETE, // "delete"
}

pub enum BinaryOperator {
    BO_EQ, // "=="
    BO_NE, // "!="
    BO_STRICT_EQ, // "==="
    BO_STRICT_NE, // "!=="
    BO_LT, // "<"
    BO_LE, // "<="
    BO_GT, // ">"
    BO_GE, // ">="
    BO_LSH, // "<<"
    BO_RSH, // ">>"
    BO_URSH, // ">>>"
    BO_PLUS, // "+"
    BO_MINUS, // "-"
    BO_MUL, // "*"
    BO_DIV, // "/"
    BO_MOD, // "%"
    BO_BITWISE_OR, // "|"
    BO_BITWISE_XOR, // "^"
    BO_BITWISE_AND, // "&"
    BO_IN, // "in"
    BO_INSTANCEOF, // "instanceof"
    BO_DOTDOT, // ".."
}

pub enum LogicalOperator {
    LO_OR, // "||"
    LO_AND, // "&&"
}

pub enum AssignmentOperator {
    AO_ASSIGN, // "="
    AO_PLUS, // "+="
    AO_MINUS, // "-="
    AO_MUL, // "*="
    AO_DIV, // "/="
    AO_MOD, // "%="
    AO_LSH, // "<<="
    AO_RSH, // ">>="
    AO_RUSH, // ">>>="
    AO_BITWISE_OR, // "|="
    AO_BITWISE_XOR, // "^="
    AO_BITWISE_AND, // "&="
}

pub enum UpdateOperator {
    UO_INCREASE, // "++"
    UO_DECREASE, // "--"
}


