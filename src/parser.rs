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
        self.equality()
    }

    fn equality(&mut self) -> SyntaxNode {
        let mut left = self.comparison();

        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::BangEqual | SyntaxKind::EqualEqual => {
                    self.advance();
                    let right = self.comparison();
                    left = SyntaxNode::new(SyntaxKind::BinExpr, vec![left.into(), token.into(), right.into()])
                },
                _ => break
            }
        }
        left
    }

    fn comparison(&mut self) -> SyntaxNode {
        let mut left = self.term();

        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::Greater | SyntaxKind::GreaterEqual | SyntaxKind::Less | SyntaxKind::LessEqual => {
                    self.advance();
                    let right = self.term();
                    left = SyntaxNode::new(SyntaxKind::BinExpr, vec![left.into(), token.into(), right.into()]);
                },
                _ => break
            }
        }
        left
    }

    fn term(&mut self) -> SyntaxNode {
        let mut left = self.factor();

        while let Some(token) = self.peek() {
            if let SyntaxKind::Minus | SyntaxKind::Plus = token.kind() {
                self.advance();
                let right = self.factor();
                left = SyntaxNode::new(SyntaxKind::BinExpr, vec![left.into(), token.into(), right.into()]);
            } else {
                break
            }
        }
        left
    }

    fn factor(&mut self) -> SyntaxNode {
        let mut left = self.unary();

        while let Some(token) = self.peek() {
            if let SyntaxKind::Slash | SyntaxKind::Star = token.kind() {
                self.advance();
                let right = self.unary();
                left = SyntaxNode::new(SyntaxKind::BinExpr, vec![left.into(), token.into(), right.into()])
            } else {
                break
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

    #[test]
    fn term() {
        check_parse("1 + 2", SyntaxKind::BinExpr);
        check_parse("1 - 2 + 2", SyntaxKind::BinExpr);
        check_parse("-1 - 2 / 2", SyntaxKind::BinExpr);
    }

    #[test] 
    fn comparison() {
        check_parse("1 > 2", SyntaxKind::BinExpr);
        check_parse("1 >= 2", SyntaxKind::BinExpr);
        check_parse("1 < 2", SyntaxKind::BinExpr);
        check_parse("1 <= 2", SyntaxKind::BinExpr);
    }

    #[test]
    fn equality() {
        check_parse("1 == 2", SyntaxKind::BinExpr);
        check_parse("1 != 2", SyntaxKind::BinExpr);
    }
}
