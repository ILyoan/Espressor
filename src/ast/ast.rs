#[link(name = "ast",
       package_id = "ast",
       vers = "0.1-pre")];

#[crate_type = "lib"];

pub use node_type::NodeType;
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
    FnFunctionDeclaration(~Node<FunctionDeclaration>),
    // From Expressions.
    FnFunctionExperssion(~Node<FunctionExpression>),
    FnArrowExpression(~Node<ArrowExpression>),
}

pub struct FunctionBody {
    id: Option<Node<Identifier>>,
    params: ~[Pattern],
    defaults: ~[Expression],
    rest: Option<Node<Identifier>>,
    body: Either<Node<BlockStatement>, Expression>,
    generator: bool,
    expression: bool,
}


// Statements.

pub enum Statement {
    StmtEmpty(~Node<EmptyStatement>),
    StmtBlock(~Node<BlockStatement>),
    StmtExpression(~Node<ExpressionStatement>),
    StmtIf(~Node<IfStatement>),
    StmtLabled(~Node<LabledStatement>),
    StmtBreak(~Node<BreakStatement>),
    StmtContinue(~Node<ContinueStatement>),
    StmtWith(~Node<WithStatement>),
    StmtSwitch(~Node<SwitchStatement>),
    StmtReturn(~Node<ReturnStatement>),
    StmtThrow(~Node<ThrowStatement>),
    StmtTry(~Node<TryStatement>),
    StmtWhile(~Node<WhileStatement>),
    StmtDoWhile(~Node<DoWhileStatement>),
    StmtFor(~Node<ForStatement>),
    StmtForIn(~Node<ForInStatement>),
    StmtForOf(~Node<ForOfStatement>),
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
    lable: Node<Identifier>,
    body: Statement,
}

pub struct BreakStatement {
    lable: Option<Node<Identifier>>,
}

pub struct ContinueStatement {
    lable: Option<Node<Identifier>>,
}

pub struct WithStatement {
    object: Expression,
    body: Statement,
}

pub struct SwitchStatement {
    discriminant: Expression,
    cases: ~[Node<SwitchCase>],
    lexical: bool,
}

pub struct ReturnStatement {
    argument: Option<Expression>,
}

pub struct ThrowStatement {
    argument: ~Expression,
}

pub struct TryStatement {
    block: Node<BlockStatement>,
    handler: Option<Node<CatchClause>>,
    guardedHandlers: ~[Node<CatchClause>],
    finalizer: Option<Node<BlockStatement>>,
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
    init: Option<Either<Node<VariableDeclaration>, Expression>>,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Statement,
}

pub struct ForInStatement {
    left: Either<Node<VariableDeclaration>, Expression>,
    right: Expression,
    body: Statement,
    each: bool,
}

pub struct ForOfStatement {
    left: Either<Node<VariableDeclaration>, Expression>,
    right: Expression,
    body: Statement,
}


// Declarations.

pub enum Declaration {
    DeclFunction(~Node<FunctionDeclaration>),
    DeclVariable(~Node<VariableDeclaration>),
}

pub enum DeclarationKind {
    Var,
    Let,
    Const,
}

pub struct FunctionDeclaration {
    function: Node<FunctionBody>,
}

pub struct VariableDeclaration {
    declarations: ~[Node<VariableDeclarator>],
    kind: Node<DeclarationKind>,
}

pub struct VariableDeclarator {
    id: Pattern,
    init: Option<Expression>,
}


// Patterns

pub enum Pattern {
    PtrnObject(~Node<ObjectPattern>),
    PtrnArray(~Node<ArrayPattern>),
    // From Expressions.
    PtrnExpression(~Expression),
    // From Miscellaneous.
    PtrnIdentifier(~Node<Identifier>),
}

pub struct ObjectPatternProperty {
    key: Either<Node<Literal>, Node<Identifier>>,
    value: Pattern,
}

pub struct ObjectPattern {
    properties: Node<ObjectPatternProperty>,
}

pub struct ArrayPattern {
    elements: ~[Option<Pattern>],
}


// Expressions

pub enum Expression {
    ExprThis(~Node<ThisExpression>),
    ExprArray(~Node<ArrayExpression>),
    ExprObject(~Node<ObjectExpression>),
    ExprFunction(~Node<FunctionExpression>),
    ExprArrow(~Node<ArrowExpression>),
    ExprUnary(~Node<UnaryExpression>),
    ExprBinary(~Node<BinaryExpression>),
    ExprUpdate(~Node<UpdateExpression>),
    ExprLogical(~Node<LogicalExpression>),
    ExprConditional(~Node<ConditionalExpression>),
    ExprAssignment(~Node<AssignmentExpression>),    
    ExprSequence(~Node<SequenceExpression>),
    ExprNew(~Node<NewExpression>),
    ExprCall(~Node<CallExpression>),
    ExprMember(~Node<MemberExpression>),
    // From Miscellaneous.
    ExprIdentifier(~Node<Identifier>),
    ExprLiteral(~Node<Literal>),
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
    key: Either<Node<Literal>, Node<Identifier>>,
    value: Expression,
    kind: ObjectExpressionPropertyKind,
}

pub struct ObjectExpression {
    properties: ~[ObjectExpressionProperty],
}

pub struct FunctionExpression {
    function: Node<FunctionBody>,
}

pub struct ArrowExpression {
    // Remember that ArrowExpression does not have 'id' field.
    function: Node<FunctionBody>,
}

pub struct UnaryExpression {
    operator: UnaryOperator,
    argument: Expression,
    prefix: bool,
}

pub struct BinaryExpression {
    operator: BinaryOperator,
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

pub struct ConditionalExpression {
    test: Expression,
    consequent: Expression,
    alternate: Expression,    
}

pub struct AssignmentExpression {
    operator: AssignmentOperator,
    left: Expression,
    right: Expression,
}

pub struct SequenceExpression {
    expression: ~[Expression],
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
    body: Node<BlockStatement>,
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
    AO_URSH, // ">>>="
    AO_BITWISE_OR, // "|="
    AO_BITWISE_XOR, // "^="
    AO_BITWISE_AND, // "&="
}

pub enum UpdateOperator {
    UO_INCREASE, // "++"
    UO_DECREASE, // "--"
}


// Impls for Node

impl<T> Node<T> {
    pub fn new(loc: SourceLocation, t: T) -> Node<T> {
        Node {
            loc: loc,
            body: t,
        }
    }
}

impl SourceLocation {
    pub fn new(start: Position, end: Position) -> SourceLocation {
        SourceLocation {
            start: start,
            end: end,
        }
    }
}

impl Position {
    pub fn new(line: u32, column: u32) -> Position {
        Position {
            line: line,
            column: column,
        }
    }
}

impl Program {
    pub fn new() -> Program {
        Program {
            body: ~[],
        }
    }
}


// Impls for Statements.

impl BlockStatement {
    pub fn new(body: ~[Statement]) -> BlockStatement {
        BlockStatement {
            body: body,
        }
    }
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> ExpressionStatement {
        ExpressionStatement {
            expression: expression,
        }
    }
}


// Impls for expressions.

impl ThisExpression {
    pub fn new() -> ThisExpression {
        ThisExpression
    }
}

impl ArrayExpression {
    pub fn new(elements: ~[Option<Expression>]) -> ArrayExpression {
        ArrayExpression {
            elements: elements,
        }
    }
}

impl ObjectExpressionProperty {
    pub fn new(key: Either<Node<Literal>, Node<Identifier>>,
               value: Expression,
               kind: ObjectExpressionPropertyKind)
            -> ObjectExpressionProperty {
        ObjectExpressionProperty {
            key: key,
            value: value,
            kind: kind,
        }
    }
    pub fn from_literal(literal: Node<Literal>,
                        value: Expression,
                        kind: ObjectExpressionPropertyKind)
            -> ObjectExpressionProperty {
        ObjectExpressionProperty {
            key: Left(literal),
            value: value,
            kind: kind,
        }
    }
    pub fn new_from_identifier(identifier: Node<Identifier>,
                               value: Expression,
                               kind: ObjectExpressionPropertyKind)
            -> ObjectExpressionProperty {
        ObjectExpressionProperty {
            key: Right(identifier),
            value: value,
            kind: kind,
        }
    }
}

impl ObjectExpression {
    pub fn new(properties: ~[ObjectExpressionProperty]) -> ObjectExpression {
        ObjectExpression {
            properties: properties,
        }
    }
}

impl UnaryExpression {
    pub fn new(op: UnaryOperator, arg: Expression, prefix: bool) -> UnaryExpression {
        UnaryExpression {
            operator: op,
            argument: arg,
            prefix: prefix,
        }
    }
}

impl BinaryExpression {
    pub fn new(op: BinaryOperator, left: Expression, right: Expression) -> BinaryExpression {
        BinaryExpression {
            operator: op,
            left: left,
            right: right,
        }
    }
}

impl UpdateExpression {
    pub fn new(op: UpdateOperator, arg: Expression, prefix: bool) -> UpdateExpression {
        UpdateExpression {
            operator: op,
            argument: arg,
            prefix: prefix,
        }
    }
}

impl LogicalExpression {
    pub fn new(op: LogicalOperator, left: Expression, right: Expression) -> LogicalExpression {
        LogicalExpression {
            operator: op,
            left: left,
            right: right,
        }
    }
}

impl ConditionalExpression {
    pub fn new(test: Expression,
               consequent: Expression,
               alternate: Expression)
            -> ConditionalExpression {
        ConditionalExpression {
            test: test,
            consequent: consequent,
            alternate: alternate,
        }
    }
}

impl AssignmentExpression {
    pub fn new(operator: AssignmentOperator,
               left: Expression,
               right: Expression)
            -> AssignmentExpression {
        AssignmentExpression {
            operator: operator,
            left: left,
            right: right,
        }
    }
}

impl SequenceExpression {
    pub fn new(expression: ~[Expression]) -> SequenceExpression {
        SequenceExpression {
            expression: expression,
        }
    }
}

impl NewExpression {
    pub fn new(callee: Expression, arguments: ~[Expression]) -> NewExpression {
        NewExpression {
            callee: callee,
            arguments: arguments,
        }
    }
}

impl CallExpression {
    pub fn new(callee: Expression, arguments: ~[Expression]) -> CallExpression {
        CallExpression {
            callee: callee,
            arguments: arguments,
        }
    }
}

impl MemberExpression {
    pub fn new_from_identifier(object: Expression, property: Identifier) -> MemberExpression {
        MemberExpression {
            object: object,
            property: Left(property),
            computed: false,
        }
    }
    pub fn new_from_expression(object: Expression, property: Expression) -> MemberExpression {
        MemberExpression {
            object: object,
            property: Right(property),
            computed: true,
        }
    }
}


impl Identifier {
    pub fn new(name: &str) -> Identifier {
        Identifier {
            name: name.to_owned(),
        }
    }
}

impl Literal {
    pub fn new(value: LiteralValue) -> Literal {
        Literal {
            value: value,
        }
    }
}