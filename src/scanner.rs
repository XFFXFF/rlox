use crate::kinds::SyntaxKind;

#[derive(Clone, PartialEq, Eq)]
struct SyntaxToken {
    kind: SyntaxKind,
    text: String,
}

impl SyntaxToken {
    fn new(kind: SyntaxKind, text: &str) -> SyntaxToken {
        SyntaxToken {
            kind,
            text: text.to_string(),
        }
    }

    fn kind(&self) -> SyntaxKind {
        self.kind.clone()
    }

    fn text(&self) -> &str {
        self.text.as_str()
    }
}

struct Scanner {
    source: String,
    tokens: Vec<SyntaxToken>,
    start: usize,
    current: usize,
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> impl Iterator<Item = &SyntaxToken> {
        while let Some(c) = self.advance() {
            self.start = self.current - 1;
            match c {
                '(' => self.add_token(SyntaxKind::LeftParen),
                ')' => self.add_token(SyntaxKind::RightParen),
                '{' => self.add_token(SyntaxKind::LeftBrace),
                '}' => self.add_token(SyntaxKind::RightBrace),
                ',' => self.add_token(SyntaxKind::Comma),
                '.' => self.add_token(SyntaxKind::Dot),
                '-' => self.add_token(SyntaxKind::Minus),
                '+' => self.add_token(SyntaxKind::Plus),
                ';' => self.add_token(SyntaxKind::Semicolon),
                '*' => self.add_token(SyntaxKind::Star),
                _ => {}
            }
        }
        self.tokens.iter()
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.source.chars().nth(self.current);
        self.current += 1;
        char
    }

    fn add_token(&mut self, kind: SyntaxKind) {
        let text = &self.source[self.start..self.current];
        let token = SyntaxToken::new(kind, text);
        self.tokens.push(token);
    }
}

#[cfg(test)]
mod tests {
    use super::{Scanner, SyntaxToken};
    use crate::kinds::SyntaxKind;

    fn test_scan_one_token(source: &str, kind: SyntaxKind) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan().collect::<Vec<&SyntaxToken>>();
        assert_eq!(tokens.len(), 1);
        let &token = tokens.first().unwrap();
        assert_eq!(token.kind(), kind);
        assert_eq!(token.text(), source);
    }

    #[test]
    fn test_single_char_token() {
        test_scan_one_token("(", SyntaxKind::LeftParen);
        test_scan_one_token(")", SyntaxKind::RightParen);
        test_scan_one_token("{", SyntaxKind::LeftBrace);
        test_scan_one_token("}", SyntaxKind::RightBrace);
        test_scan_one_token(",", SyntaxKind::Comma);
        test_scan_one_token(".", SyntaxKind::Dot);
        test_scan_one_token("-", SyntaxKind::Minus);
        test_scan_one_token("+", SyntaxKind::Plus);
        test_scan_one_token(";", SyntaxKind::Semicolon);
        test_scan_one_token("*", SyntaxKind::Star);
        // test_scan_one_token("!", SyntaxKind::Bang);
        // test_scan_one_token("=", SyntaxKind::Equal);
        // test_scan_one_token("<", SyntaxKind::Less);
        // test_scan_one_token(">", SyntaxKind::Greater)
    }
}
