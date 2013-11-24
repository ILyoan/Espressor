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


    // ECMA 11.13 Assignment Operators
    fn parse_assignment_expression(&mut self) {
        fail!("Not Implemented")
    }

    // ECMA 11.14 Comma Operator (,)
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