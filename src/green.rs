use crate::kinds::SyntaxKind;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SyntaxToken {
    kind: SyntaxKind,
    text: String,
}

impl SyntaxToken {
    pub fn new(kind: SyntaxKind, text: String) -> SyntaxToken {
        SyntaxToken {
            kind,
            text,
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind.clone()
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}


