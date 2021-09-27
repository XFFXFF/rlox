use crate::green::{SyntaxElement, SyntaxNode, SyntaxToken};
use crate::kinds::SyntaxKind;

pub struct Parser {
    tokens: Vec<SyntaxToken>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SyntaxToken>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> impl Iterator<Item = SyntaxNode> {
        let mut statements = Vec::new();
        while let Some(_) = self.peek() {
            statements.push(self.statement());
        }
        statements.into_iter()
    }

    fn statement(&mut self) -> SyntaxNode {
        if let Some(token) = self.peek() {
            let stmt = match token.kind() {
                SyntaxKind::Print => self.print(),
                SyntaxKind::Var => self.var_declaration(),
                SyntaxKind::LeftBrace => self.block(),
                _ => self.expression(),
            };
            return stmt;
        }
        panic!("No more tokens left.");
    }

    fn block(&mut self) -> SyntaxNode {
        let mut stmts = Vec::new();
        self.consume(SyntaxKind::LeftBrace, "Expect '{' before block");
        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::RightBrace => break,
                _ => stmts.push(self.statement()),
            }
        }
        self.consume(SyntaxKind::RightBrace, "Expect '}' after block");
        let stmts = stmts
            .into_iter()
            .map(|stmt| stmt.into())
            .collect::<Vec<SyntaxElement>>();
        SyntaxNode::new(SyntaxKind::Block, stmts)
    }

    fn var_declaration(&mut self) -> SyntaxNode {
        assert_eq!(self.peek().unwrap().kind(), SyntaxKind::Var);
        self.consume(SyntaxKind::Var, "Expect 'Var' keyword");
        let ident = self.consume(SyntaxKind::Identifier, "Expect an Identifier");
        let initializer = match self.peek().unwrap().kind() {
            SyntaxKind::Equal => {
                self.advance();
                self.expression()
            }
            _ => SyntaxNode::new(SyntaxKind::Nil, vec![]),
        };
        self.consume(SyntaxKind::Semicolon, "Expect ';' after value.");
        SyntaxNode::new(SyntaxKind::Var, vec![ident.into(), initializer.into()])
    }

    fn print(&mut self) -> SyntaxNode {
        let token = self.peek().unwrap();
        assert_eq!(token.kind(), SyntaxKind::Print);
        self.advance();
        let expr = self.expression();
        self.consume(SyntaxKind::Semicolon, "Expect ';' after value.");
        SyntaxNode::new(SyntaxKind::Print, vec![token.into(), expr.into()])
    }

    fn expression(&mut self) -> SyntaxNode {
        self.equality()
    }

    fn equality(&mut self) -> SyntaxNode {
        let mut left = self.comparison();

        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::BangEqual | SyntaxKind::EqualEqual => {
                    self.advance();
                    let right = self.comparison();
                    left = SyntaxNode::new(
                        SyntaxKind::BinExpr,
                        vec![left.into(), token.into(), right.into()],
                    )
                }
                _ => break,
            }
        }
        left
    }

    fn comparison(&mut self) -> SyntaxNode {
        let mut left = self.term();

        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::Greater
                | SyntaxKind::GreaterEqual
                | SyntaxKind::Less
                | SyntaxKind::LessEqual => {
                    self.advance();
                    let right = self.term();
                    left = SyntaxNode::new(
                        SyntaxKind::BinExpr,
                        vec![left.into(), token.into(), right.into()],
                    );
                }
                _ => break,
            }
        }
        left
    }

    fn term(&mut self) -> SyntaxNode {
        let mut left = self.factor();

        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::Minus | SyntaxKind::Plus => {
                    self.advance();
                    let right = self.factor();
                    left = SyntaxNode::new(
                        SyntaxKind::BinExpr,
                        vec![left.into(), token.into(), right.into()],
                    );
                }
                _ => break,
            }
        }
        left
    }

    fn factor(&mut self) -> SyntaxNode {
        let mut left = self.unary();

        while let Some(token) = self.peek() {
            match token.kind() {
                SyntaxKind::Slash | SyntaxKind::Star => {
                    self.advance();
                    let right = self.unary();
                    left = SyntaxNode::new(
                        SyntaxKind::BinExpr,
                        vec![left.into(), token.into(), right.into()],
                    )
                }
                _ => break,
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
                }
                _ => self.primary(),
            };
            return node;
        }
        panic!("No more tokens left");
    }

    fn primary(&mut self) -> SyntaxNode {
        if let Some(token) = self.peek() {
            self.advance();
            let node = match token.kind() {
                SyntaxKind::False
                | SyntaxKind::True
                | SyntaxKind::Nil
                | SyntaxKind::Number
                | SyntaxKind::String => SyntaxNode::new(SyntaxKind::Literal, vec![token.into()]),
                SyntaxKind::Identifier => {
                    SyntaxNode::new(SyntaxKind::Identifier, vec![token.into()])
                }
                _ => panic!("{:?} unimplemented", token.kind()),
            };
            return node;
        }
        panic!("No more tokens left");
    }

    fn peek(&self) -> Option<SyntaxToken> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn consume(&mut self, kind: SyntaxKind, error: &'static str) -> SyntaxToken {
        let token = self.peek().expect(error);
        if token.kind() != kind {
            panic!("{}", error);
        }
        self.advance();
        token
    }
}
