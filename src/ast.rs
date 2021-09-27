use crate::green::{SyntaxElement, SyntaxNode, SyntaxToken};
use crate::kinds::SyntaxKind;

pub trait AstNode {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    fn syntax(&self) -> &SyntaxNode;
}

pub struct Literal(SyntaxNode);
impl AstNode for Literal {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::Literal {
            Some(Literal(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl Literal {
    pub fn token(&self) -> SyntaxToken {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_token)
            .unwrap()
    }
}

pub struct UnaryExpr(SyntaxNode);
impl AstNode for UnaryExpr {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::UnaryExpr {
            Some(UnaryExpr(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl UnaryExpr {
    pub fn op(&self) -> SyntaxToken {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_token)
            .unwrap()
    }

    pub fn node(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_node)
            .unwrap()
    }
}

pub struct BinExpr(SyntaxNode);
impl AstNode for BinExpr {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::BinExpr {
            Some(BinExpr(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl BinExpr {
    pub fn left(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_node)
            .unwrap()
    }

    pub fn op(&self) -> SyntaxToken {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_token)
            .unwrap()
    }

    pub fn right(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .filter_map(SyntaxElement::into_node)
            .last()
            .unwrap()
    }
}

pub struct Print(SyntaxNode);
impl AstNode for Print {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::Print {
            Some(Print(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl Print {
    pub fn expr(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_node)
            .unwrap()
    }
}

pub struct VarDeclaration(SyntaxNode);
impl AstNode for VarDeclaration {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::Var {
            Some(VarDeclaration(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl VarDeclaration {
    pub fn ident(&self) -> SyntaxToken {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_token)
            .unwrap()
    }

    pub fn initializer(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .find_map(SyntaxElement::into_node)
            .unwrap()
    }
}

pub struct Identifier(SyntaxNode);
impl AstNode for Identifier {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::Identifier {
            Some(Identifier(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl Identifier {
    pub fn name(&self) -> String {
        let token = self
            .syntax()
            .children()
            .find_map(SyntaxElement::into_token)
            .unwrap();
        token.text().to_string()
    }
}

pub struct Block(SyntaxNode);
impl AstNode for Block {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::Block {
            Some(Block(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl Block {
    pub fn children(&self) -> impl Iterator<Item = SyntaxNode> + '_ {
        self.syntax()
            .children()
            .filter_map(SyntaxElement::into_node)
    }
}

pub struct If(SyntaxNode);
impl AstNode for If {
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if node.kind() == SyntaxKind::If {
            Some(If(node))
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl If {
    pub fn condition(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .filter_map(SyntaxElement::into_node)
            .next()
            .unwrap()
    }

    pub fn then_branch(&self) -> SyntaxNode {
        self.syntax()
            .children()
            .filter_map(SyntaxElement::into_node)
            .nth(1)
            .unwrap()
    }

    pub fn else_branch(&self) -> Option<SyntaxNode> {
        self.syntax()
            .children()
            .filter_map(SyntaxElement::into_node)
            .nth(2)
    }
}
