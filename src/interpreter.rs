use crate::ast::{self, AstNode};
use crate::env::Environment;
use crate::green::SyntaxNode;
use crate::kinds::SyntaxKind;
use crate::value::Value;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn default() -> Interpreter {
        Interpreter {
            env: Environment::default(),
        }
    }

    pub fn interpret(&mut self, syntax_node: SyntaxNode) -> Value {
        match syntax_node.kind() {
            SyntaxKind::Literal => self.evaluate_literal(syntax_node),
            SyntaxKind::UnaryExpr => self.evaluate_unary(syntax_node),
            SyntaxKind::BinExpr => self.evaluate_binary(syntax_node),
            SyntaxKind::Print => self.print(syntax_node),
            SyntaxKind::Var => self.var_declaration(syntax_node),
            SyntaxKind::Identifier => self.identifier(syntax_node),
            SyntaxKind::Block => self.block(syntax_node),
            SyntaxKind::If => self.if_condition(syntax_node),
            _ => panic!("{:?} can not be interpreted", syntax_node.kind()),
        }
    }

    fn if_condition(&mut self, syntax_node: SyntaxNode) -> Value {
        let if_condition = ast::If::cast(syntax_node).unwrap();
        let condition = self.interpret(if_condition.condition());
        if let Value::Bool(true) = condition {
            self.interpret(if_condition.then_branch());
        } else if let Some(else_branch) = if_condition.else_branch() {
            self.interpret(else_branch);
        }
        Value::Nil
    }

    fn block(&mut self, syntax_node: SyntaxNode) -> Value {
        let previous_env = self.env.clone();
        self.env = Environment::new(previous_env.clone());
        let block = ast::Block::cast(syntax_node).unwrap();
        for child in block.children() {
            self.interpret(child);
        }
        self.env = previous_env;
        Value::Nil
    }

    fn identifier(&mut self, syntax_node: SyntaxNode) -> Value {
        let ident = ast::Identifier::cast(syntax_node).unwrap();
        let value = self
            .env
            .get(&ident.name())
            .expect(&format!("undefind variable {}", ident.name()));
        value
    }

    fn var_declaration(&mut self, syntax_node: SyntaxNode) -> Value {
        let var_declaration = ast::VarDeclaration::cast(syntax_node).unwrap();
        let ident = var_declaration.ident();
        let initial_value = self.interpret(var_declaration.initializer());
        self.env.assign(ident.text(), initial_value);
        Value::Nil
    }

    fn print(&mut self, syntax_node: SyntaxNode) -> Value {
        let print = ast::Print::cast(syntax_node).unwrap();
        let value = self.interpret(print.expr());
        println!("{}", value);
        Value::Nil
    }

    fn evaluate_binary(&mut self, syntax_node: SyntaxNode) -> Value {
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

    fn evaluate_unary(&mut self, syntax_node: SyntaxNode) -> Value {
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
        let mut stmts = parser.parse();
        let mut interpreter = Interpreter::default();
        let value = interpreter.interpret(stmts.next().unwrap().clone());
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
