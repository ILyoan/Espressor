use super::Node;
use super::Program;
use super::Function;
use super::{FnFunctionDeclaration, FnFunctionExperssion, FnArrowExpression};
use super::Statement;
use super::{StmtEmpty, StmtBlock, StmtExpression, StmtIf};
use super::{StmtLabled, StmtBreak, StmtContinue, StmtWith};
use super::{StmtSwitch, StmtReturn, StmtThrow, StmtTry};
use super::{StmtWhile, StmtDoWhile, StmtFor, StmtForIn};
use super::{StmtForOf, StmtDeclaration};
use super::{EmptyStatement, BlockStatement, ExpressionStatement, IfStatement};
use super::{LabledStatement, BreakStatement, ContinueStatement, WithStatement};
use super::{SwitchStatement, ReturnStatement, ThrowStatement, TryStatement};
use super::{WhileStatement, DoWhileStatement, ForStatement, ForInStatement};
use super::{ForOfStatement};
use super::Declaration;
use super::{DeclFunction, DeclVariable};
use super::{FunctionDeclaration, VariableDeclaration, VariableDeclarator}; 
use super::Pattern;
use super::{PtrnObject, PtrnArray, PtrnExpression, PtrnIdentifier};
use super::{ObjectPattern, ArrayPattern};
use super::Expression;
use super::{ExprThis, ExprArray, ExprObject, ExprFunction};
use super::{ExprArrow, ExprSequence, ExprUnary, ExprBinary};
use super::{ExprAssignment, ExprUpdate, ExprLogical, ExprConditional};
use super::{ExprNew, ExprCall, ExprMember, ExprIdentifier};
use super::{ExprLiteral};
use super::{ThisExpression, ArrayExpression, ObjectExpression, FunctionExpression};
use super::{ArrowExpression, SequenceExpression, UnaryExpression, BinaryExpression};
use super::{AssignmentExpression, UpdateExpression, LogicalExpression, CondionalExpression};
use super::{NewExpression, CallExpression, MemberExpression};
use super::{Identifier, Literal, SwitchCase, CatchClause};


pub trait NodeType {
    fn type_name(&self) -> &str;
}

impl<T: NodeType> NodeType for Node<T> {
    fn type_name(&self) -> &str {
        self.body.type_name()
    }
}

impl NodeType for Program {
    fn type_name(&self) -> &str { "Program" }
}

impl NodeType for Function {
    fn type_name(&self) -> &str {
        match *self {
            FnFunctionDeclaration(ref v) => v.type_name(),
            FnFunctionExperssion(ref v) => v.type_name(),
            FnArrowExpression(ref v) => v.type_name(),
        }
    }
}

impl NodeType for Statement {
    fn type_name(&self) -> &str {
        match *self {
            StmtEmpty(ref v) => v.type_name(),
            StmtBlock(ref v) => v.type_name(),
            StmtExpression(ref v) => v.type_name(),
            StmtIf(ref v) => v.type_name(),
            StmtLabled(ref v) => v.type_name(),
            StmtBreak(ref v) => v.type_name(),
            StmtContinue(ref v) => v.type_name(),
            StmtWith(ref v) => v.type_name(),
            StmtSwitch(ref v) => v.type_name(),
            StmtReturn(ref v) => v.type_name(),
            StmtThrow(ref v) => v.type_name(),
            StmtTry(ref v) => v.type_name(),
            StmtWhile(ref v) => v.type_name(),
            StmtDoWhile(ref v) => v.type_name(),
            StmtFor(ref v) => v.type_name(),
            StmtForIn(ref v) => v.type_name(),
            StmtForOf(ref v) => v.type_name(),
            StmtDeclaration(ref v) => v.type_name(),
        }
    }
}

impl NodeType for EmptyStatement {
    fn type_name(&self) -> &str { "EmptyStatement" }
}
impl NodeType for BlockStatement {
    fn type_name(&self) -> &str { "BlockStatement" }
}
impl NodeType for ExpressionStatement {
    fn type_name(&self) -> &str { "ExpressionStatement" }
}
impl NodeType for IfStatement {
    fn type_name(&self) -> &str { "IfStatement" }
}
impl NodeType for LabledStatement {
    fn type_name(&self) -> &str { "LabledStatement" }
}
impl NodeType for BreakStatement {
    fn type_name(&self) -> &str { "BreakStatement" } 
}
impl NodeType for ContinueStatement {
    fn type_name(&self) -> &str { "ContinueStatement" }
}
impl NodeType for WithStatement {
    fn type_name(&self) -> &str { "WithStatement" }
}
impl NodeType for SwitchStatement {
    fn type_name(&self) -> &str { "SwitchStatement" }
}
impl NodeType for ReturnStatement {
    fn type_name(&self) -> &str { "ReturnStatement" }
}
impl NodeType for ThrowStatement {
    fn type_name(&self) -> &str { "ThrowStatement" }
}
impl NodeType for TryStatement {
    fn type_name(&self) -> &str { "TryStatement" }
}
impl NodeType for WhileStatement {
    fn type_name(&self) -> &str { "WhileStatement" }
}
impl NodeType for DoWhileStatement {
    fn type_name(&self) -> &str { "DoWhileStatement" }
}
impl NodeType for ForStatement {
    fn type_name(&self) -> &str { "ForStatement" }
}
impl NodeType for ForInStatement {
    fn type_name(&self) -> &str { "ForInStatement" }
}
impl NodeType for ForOfStatement {
    fn type_name(&self) -> &str { "ForOfStatement" }
}

impl NodeType for Declaration {
    fn type_name(&self) -> &str {
        match *self {
            DeclFunction(ref v) => v.type_name(),
            DeclVariable(ref v) => v.type_name(),
        }
    }
}

impl NodeType for FunctionDeclaration {
    fn type_name(&self) -> &str { "FunctionDeclaration" }
}
impl NodeType for VariableDeclaration {
    fn type_name(&self) -> &str { "VariableDeclaration" }
}
impl NodeType for VariableDeclarator {
    fn type_name(&self) -> &str { "VariableDeclarator" }
}

impl NodeType for Pattern {
    fn type_name(&self) -> &str {
        match *self {
            PtrnObject(ref v) => v.type_name(),
            PtrnArray(ref v) => v.type_name(),
            PtrnExpression(ref v) => v.type_name(),
            PtrnIdentifier(ref v) => v.type_name(),
        }
    }
}

impl NodeType for ObjectPattern {
    fn type_name(&self) -> &str { "ObjectPattern" }
}
impl NodeType for ArrayPattern {
    fn type_name(&self) -> &str { "ArrayPattern" }
}

impl NodeType for Expression {
    fn type_name(&self) -> &str {
        match *self {
            ExprThis(ref v) => v.type_name(),
            ExprArray(ref v) => v.type_name(),
            ExprObject(ref v) => v.type_name(),
            ExprFunction(ref v) => v.type_name(),
            ExprArrow(ref v) => v.type_name(),
            ExprSequence(ref v) => v.type_name(),
            ExprUnary(ref v) => v.type_name(),
            ExprBinary(ref v) => v.type_name(),
            ExprAssignment(ref v) => v.type_name(),
            ExprUpdate(ref v) => v.type_name(),
            ExprLogical(ref v) => v.type_name(),
            ExprConditional(ref v) => v.type_name(),
            ExprNew(ref v) => v.type_name(),
            ExprCall(ref v) => v.type_name(),
            ExprMember(ref v) => v.type_name(),
            ExprIdentifier(ref v) => v.type_name(),
            ExprLiteral(ref v) => v.type_name(),
        }
    }
}

impl NodeType for ThisExpression {
    fn type_name(&self) -> &str { "ThisExpression" }
}
impl NodeType for ArrayExpression {
    fn type_name(&self) -> &str { "ArrayExpression" }
}
impl NodeType for ObjectExpression {
    fn type_name(&self) -> &str { "ObjectExpression" }
}
impl NodeType for FunctionExpression {
    fn type_name(&self) -> &str { "FunctionExpression" }
}
impl NodeType for ArrowExpression {
    fn type_name(&self) -> &str { "ArrowExpression" }
}
impl NodeType for SequenceExpression {
    fn type_name(&self) -> &str { "SequenceExpression" }
}
impl NodeType for UnaryExpression {
    fn type_name(&self) -> &str { "UnaryExpression" }
}
impl NodeType for BinaryExpression {
    fn type_name(&self) -> &str { "BinaryExpression" }
}
impl NodeType for AssignmentExpression {
    fn type_name(&self) -> &str { "AssignmentExpression" }
}
impl NodeType for UpdateExpression {
    fn type_name(&self) -> &str { "UpdateExpression" }
}
impl NodeType for LogicalExpression {
    fn type_name(&self) -> &str { "LogicalExpression" }
}
impl NodeType for CondionalExpression {
    fn type_name(&self) -> &str { "CondionalExpression" }
}
impl NodeType for NewExpression {
    fn type_name(&self) -> &str { "NewExpression" }
}
impl NodeType for CallExpression {
    fn type_name(&self) -> &str { "CallExpression" }
}
impl NodeType for MemberExpression {
    fn type_name(&self) -> &str { "MemberExpression" }
}

impl NodeType for SwitchCase {
    fn type_name(&self) -> &str { "SwitchCase" }
}
impl NodeType for CatchClause {
    fn type_name(&self) -> &str { "CatchClause" }
}

impl NodeType for Identifier {
    fn type_name(&self) -> &str { "Identifier" }
}
impl NodeType for Literal {
    fn type_name(&self) -> &str { "Literal" }
}
