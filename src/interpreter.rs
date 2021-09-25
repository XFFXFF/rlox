use crate::ast::{AstNode, Literal};
use crate::green::SyntaxNode;
use crate::kinds::SyntaxKind;

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    Number(f32),
    Nil,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn default() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, syntax_node: SyntaxNode) -> Value {
        match syntax_node.kind() {
            SyntaxKind::Literal => self.evaluate_literal(syntax_node),
            _ => panic!("{:?} can not be interpreted", syntax_node.kind()),
        }
    }

    fn evaluate_literal(&self, syntax_node: SyntaxNode) -> Value {
        assert_eq!(syntax_node.kind(), SyntaxKind::Literal);
        let literal = Literal::cast(syntax_node).unwrap();
        let token = literal.token().unwrap();
        match token.kind() {
            SyntaxKind::False => Value::Bool(false),
            SyntaxKind::True => Value::Bool(true),
            SyntaxKind::String => {
                let text = token.text().chars().filter(|c| *c != '\"').collect();
                Value::String(text)
            }
            SyntaxKind::Number => {
                let number = token.text().parse::<f32>().unwrap();
                Value::Number(number)
            }
            SyntaxKind::Nil => Value::Nil,
            _ => panic!("Unexpected token in Literal: {:?}", token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use super::Value;
    use crate::Parser;
    use crate::Scanner;

    fn check_interpret(source: &str, expected: Value) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan().cloned().collect();
        let mut parser = Parser::new(tokens);
        let syntax_node = parser.parse();
        let interpreter = Interpreter::default();
        let value = interpreter.interpret(syntax_node);
        assert_eq!(value, expected);
    }

    #[test]
    fn literal() {
        check_interpret("true", Value::Bool(true));
        check_interpret("false", Value::Bool(false));
        check_interpret("\"hello\"", Value::String("hello".to_string()));
        check_interpret("nil", Value::Nil);
    }
}
