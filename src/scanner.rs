use crate::kinds::SyntaxKind;
use crate::green::SyntaxToken;

macro_rules! is_digit {
    ($c: expr) => {
        $c >= '0' && $c <= '9'
    };
}

macro_rules! is_alpha {
    ($c: expr) => {
        ($c >= 'a' && $c <= 'z') || ($c >= 'A' && $c <= 'Z') || $c == '_'
    };
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

                '/' => self.slash(),

                ' ' | '\r' | '\t' | '\n' => {}

                '"' => self.string(),

                '0'..='9' => self.number(),

                'a'..='z' | 'A'..='Z' | '_' => self.identifier(),

                _ => panic!("Unexpected character."),
            }
        }
        self.tokens.iter()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn advance(&mut self) -> Option<char> {
        let char = self.peek();
        self.current += 1;
        char
    }

    fn add_token(&mut self, kind: SyntaxKind) {
        let text = &self.source[self.start..self.current];
        let token = SyntaxToken::new(kind, text.to_string());
        self.tokens.push(token);
    }

    fn add_token_based_on_next_char(
        &mut self,
        expected: char,
        expected_syntax_kind: SyntaxKind,
        otherwise_syntax_kind: SyntaxKind,
    ) {
        if let Some(c) = self.peek() {
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
        if let Some(current) = self.peek() {
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

    fn string(&mut self) {
        while let Some(c) = self.advance() {
            if c == '"' {
                self.add_token(SyntaxKind::String);
                return;
            }
        }
        panic!("Unterminated string.")
    }

    fn number(&mut self) {
        while let Some(c) = self.advance() {
            if !is_digit!(c) && c != '.' {
                break;
            }
        }
        self.current -= 1;
        self.add_token(SyntaxKind::Number);
    }

    fn identifier(&mut self) {
        while let Some(c) = self.advance() {
            if !is_alpha!(c) {
                break;
            }
        }
        self.current -= 1;
        let text = &self.source[self.start..self.current];
        match text {
            "and" => self.add_token(SyntaxKind::And),
            "class" => self.add_token(SyntaxKind::Class),
            "else" => self.add_token(SyntaxKind::Else),
            "false" => self.add_token(SyntaxKind::False),
            "for" => self.add_token(SyntaxKind::For),
            "fun" => self.add_token(SyntaxKind::Fun),
            "if" => self.add_token(SyntaxKind::If),
            "nil" => self.add_token(SyntaxKind::Nil),
            "or" => self.add_token(SyntaxKind::Or),
            "print" => self.add_token(SyntaxKind::Print),
            "return" => self.add_token(SyntaxKind::Return),
            "super" => self.add_token(SyntaxKind::Super),
            "this" => self.add_token(SyntaxKind::This),
            "true" => self.add_token(SyntaxKind::True),
            "var" => self.add_token(SyntaxKind::Var),
            "while" => self.add_token(SyntaxKind::While),
            _ => self.add_token(SyntaxKind::Identifier),
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

    #[test]
    fn whitespace() {
        test_scan_expected_empty(" ");
        test_scan_expected_empty("\r");
        test_scan_expected_empty("\t");
        test_scan_expected_empty("  ");
        test_scan_expected_empty("\n");
        test_scan_expected_empty(
            "
        ",
        );
    }

    #[test]
    fn string() {
        test_scan_one_token("\"hello\"", SyntaxKind::String);
        test_scan_one_token("\"\"", SyntaxKind::String);
        test_scan_one_token("\" \"", SyntaxKind::String);
    }

    #[test]
    fn number() {
        test_scan_one_token("123", SyntaxKind::Number);
        // TODO: should panic?
        test_scan_one_token("0123", SyntaxKind::Number);
        test_scan_one_token("123.", SyntaxKind::Number);
        test_scan_one_token("123.43", SyntaxKind::Number);
        // TODO: should panic?
        test_scan_one_token("123..43", SyntaxKind::Number);
        // TODO: should panic?
        test_scan_one_token("123.4.3", SyntaxKind::Number);
    }

    #[test]
    fn keyword() {
        test_scan_one_token("and", SyntaxKind::And);
        test_scan_one_token("class", SyntaxKind::Class);
        test_scan_one_token("else", SyntaxKind::Else);
        test_scan_one_token("false", SyntaxKind::False);
        test_scan_one_token("for", SyntaxKind::For);
        test_scan_one_token("fun", SyntaxKind::Fun);
        test_scan_one_token("if", SyntaxKind::If);
        test_scan_one_token("nil", SyntaxKind::Nil);
        test_scan_one_token("or", SyntaxKind::Or);
        test_scan_one_token("print", SyntaxKind::Print);
        test_scan_one_token("return", SyntaxKind::Return);
        test_scan_one_token("super", SyntaxKind::Super);
        test_scan_one_token("this", SyntaxKind::This);
        test_scan_one_token("true", SyntaxKind::True);
        test_scan_one_token("var", SyntaxKind::Var);
        test_scan_one_token("while", SyntaxKind::While);
    }

    #[test]
    fn identifier() {
        test_scan_one_token("key", SyntaxKind::Identifier);
        test_scan_one_token("_key", SyntaxKind::Identifier);
        test_scan_one_token("__key", SyntaxKind::Identifier);
        test_scan_one_token("k_e_y", SyntaxKind::Identifier);
    }
}
