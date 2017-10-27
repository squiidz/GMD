#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LETTER(char),
    NUMBER(char),
    UNDERSCORE,
    HYPHEN,
    WHITESPACE,
    INVALID,
}

impl From<char> for Token {
    fn from(c: char) -> Token {
        if c.is_numeric() {
            return Token::NUMBER(c)
        } else if c.is_alphabetic() {
            return Token::LETTER(c)
        } else if c == '_' {
            return Token::UNDERSCORE
        } else if c == '-' {
            return Token::HYPHEN
        } else if c.is_whitespace() {
            return Token::WHITESPACE
        }
        Token::INVALID
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::NUMBER(n) => n,
            Token::LETTER(l) => l,
            Token::UNDERSCORE => '_',
            Token::HYPHEN => '_',
            Token::WHITESPACE => '_',
            Token::INVALID => '\0',
        }
    }
}
