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
    priv token_next: token::Token,
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
            token_next: token_next,
        }
    }

    pub fn parse(&mut self) {
        while !self.is_eof() {
            self.parse_statement()
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.token == token::EOF
    }

    #[inline]
    fn bump(&mut self) {
        if !self.is_eof() {
            self.token = self.token_next.clone();
            self.token_next = self.lexer.next_token().map_default(token::EOF, |token| token);
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
    fn bump_if_any(&mut self, tokens: &[token::Token]) -> bool {
        if self.is_curr_any(tokens) {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline]
    fn is_curr(&self, token: token::Token) -> bool {
        self.token == token
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


    // ECMA 11.2 Left-Hand-Side Expressions
    fn parse_left_hand_side_expression(&mut self) -> ast::Expression {
        fail!("Not Implemented");
    }

    // ECMA 11.3 Postfix Expressions
    fn parse_postfix_expression(&mut self) -> ast::Expression {
        let exp = self.parse_left_hand_side_expression();
        if self.is_curr_any([token::INCREMENT, token::DECREMENT]) {
            let op = self.bump_curr();
            ast::ExprUpdate(~self.new_node(ast::UpdateExpression::new(util::token_to_update_operator(op), exp, false)))
        } else {
            exp
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
            let exp = self.parse_unary_expression();
            ast::ExprUnary(~self.new_node(ast::UnaryExpression::new(util::token_to_unary_operator(op), exp, true)))
        } else if self.is_curr_any([token::INCREMENT, token::DECREMENT]) {
            let op = self.bump_curr();
            let exp = self.parse_unary_expression();
            ast::ExprUpdate(~self.new_node(ast::UpdateExpression::new(util::token_to_update_operator(op), exp, true)))
        } else {
            self.parse_postfix_expression()
        }
    }

    // ECMA 11.5 Multiplicative Operators
    fn parse_multiplicative_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_unary_expression();
        while self.is_curr_any([token::BINOP(token::MUL), token::BINOP(token::DIV), token::BINOP(token::MOD)]) {
            let op = self.bump_curr();
            let exp2 = self.parse_unary_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), exp, exp2)));
        }
        exp
    }

    // ECMA 11.6 Additive Operators
    fn parse_additive_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_multiplicative_expression();
        while self.is_curr_any([token::BINOP(token::PLUS), token::BINOP(token::MINUS)]) {
            let op = self.bump_curr();
            let exp2 = self.parse_multiplicative_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), exp, exp2)));
        }
        exp
    }
    // ECMA 11.7 Bitwise Shift Operators
    fn parse_shift_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_additive_expression();
        while self.is_curr_any([token::BINOP(token::LSH), token::BINOP(token::RSH), token::BINOP(token::URSH)]) {
            let op = self.bump_curr();
            let exp2 = self.parse_additive_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), exp, exp2)));
        }
        exp
    }

    // ECMA 11.8 Relational Operators
    fn parse_relational_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_shift_expression();
        while self.is_curr_any([token::LT, token::GT, token::LE, token::GE, token::IDENT(~"instanceof"), token::IDENT(~"in")]) {
            let op = self.bump_curr();
            let exp2 = self.parse_shift_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), exp, exp2)));
        }
        exp
    }

    // ECMA 11.9 Equality Operators
    fn parse_equality_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_relational_expression();
        while self.is_curr_any([token::EQ, token::STRICT_EQ, token::NE, token::STRICT_NE]) {
            let op = self.bump_curr();
            let exp2 = self.parse_relational_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(util::token_to_binary_operator(op), exp, exp2)));
        }
        exp
    }

    // ECMA 11.10 Binary Bitwise Operators
    fn parse_bitwise_and_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_equality_expression();
        while self.bump_if(token::BINOP(token::BITWISE_AND)) {
            let exp2 = self.parse_equality_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(ast::BO_BITWISE_AND, exp, exp2)));
        }
        exp
    }
    fn parse_bitwise_xor_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_bitwise_and_expression();
        while self.bump_if(token::BINOP(token::BITWISE_XOR)) {
            let exp2 = self.parse_bitwise_and_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(ast::BO_BITWISE_XOR, exp, exp2)));
        }
        exp
    }

    fn parse_bitwise_or_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_bitwise_xor_expression();
        while self.bump_if(token::BINOP(token::BITWISE_OR)) {
            let exp2 = self.parse_bitwise_xor_expression();
            exp = ast::ExprBinary(~self.new_node(ast::BinaryExpression::new(ast::BO_BITWISE_OR, exp, exp2)));
        }
        exp
    }


    // ECMA 11.11 Binary Logical Operators
    fn parse_logical_and_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_bitwise_or_expression();
        while self.bump_if(token::AND) {
            let exp2 = self.parse_bitwise_or_expression();
            exp = ast::ExprLogical(~self.new_node(ast::LogicalExpression::new(ast::LO_OR, exp, exp2)));
        }
        exp
    }
    
    fn parse_logical_or_expression(&mut self) -> ast::Expression {
        let mut exp = self.parse_logical_and_expression();
        while self.bump_if(token::OR) {
            let exp2 = self.parse_logical_and_expression();
            exp = ast::ExprLogical(~self.new_node(ast::LogicalExpression::new(ast::LO_OR, exp, exp2)));
        }
        exp
    }

    // ECMA 11.12 Conditional Operator ( ? : )
    fn parse_conditional_expression(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 11.13 Assignment Operators
    fn parse_assignment_expression(&mut self) {
        self.parse_conditional_expression();
        fail!("Not Implemented")
    }

    // ECMA 11.14 Comma Operator ( , )
    fn parse_expression(&mut self) {
        self.parse_assignment_expression();
        fail!("Not Implemented");
    }


    // ECMA 12 Statement
    fn parse_statement(&mut self) {
        // FIXME: remove copy.
        let token = self.token.clone();
        match token {
            token::SEMICOLON => self.parse_empty_statement(),
            token::LBRACE => self.parse_block(),
            token::LPAREN => self.parse_expression_statement(),
            token::IDENT(ref ident) => {
                match ident.as_slice() {
                    "if" => self.parse_if_statement(),
                    "do" => self.parse_do_while_statement(),
                    "while" => self.parse_while_statement(),
                    "for" => self.parse_for_statement(),
                    "continue" => self.parse_continue_statement(),
                    "break" => self.parse_with_statement(),
                    "return" => self.parse_return_statement(),
                    "with" => self.parse_with_statement(),
                    "switch" => self.parse_switch_statement(),
                    "throw" => self.parse_throw_statement(),
                    "try" => self.parse_try_statement(),
                    "var" => self.parse_variable_statment(),
                    _ => fail!("Not Implemented")
                }
            }
            _ => self.parse_expression(),
        }
    }

    // ECMA 12.1 Block
    fn parse_block(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_statmemt_list(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.2 Variable Statement
    fn parse_variable_statment(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.3 Empty Statement
    fn parse_empty_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.4 Expression Statement
    fn parse_expression_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.5 if Statement
    fn parse_if_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.6 Iteration Statement

    // ECMA 12.6.1 do-while Statement
    fn parse_do_while_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.6.2 while Statement
    fn parse_while_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.6.3 for Statement
    fn parse_for_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.6.4 for-in Statement
    fn parse_for_in_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.7 continue Statement
    fn parse_continue_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.8 break Statement
    fn parse_break_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.9 return Statement
    fn parse_return_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.10 with Statement
    fn parse_with_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.11 switch Statement
    fn parse_switch_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.12 Labelled Statement
    fn parse_labelled_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.13 throw Statement
    fn parse_throw_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.14 try Statement
    fn parse_try_statement(&mut self) {
        fail!("Not Implemented");
    }

    // ECMA 12.15 debugger Statement
    fn parse_debugger_statement(&mut self) {
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