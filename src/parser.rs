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
        self.primary()
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
    use crate::{green::SyntaxNode, kinds::SyntaxKind, scanner::Scanner};

    #[test]
    fn smoke() {
        let mut scanner = Scanner::new("1");
        let tokens = scanner.scan().cloned().collect();
        let mut parser = Parser::new(tokens);
        let node = parser.parse();
        assert_eq!(node.kind(), SyntaxKind::Literal);
    }
}
