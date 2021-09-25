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
