use ast;
use ast::Node;

use lexer::Lexer;
use token;
use util;

pub struct Parser {
    // Lexer
    lexer: Lexer,
    // The root of the AST.
    program: ~Node<ast::Program>,
    // The current token.
    priv token: token::Token,
    // Next token.
    priv token_next: Option<token::Token>,
}

impl Parser {
    pub fn new(src: ~str) -> Parser {
        let mut lexer = Lexer::new(src);
        let token = lexer.next_token().map_default(token::EOF, |token| token);
        let token_next = lexer.next_token().map_default(token::EOF, |token| token);

        Parser {
            lexer: lexer,
            program: ~Node::new(
                ast::SourceLocation::new(
                    ast::Position::new(0, 0),
                    ast::Position::new(0, 0)),
                ast::Program::new()),
            token: token,
            token_next: None,
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof() {
            self.parse_statement();
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.token == token::EOF
    }

    #[inline]
    fn bump(&mut self) {
        if !self.is_eof() {
            self.token = if self.token_next.is_some() {
                // FIXME: remove unnecessary clone.
                self.token_next.clone().unwrap()
            } else {
                self.lexer.next_token().map_default(token::EOF, |token| token)
            };
            self.token_next = None
        }
    }

    #[inline]
    fn bump_curr(&mut self) -> token::Token {
        let token = self.token.clone();
        self.bump();
        token
    }

    #[inline]
    fn bump_if(&mut self, token: token::Token) -> bool {
        if self.is_curr(token) {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline]
    fn bump_expected(&mut self, token: token::Token) {
        if self.is_curr(token.clone()) {
            self.bump();
        } else {
            fail!("Expected token {:?}, but found {:?}", token, self.token);
        }
    }

    #[inline]
    fn bump_if_any(&mut self, tokens: &[token::Token]) -> bool {
        if self.is_curr_any(tokens) {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline]
    fn bump_semicolon(&mut self) {
        // `token_next` should be None.
        assert!(self.token_next.is_none());

        if !self.is_eof() {
            match self.lexer.lex_semicolon() {
                Some(v) => self.token = v,
                None => fail!("Expected semicolon(;), but found {:?}", self.bump_curr())
            }
        }
    }

    #[inline(always)]
    fn is_curr(&self, token: token::Token) -> bool {
        self.token == token
    }

    #[inline(always)]
    fn is_next(&mut self, token: token::Token) -> bool {
        if self.token_next.is_none() {
            self.token_next = Some(
                self.lexer.next_token().map_default(token::EOF, |token| token));
        }
        *self.token_next.get_ref() == token
    }

    #[inline]
    fn is_curr_any(&self, tokens: &[token::Token]) -> bool {
        tokens.iter().any(|token| self.token == *token)
    }

    #[inline]
    fn new_node<T>(&self, t: T) -> Node<T> {
        Node::new(
            ast::SourceLocation::new(
                ast::Position::new(0, 0),
                ast::Position::new(0, 0)),
            t)
    }

    
    // ECMA 11.1 Primary expressions
    fn parse_primary_expression(&mut self) -> ast::Expression {
        match self.token {
            token::KEYWORD(keyword) => {
                match keyword {
                    token::FUNCTION => {
                        self.parse_function_expression()
                    }
                    token::THIS => {
                        self.parse_this_expression()
                    }
                    _ => {
                        fail!("Expcted primary expression but found {:?}", self.token);
                    }
                }
            }
            token::IDENT(_) => {
                self.parse_identfier_expression()
            }
            token::LITERAL(_) => {
                self.parse_literal_expression()
            }
            token::LBRACKET => {
                self.parse_array_initialiser()
            }
            token::LBRACE => {
                self.parse_object_lnitiliser()
            }
            token::LPAREN => {
                self.parse_group_expression()
            }
            _ => {
                fail!("Expcted primary expression but found {:?}", self.token);
            }
        }
    }

    // ECMA 11.1.1 The this Keyword
    fn parse_this_expression(&mut self) -> ast::Expression {
        self.bump_expected(token::KEYWORD(token::THIS));
        ast::ExprThis(~self.new_node(ast::ThisExpression::new()))
    }

    // ECMA 11.1.2 Identifier Reference
    fn parse_identfier_expression(&mut self) -> ast::Expression {
        match self.bump_curr() {
            token::IDENT(ref ident) => {
                ast::ExprIdentifier(~self.new_node(ast::Identifier::new(*ident)))
            }
            _ => {
                fail!("Expcted an identifier but found {:?}", self.token);
            }
        }
    }

    // ECMA 11.1.3 Literal Reference
    fn parse_literal_expression(&mut self) -> ast::Expression {
        match self.bump_curr() {
            token::LITERAL(lit) => {
                let lit = util::convert_literal(lit);
                ast::ExprLiteral(~self.new_node(ast::Literal::new(lit)))
            }
            _ => {
                fail!("Expcted a literal but found {:?}", self.token);
            }
        }
    }

    // ECMA 11.1.4 Array Initialiser
    fn parse_array_initialiser(&mut self) -> ast::Expression {
        let mut elements = ~[];
        self.bump_expected(token::LBRACKET);
        while !self.is_curr(token::RBRACKET) {
            if self.bump_if(token::COMMA) {
                elements.push(None);
            } else {
                elements.push(Some(self.parse_assignment_expression()));
                if !self.is_curr(token::RBRACKET) {
                    self.bump_expected(token::COMMA);
                }
            }
        }
        self.bump_expected(token::RBRACKET);
        ast::ExprArray(~self.new_node(ast::ArrayExpression::new(elements)))
    }

    // ECMA 11.1.5 Object Initialiser
    fn parse_object_lnitiliser(&mut self) -> ast::Expression {
        let mut properties = ~[];
        self.bump_expected(token::LBRACE);
        while !self.is_curr(token::RBRACE) {
            properties.push(self.parse_property_assignment());
            if !self.is_curr(token::RBRACE) {
                self.bump_expected(token::COMMA);
            }
        }
        self.bump_expected(token::RBRACE);
        ast::ExprObject(~self.new_node(ast::ObjectExpression::new(properties)))
    }

    fn parse_property_assignment(&mut self) -> ast::ObjectExpressionProperty {
        match self.token.clone() {
            token::IDENT(ref ident) => {
                match ident.as_slice() {
                    "get" if !self.is_next(token::COLON) => {
                        self.parse_property_get_function()
                    }
                    "set" if !self.is_next(token::COLON) => {
                        self.parse_property_get_function()
                    }
                    _ => {
                        self.parse_property_init()
                    }
                }
            }
            token::LITERAL(_) => {
                self.parse_property_init()
            }
            _ => {
                fail!("Expected PropertyName but found {:?}", self.token)
            }
        }
    }

    fn parse_perperty_name(&mut self)
            -> Either<ast::Node<ast::Literal>, ast::Node<ast::Identifier>> {
        match self.bump_curr() {
            token::LITERAL(lit) => {
                let lit = util::convert_literal(lit);
                Left(self.new_node(ast::Literal::new(lit)))
            }
            token::IDENT(ident) => {
                Right(self.new_node(ast::Identifier::new(ident)))
            }
            v => {
                fail!("Expected PropertyName but found {:?}", v)
            }
        }
    }

    fn parse_property_init(&mut self) -> ast::ObjectExpressionProperty {
        let name = self.parse_perperty_name();
        self.bump_expected(token::COLON);
        let expr = self.parse_assignment_expression();
        ast::ObjectExpressionProperty::new(name, expr, ast::Init)
    }

    fn parse_property_get_function(&mut self) -> ast::ObjectExpressionProperty {
        self.bump(); // "get"
        let name = self.parse_perperty_name();
        self.bump_expected(token::LPAREN);
        self.bump_expected(token::RPAREN);
        fail!("Not Implemented");
        //ast::ObjectExpressionProperty::new(name, function, ast::Get)
    }

    fn parse_property_set_function(&mut self) -> ast::ObjectExpressionProperty {
        self.bump(); // "set"
        let name = self.parse_perperty_name();
        self.bump_expected(token::LPAREN);
        let param = self.bump_curr();
        self.bump_expected(token::RPAREN);
        match param {
            token::IDENT(_) => {
                fail!("Not Implemented");
            }
            _ => {
                fail!("Expected identifier but found {:?}", param);
            }
        };
        //ast::ObjectExpressionProperty::new(name, function, ast::Set)
    }

    // ECMA 11.1.6 The Group Operator
    fn parse_group_expression(&mut self) -> ast::Expression {
        self.bump_expected(token::LPAREN);
        let expr = self.parse_expression();
        self.bump_expected(token::RPAREN);
        expr
    }


    // ECMA 11.2 Left-Hand-Side Expressions
    fn parse_new_expression(&mut self) -> ast::Expression {
        let exp = self.parse_left_hand_side_expression(false);
        let arg = if self.is_curr(token::LPAREN) {
            self.parse_arguments()
        } else {
            ~[]
        };
        ast::ExprNew(~self.new_node(ast::NewExpression::new(exp, arg)))
    }

    fn parse_arguments(&mut self) -> ~[ast::Expression] {
        let mut args = ~[];
        self.bump_expected(token::LPAREN);
        while !self.is_curr(token::RPAREN) {
            args.push(self.parse_assignment_expression());
            if !self.bump_if(token::COMMA) {                
                break;
            }
        }
        self.bump_expected(token::RPAREN);
        args
    }

    fn parse_identifier_name(&mut self) -> ast::Identifier {
        let ident = self.bump_curr();
        if !util::is_ident_name(&ident) {
            fail!("Expected an identifiner name but found {:?}", ident);
        }
        ast::Identifier::new(ident.to_str())
    }

    fn parse_left_hand_side_expression(&mut self, is_allow_call: bool) -> ast::Expression {
        let mut expr = if self.bump_if(token::IDENT(~"new")) {
            self.parse_new_expression()
        } else {
            self.parse_primary_expression()
        };
        loop {
            match self.token {
                token::LBRACKET => {
                    self.bump();
                    let property = self.parse_expression();
                    expr = ast::ExprMember(~self.new_node(ast::MemberExpression::new_from_expression(expr, property)));
                    self.bump_expected(token::RBRACKET);
                }
                token::DOT => {
                    self.bump();
                    let property = self.parse_identifier_name();
                    expr = ast::ExprMember(~self.new_node(ast::MemberExpression::new_from_identifier(expr, property)));
                }
                token::LPAREN if is_allow_call => {
                    let args = self.parse_arguments();
                    expr = ast::ExprCall(~self.new_node(ast::CallExpression::new(expr, args)));
                }
                _ => {
                    break;
                }
            }
        }
        expr
    }

    // ECMA 11.3 Postfix Expressions
    fn parse_postfix_expression(&mut self) -> ast::Expression {
        let expr = self.parse_left_hand_side_expression(true);
        if self.is_curr_any([token::INCREMENT, token::DECREMENT]) {
            let op = self.bump_curr();
            ast::ExprUpdate(~self.new_node(ast::UpdateExpression::new(util::token_to_update_operator(op), expr, false)))
        } else {
            expr
        }
    }

    // ECMA 11.4 Unary Operator
    fn parse_unary_expression(&mut self) -> ast::Expression {
        if self.is_curr_any([
                token::BINOP(token::PLUS),
                token::BINOP(token::MINUS),
                token::BITWISE_NOT,
                token::NOT,
                token::IDENT(~"delete"),
                token::IDENT(~"void"),
                token::IDENT(~"typeof")]) {
            let op = self.bump_curr();
            let expr = self.parse_unary_expression();
            ast::ExprUnary(~self.new_node(ast::UnaryExpression::new(util::token_to_unary_operator(op), expr, true)))
        } else if self.is_curr_any([token::INCREMENT, token::DECREMENT]) {
            let op = self.bump_curr();
            let expr = self.parse_unary_expression();
            ast::ExprUpdate(~self.new_node(ast::UpdateExpression::new(util::token_to_update_operator(op), expr, true)))
        } else {
            self.parse_postfix_expression()
        }
    }

    // ECMA 11.5 Multiplicative Operators
    fn parse_multiplicative_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_unary_expression();
        while self.is_curr_any([token::BINOP(token::MUL), token::BINOP(token::DIV), token::BINOP(token::MOD)]) {
            let op = self.bump_curr();
            let expr2 = self.parse_unary_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), expr, expr2)));
        }
        expr
    }

    // ECMA 11.6 Additive Operators
    fn parse_additive_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_multiplicative_expression();
        while self.is_curr_any([token::BINOP(token::PLUS), token::BINOP(token::MINUS)]) {
            let op = self.bump_curr();
            let expr2 = self.parse_multiplicative_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), expr, expr2)));
        }
        expr
    }
    // ECMA 11.7 Bitwise Shift Operators
    fn parse_shift_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_additive_expression();
        while self.is_curr_any([token::BINOP(token::LSH), token::BINOP(token::RSH), token::BINOP(token::URSH)]) {
            let op = self.bump_curr();
            let expr2 = self.parse_additive_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), expr, expr2)));
        }
        expr
    }

    // ECMA 11.8 Relational Operators
    fn parse_relational_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_shift_expression();
        while self.is_curr_any([token::LT, token::GT, token::LE, token::GE, token::IDENT(~"instanceof"), token::IDENT(~"in")]) {
            let op = self.bump_curr();
            let expr2 = self.parse_shift_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), expr, expr2)));
        }
        expr
    }

    // ECMA 11.9 Equality Operators
    fn parse_equality_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_relational_expression();
        while self.is_curr_any([token::EQ, token::STRICT_EQ, token::NE, token::STRICT_NE]) {
            let op = self.bump_curr();
            let expr2 = self.parse_relational_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), expr, expr2)));
        }
        expr
    }

    // ECMA 11.10 Binary Bitwise Operators
    fn parse_bitwise_and_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_equality_expression();
        while self.bump_if(token::BINOP(token::BITWISE_AND)) {
            let expr2 = self.parse_equality_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(ast::BO_BITWISE_AND, expr, expr2)));
        }
        expr
    }
    fn parse_bitwise_xor_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_bitwise_and_expression();
        while self.bump_if(token::BINOP(token::BITWISE_XOR)) {
            let expr2 = self.parse_bitwise_and_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(ast::BO_BITWISE_XOR, expr, expr2)));
        }
        expr
    }

    fn parse_bitwise_or_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_bitwise_xor_expression();
        while self.bump_if(token::BINOP(token::BITWISE_OR)) {
            let expr2 = self.parse_bitwise_xor_expression();
            expr = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(ast::BO_BITWISE_OR, expr, expr2)));
        }
        expr
    }


    // ECMA 11.11 Binary Logical Operators
    fn parse_logical_and_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_bitwise_or_expression();
        while self.bump_if(token::AND) {
            let expr2 = self.parse_bitwise_or_expression();
            expr = ast::ExprLogical(~self.new_node(ast::LogicalExpression::new(ast::LO_OR, expr, expr2)));
        }
        expr
    }
    
    fn parse_logical_or_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_logical_and_expression();
        while self.bump_if(token::OR) {
            let expr2 = self.parse_logical_and_expression();
            expr = ast::ExprLogical(~self.new_node(ast::LogicalExpression::new(ast::LO_OR, expr, expr2)));
        }
        expr
    }

    // ECMA 11.12 Conditional Operator ( ? : )
    fn parse_conditional_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_logical_or_expression();
        if self.bump_if(token::HOOK) {
            let consquent = self.parse_assignment_expression();
            self.bump_expected(token::COLON);
            let alternate = self.parse_assignment_expression();
            expr = ast::ExprConditional(
                    ~self.new_node(ast::ConditionalExpression::new(expr, consquent, alternate)));
        }
        expr
    }

    // ECMA 11.13 Assignment Operators
    fn parse_assignment_expression(&mut self) -> ast::Expression {
        let mut expr = self.parse_conditional_expression();
        if util::is_assignment_operator(&self.token) {
            // FIXME: `expr` should be a LeftHandSideExpression.
            let operator = self.bump_curr();
            let operator = util::token_to_assignment_operator(operator);
            let right = self.parse_assignment_expression();
            expr = ast::ExprAssignment(
                ~self.new_node(ast::AssignmentExpression::new(operator, expr, right)))
        }
        expr   
    }

    // ECMA 11.14 Comma Operator ( , )
    fn parse_expression(&mut self) -> ast::Expression {
        let expr = self.parse_assignment_expression();
        if self.is_curr(token::COMMA) {
            let mut exprs = ~[expr];
            while self.bump_if(token::COMMA) {
                exprs.push(self.parse_assignment_expression());
            }
            ast::ExprSequence(~self.new_node(ast::SequenceExpression::new(exprs)))
        } else {
            expr
        }
    }


    // ECMA 12 Statement
    fn parse_statement(&mut self) -> ast::Statement {
        // FIXME: remove copy.
        let token = self.token.clone();
        match token {
            token::SEMICOLON => self.parse_empty_statement(),
            token::LBRACE => self.parse_block(),
            token::LPAREN => self.parse_expression_statement(),
            token::KEYWORD(token::IF) => self.parse_if_statement(),
            token::KEYWORD(token::DO) => self.parse_do_while_statement(),
            token::KEYWORD(token::WHILE) => self.parse_while_statement(),
            token::KEYWORD(token::FOR) => self.parse_for_statement(),
            token::KEYWORD(token::CONTINUE) => self.parse_continue_statement(),
            token::KEYWORD(token::WITH) => self.parse_with_statement(),
            token::KEYWORD(token::RETURN) => self.parse_return_statement(),
            token::KEYWORD(token::SWITCH) => self.parse_switch_statement(),
            token::KEYWORD(token::THROW) => self.parse_throw_statement(),
            token::KEYWORD(token::TRY) => self.parse_try_statement(),
            token::KEYWORD(token::VAR) => self.parse_variable_statment(),
            token::KEYWORD(v) => fail!("Not Implemented keyword {:?}", v),
            _ => {
                let expr = self.parse_expression();
                ast::StmtExpression(~self.new_node(ast::ExpressionStatement::new(expr)))
            }
        }
    }

    // ECMA 12.1 Block
    fn parse_block(&mut self) -> ast::Statement {
        self.bump_expected(token::LBRACE);
        let stmts = self.parse_statement_list();
        self.bump_expected(token::RBRACE);
        ast::StmtBlock(~self.new_node(ast::BlockStatement::new(stmts)))
    }

    fn parse_statement_list(&mut self) -> ~[ast::Statement] {
        let mut stmts = ~[];
        while !self.is_curr(token::RBRACE) {
            stmts.push(self.parse_statement());
        }
        stmts
    }

    // ECMA 12.2 Variable Statement
    fn parse_variable_statment(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.3 Empty Statement
    fn parse_empty_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.4 Expression Statement
    fn parse_expression_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.5 if Statement
    fn parse_if_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.6 Iteration Statement

    // ECMA 12.6.1 do-while Statement
    fn parse_do_while_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.6.2 while Statement
    fn parse_while_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.6.3 for Statement
    fn parse_for_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.6.4 for-in Statement
    fn parse_for_in_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.7 continue Statement
    fn parse_continue_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.8 break Statement
    fn parse_break_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.9 return Statement
    fn parse_return_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.10 with Statement
    fn parse_with_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.11 switch Statement
    fn parse_switch_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.12 Labelled Statement
    fn parse_labelled_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.13 throw Statement
    fn parse_throw_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.14 try Statement
    fn parse_try_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }

    // ECMA 12.15 debugger Statement
    fn parse_debugger_statement(&mut self) -> ast::Statement {
        fail!("Not Implemented");
    }


    // ECMA 13 Function Definition
    fn parse_function_expression(&mut self) -> ast::Expression {
        fail!("Not Implemented");
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn simple_test() {
        let src = ~"3+4";
        let mut parser = Parser::new(src);
        parser.parse();
    }
}