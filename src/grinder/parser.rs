use ast::newable::Newable;
use ast::Node;
use ast::Program;

use lexer::Lexer;
use token;

pub struct Parser {
    // Lexer
    lexer: Lexer,
    // The root of the AST.
    program: ~Node<Program>,
    // The current token.
    priv token: token::Token,
}

impl Parser {
    pub fn new(src: ~str) -> Parser {
        let mut lexer = Lexer::new(src);
        let token = if lexer.is_eof() {
            token::EOF
        } else {
            lexer.next_token().unwrap()
        };

        Parser {
            lexer: lexer,
            program: ~Newable::new(),
            token: token,
        }
    }

    pub fn parse(&mut self) {
        while !self.lexer.is_eof() {
            self.parse_statement()
        }
    }

    fn parse_statement(&mut self) {
        // FIXME: remove copy.
        let token = self.token.clone();
        match token {
            token::SEMICOLON => self.parse_stmt_empty(),
            token::LBRACE => self.parse_stmt_block(),
            token::LPAREN => self.parse_stmt_expression(),
            token::IDENT(ref ident) => {
                match ident.as_slice() {
                    "if" => self.parse_stmt_if(),
                    "break" => self.parse_stmt_break(),
                    "continue" => self.parse_stmt_continue(),
                    "with" => self.parse_stmt_with(),
                    "switch" => self.parse_stmt_switch(),
                    "return" => self.parse_stmt_return(),
                    "throw" => self.parse_stmt_throw(),
                    "try" => self.parse_stmt_try(),
                    "while" => self.parse_stmt_while(),
                    "do" => self.parse_stmt_do_while(),
                    "for" => self.parse_stmt_for(),
                    "var" => self.parse_stmt_declaration(),
                    _ => fail!("Not Implemented")
                }
            }
            _ => self.parse_expression(),
        }
    }

    fn parse_stmt_empty(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_block(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_expression(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_if(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_break(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_continue(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_with(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_switch(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_return(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_throw(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_try(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_while(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_do_while(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_for(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_expression(&mut self) {
        fail!("Not Implemented");
    }

    fn parse_stmt_declaration(&mut self) {
        fail!("Not Implemented");
    }

    fn next_token(&mut self) -> token::Token {
        self.lexer.next_token().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Parser;

    #[test]
    fn simple_test() {
        let src = ~"var a = 1; var b = 2; a + b;";
        let mut parser = Parser::new(src);
        parser.parse();
    }
}