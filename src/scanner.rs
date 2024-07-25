use crate::token::Token;

enum ScanError {
    InvalidToken,
}

struct TokenIterator<'a> {
    input: &'a [char],
}

impl Iterator for TokenIterator<'_> {
    type Item = Result<Token, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars: &[char] = self.input;

        let token = match chars.first()? {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBrace,
            ']' => Token::RightBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '-' => Token::Minus,
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '/' => Token::Slash,
            '*' => Token::Star,

            '!' => match chars.get(1) {
                Some('=') => Token::BangEqual,
                _ => Token::Bang,
            },

            '=' => match chars.get(1) {
                Some('=') => Token::EqualEqual,
                _ => Token::Equal,
            },
            '>' => match chars.get(1) {
                Some('=') => Token::GreaterEqual,
                _ => Token::Greater,
            },
            '<' => match chars.get(1) {
                Some('=') => Token::LessEqual,
                _ => Token::Less,
            },

            _ => return Some(Err(ScanError::InvalidToken)),
        };

        Some(Ok(token))
    }
}

fn scan_tokens(input: &str) -> Vec<Token> {
    [].into()
}
