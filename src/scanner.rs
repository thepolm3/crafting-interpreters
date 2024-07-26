use crate::token::Token;

#[derive(Debug)]
pub enum ScanError {
    InvalidToken,
    MismatchedQuote,
}

struct TokenIterator<'a> {
    input: &'a str,
}

impl Iterator for TokenIterator<'_> {
    type Item = Result<Token, ScanError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.char_indices();

        let (first_index, first) = chars.next()?;

        if first.is_whitespace() {
            let whitespace_index = chars.find(|(_, c)| !c.is_whitespace())?.0;

            self.input = &self.input[whitespace_index..];
            return self.next();
        }
        let single_token: Option<Token> = match first {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '[' => Some(Token::LeftBrace),
            ']' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '/' => Some(Token::Slash),
            '*' => Some(Token::Star),
            _ => None,
        };
        if let Some(token) = single_token {
            self.input = &self.input[first_index + 1..];
            return Some(Ok(token));
        }

        if first == '"' {
            println!("{}", self.input);
            if let Some((index, _)) = chars.find(|&(_, x)| x == '"') {
                println!("hi");
                let token = Token::String(self.input[1..index].to_owned());
                self.input = &self.input[index + 1..];
                return Some(Ok(token));
            } else {
                self.input = "";
                return Some(Err(ScanError::MismatchedQuote));
            }
        }

        if first.is_numeric() {
            let index = chars
                .find(|(_, c)| !(c.is_numeric()))
                .map(|t| t.0)
                .unwrap_or(self.input.len());
            let token = Token::Number(self.input[..index].to_owned());
            self.input = &self.input[index..];
            return Some(Ok(token));
        }

        let second = chars.next();
        let double_token = match first {
            '!' => Some(match second {
                Some((index, '=')) => (index, Token::BangEqual),
                _ => (first_index, Token::Bang),
            }),

            '=' => Some(match second {
                Some((index, '=')) => (index, Token::EqualEqual),
                _ => (first_index, Token::Equal),
            }),

            '>' => Some(match second {
                Some((index, '=')) => (index, Token::GreaterEqual),
                _ => (first_index, Token::Greater),
            }),

            '<' => Some(match second {
                Some((index, '=')) => (index, Token::LessEqual),
                _ => (first_index, Token::Less),
            }),

            _ => None,
        };

        if let Some((index, token)) = double_token {
            self.input = &self.input[index + 1..];
            return Some(Ok(token));
        }

        let end_of_word = chars
            .find(|(_, c)| !c.is_alphanumeric())
            .map(|t| t.0)
            .unwrap_or(self.input.len());
        let word = &self.input[..end_of_word];
        self.input = &self.input[end_of_word..];

        for (keyword, token) in [
            ("and", Token::And),
            ("class", Token::Class),
            ("else", Token::Else),
            ("false", Token::False),
            ("fun", Token::Fun),
            ("for", Token::For),
            ("if", Token::If),
            ("nil", Token::Nil),
            ("or", Token::Or),
            ("print", Token::Print),
            ("return", Token::Return),
            ("super", Token::Super),
            ("this", Token::This),
            ("true", Token::True),
            ("var", Token::Var),
            ("while", Token::While),
        ] {
            if word == keyword {
                return Some(Ok(token));
            }
        }

        return Some(Ok(Token::Identifier(word.to_owned())));
    }
}

pub fn scan_tokens(input: &str) -> Result<Vec<Token>, ScanError> {
    TokenIterator { input }.collect()
}
