use crate::ast::{self, AstNode};
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
            SyntaxKind::UnaryExpr => self.evaluate_unary(syntax_node),
            SyntaxKind::BinExpr => self.evaluate_binary(syntax_node),
            _ => panic!("{:?} can not be interpreted", syntax_node.kind()),
        }
    }

    fn evaluate_binary(&self, syntax_node: SyntaxNode) -> Value {
        assert_eq!(syntax_node.kind(), SyntaxKind::BinExpr);
        let bin_expr = ast::BinExpr::cast(syntax_node.clone()).unwrap();
        let left_val = self.interpret(bin_expr.left());
        let right_val = self.interpret(bin_expr.right());
        match (&left_val, bin_expr.op().kind(), &right_val) {
            (Value::Number(left), SyntaxKind::Plus, Value::Number(right)) => {
                Value::Number(left + right)
            }
            (Value::Number(left), SyntaxKind::Minus, Value::Number(right)) => {
                Value::Number(left - right)
            }
            (Value::Number(left), SyntaxKind::Slash, Value::Number(right)) => {
                Value::Number(left / right)
            }
            (Value::Number(left), SyntaxKind::Star, Value::Number(right)) => {
                Value::Number(left * right)
            }
            (Value::Number(left), SyntaxKind::Greater, Value::Number(right)) => {
                Value::Bool(left > right)
            }
            (Value::Number(left), SyntaxKind::GreaterEqual, Value::Number(right)) => {
                Value::Bool(left >= right)
            }
            (Value::Number(left), SyntaxKind::Less, Value::Number(right)) => {
                Value::Bool(left < right)
            }
            (Value::Number(left), SyntaxKind::LessEqual, Value::Number(right)) => {
                Value::Bool(left <= right)
            }
            (Value::String(left), SyntaxKind::Plus, Value::String(right)) => {
                Value::String(left.to_string() + right)
            }
            (_, SyntaxKind::EqualEqual, _) => Value::Bool(left_val == right_val),
            (_, SyntaxKind::BangEqual, _) => Value::Bool(left_val != right_val),
            _ => panic!("Invalid Binary Expr: {}", syntax_node),
        }
    }

    fn evaluate_unary(&self, syntax_node: SyntaxNode) -> Value {
        assert_eq!(syntax_node.kind(), SyntaxKind::UnaryExpr);
        let unary_expr = ast::UnaryExpr::cast(syntax_node.clone()).unwrap();
        let value = self.interpret(unary_expr.node());
        match (unary_expr.op().kind(), &value) {
            (SyntaxKind::Minus, Value::Number(n)) => Value::Number(-n),
            (SyntaxKind::Bang, _) => Value::Bool(!Self::is_truthy(&value)),
            _ => panic!("Invalid Unary Expr: {}", syntax_node),
        }
    }

    fn evaluate_literal(&self, syntax_node: SyntaxNode) -> Value {
        assert_eq!(syntax_node.kind(), SyntaxKind::Literal);
        let literal = ast::Literal::cast(syntax_node).unwrap();
        let token = literal.token();
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
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn is_truthy(val: &Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
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

    #[test]
    fn unary() {
        check_interpret("!true", Value::Bool(false));
        check_interpret("!false", Value::Bool(true));
        check_interpret("!!!false", Value::Bool(true));
        check_interpret("-3", Value::Number(-3.));
    }

    #[test]
    fn binary() {
        check_interpret("1 + 2", Value::Number(3.));
        check_interpret("1 - 2", Value::Number(-1.));
        check_interpret("1 / 2", Value::Number(0.5));
        check_interpret("1 * 2", Value::Number(2.));
        check_interpret("1 > 2", Value::Bool(false));
        check_interpret("1 >= 2", Value::Bool(false));
        check_interpret("1 < 2", Value::Bool(true));
        check_interpret("1 <= 2", Value::Bool(true));
        check_interpret("1 == 2", Value::Bool(false));
        check_interpret("nil == nil", Value::Bool(true));
        check_interpret("nil == 1", Value::Bool(false));
        check_interpret("1 != 2", Value::Bool(true));
        check_interpret(
            "\"hello \" + \"world\"",
            Value::String("hello world".to_string()),
        );
    }
}
