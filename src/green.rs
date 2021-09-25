use crate::kinds::SyntaxKind;
use std::fmt;

#[derive(Debug, Clone)]
pub enum NodeOrToken<N, T> {
    Node(N),
    Token(T),
}

impl<N, T> NodeOrToken<N, T> {
    pub fn into_node(self) -> Option<N> {
        match self {
            NodeOrToken::Node(node) => Some(node),
            NodeOrToken::Token(_) => None,
        }
    }

    pub fn into_token(self) -> Option<T> {
        match self {
            NodeOrToken::Node(_) => None,
            NodeOrToken::Token(token) => Some(token),
        }
    }
}

pub type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    text: String,
}

impl SyntaxToken {
    pub fn new(kind: SyntaxKind, text: String) -> SyntaxToken {
        SyntaxToken { kind, text }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind.clone()
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}

impl fmt::Display for SyntaxToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

impl From<SyntaxToken> for SyntaxElement {
    fn from(token: SyntaxToken) -> Self {
        NodeOrToken::Token(token)
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxNode {
    kind: SyntaxKind,
    children: Vec<NodeOrToken<SyntaxNode, SyntaxToken>>,
}

impl SyntaxNode {
    pub fn new(
        kind: SyntaxKind,
        children: Vec<NodeOrToken<SyntaxNode, SyntaxToken>>,
    ) -> SyntaxNode {
        SyntaxNode { kind, children }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind.clone()
    }

    /// Get a reference to the syntax node's children.
    pub fn children(&self) -> impl Iterator<Item = SyntaxElement> + '_ {
        self.children.iter().cloned()
    }
}

impl From<SyntaxNode> for SyntaxElement {
    fn from(node: SyntaxNode) -> Self {
        NodeOrToken::Node(node)
    }
}

impl fmt::Display for SyntaxNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for child in self.children() {
            fmt::Display::fmt(&child, f)?
        }
        Ok(())
    }
}

impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeOrToken::Node(node) => fmt::Display::fmt(node, f),
            NodeOrToken::Token(token) => fmt::Display::fmt(token, f),
        }
    }
}
