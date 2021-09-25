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
        self.factor()
    }

    fn factor(&mut self) -> SyntaxNode {
        let mut left = self.unary();

        while let Some(token) = self.peek() {
            if let SyntaxKind::Slash | SyntaxKind::Star = token.kind() {
                self.advance();
                let right = self.unary();
                left = SyntaxNode::new(SyntaxKind::BinExpr, vec![left.into(), token.into(), right.into()])
            }
        }
        left
    }

    fn unary(&mut self) -> SyntaxNode {
        if let Some(token) = self.peek() {
            let node = match token.kind() {
                SyntaxKind::Bang | SyntaxKind::Minus => {
                    self.advance();
                    let right = self.unary();
                    SyntaxNode::new(SyntaxKind::UnaryExpr, vec![token.into(), right.into()])
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
        let source_without_whitespace = source.chars().filter(|c| !c.is_whitespace()).collect::<String>();
        assert_eq!(source_without_whitespace, format!("{}", node));
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
        check_parse("!true", SyntaxKind::UnaryExpr);
        check_parse("!false", SyntaxKind::UnaryExpr);
        check_parse("!!false", SyntaxKind::UnaryExpr);
    }

    #[test]
    fn factor() {
        check_parse("1 / 2", SyntaxKind::BinExpr);
        check_parse("-1 * 2 / 3", SyntaxKind::BinExpr);
    }
}
