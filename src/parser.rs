use crate::green::{SyntaxNode, SyntaxToken};
use crate::kinds::SyntaxKind;

pub struct Parser {
    tokens: Vec<SyntaxToken>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SyntaxToken>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> SyntaxNode {
        self.unary()
    }

    fn unary(&mut self) -> SyntaxNode {
        if let Some(token) = self.peek() {
            let node = match token.kind() {
                SyntaxKind::Bang | SyntaxKind::Minus => {
                    self.advance();
                    let right = self.unary();
                    SyntaxNode::new(SyntaxKind::Unary, vec![token.into(), right.into()])
                },
                _ => self.primary(),
            };
            return node;
        }
        panic!("No more tokens left");
    }

    fn primary(&mut self) -> SyntaxNode {
        if let Some(token) = self.advance() {
            let node = match token.kind() {
                SyntaxKind::False
                | SyntaxKind::True
                | SyntaxKind::Nil
                | SyntaxKind::Number
                | SyntaxKind::String => SyntaxNode::new(SyntaxKind::Literal, vec![token.into()]),
                _ => panic!("{:?} unimplemented", token.kind())
            };
            return node;
        }
        panic!("No more tokens left");
    }

    fn peek(&self) -> Option<SyntaxToken> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) -> Option<SyntaxToken> {
        let token = self.peek();
        self.current += 1;
        token
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{kinds::SyntaxKind, scanner::Scanner};

    fn check_parse(source: &str, expected_kind: SyntaxKind) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan().cloned().collect();
        let mut parser = Parser::new(tokens);
        let node = parser.parse();
        assert_eq!(node.kind(), expected_kind);
        assert_eq!(source, format!("{}", node));
    }

    #[test]
    fn primary() {
        check_parse("1", SyntaxKind::Literal);
        check_parse("true", SyntaxKind::Literal);
        check_parse("false", SyntaxKind::Literal);
        check_parse("nil", SyntaxKind::Literal);
        check_parse("\"hello\"", SyntaxKind::Literal);
    }

    #[test]
    fn unary() {
        check_parse("!true", SyntaxKind::Unary);
        check_parse("!false", SyntaxKind::Unary);
        check_parse("!!false", SyntaxKind::Unary);
    }
}
