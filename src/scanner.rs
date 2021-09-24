use crate::kinds::SyntaxKind;

#[derive(Clone, PartialEq, Eq, Debug)]
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

                // two-char-tokens
                #[rustfmt::skip]
                '!' => self.add_token_based_on_next_char(
                    '=',
                    SyntaxKind::BangEqual,
                    SyntaxKind::Bang,
                ),
                '=' => self.add_token_based_on_next_char(
                    '=',
                    SyntaxKind::EqualEqual,
                    SyntaxKind::Equal,
                ),
                #[rustfmt::skip]
                '<' => self.add_token_based_on_next_char(
                    '=',
                    SyntaxKind::LessEqual,
                    SyntaxKind::Less
                ),
                '>' => self.add_token_based_on_next_char(
                    '=',
                    SyntaxKind::GreaterEqual,
                    SyntaxKind::Greater,
                ),

                // slash
                '/' => self.slash(),

                _ => {}
            }
        }
        self.tokens.iter()
    }

    fn current(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.current();
        self.current += 1;
        char
    }

    fn add_token(&mut self, kind: SyntaxKind) {
        let text = &self.source[self.start..self.current];
        let token = SyntaxToken::new(kind, text);
        self.tokens.push(token);
    }

    fn add_token_based_on_next_char(
        &mut self,
        expected: char,
        expected_syntax_kind: SyntaxKind,
        otherwise_syntax_kind: SyntaxKind,
    ) {
        if let Some(c) = self.current() {
            if c == expected {
                self.current += 1;
                self.add_token(expected_syntax_kind);
            } else {
                self.add_token(otherwise_syntax_kind);
            }
        } else {
            self.add_token(otherwise_syntax_kind);
        }
    }

    fn slash(&mut self) {
        if let Some(current) = self.current() {
            if current == '/' {
                while let Some(next) = self.advance() {
                    if next == '\n' {
                        break;
                    }
                }
            } else {
                self.add_token(SyntaxKind::Slash);
            }
        } else {
            self.add_token(SyntaxKind::Slash);
        }
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

    fn test_scan_one_token_with_text(source: &str, kind: SyntaxKind, text: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan().collect::<Vec<&SyntaxToken>>();
        assert_eq!(tokens.len(), 1);
        let &token = tokens.first().unwrap();
        assert_eq!(token.kind(), kind);
        assert_eq!(token.text(), text);
    }

    fn test_scan_expected_empty(source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan().collect::<Vec<&SyntaxToken>>();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn single_char_token() {
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
        test_scan_one_token("!", SyntaxKind::Bang);
        test_scan_one_token("=", SyntaxKind::Equal);
        test_scan_one_token("<", SyntaxKind::Less);
        test_scan_one_token(">", SyntaxKind::Greater)
    }

    #[test]
    fn two_chars_token() {
        test_scan_one_token("!=", SyntaxKind::BangEqual);
        test_scan_one_token("==", SyntaxKind::EqualEqual);
        test_scan_one_token("<=", SyntaxKind::LessEqual);
        test_scan_one_token(">=", SyntaxKind::GreaterEqual);
    }

    #[test]
    fn slash() {
        test_scan_one_token("/", SyntaxKind::Slash);
        test_scan_expected_empty("//");
        test_scan_expected_empty("//asdfasdf");
        test_scan_one_token_with_text("//asdfasdf\n/", SyntaxKind::Slash, "/");
    }
}
