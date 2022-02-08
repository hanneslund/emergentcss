use crate::token::Token;

pub fn get_tokens(str: &str) -> Vec<Token> {
    let mut chars = str.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(char) = chars.next() {
        match char {
            '@' => tokens.push(Token::At),
            ':' => tokens.push(Token::Colon),
            '=' => tokens.push(Token::Eq),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '_' => tokens.push(Token::Underscore),

            // Raw value
            '[' => {
                let mut raw_value = String::new();
                loop {
                    match chars.next() {
                        Some(']') => break,
                        Some(char) => raw_value.push(char),
                        None => panic!("Expected ]"),
                    }
                }
                tokens.push(Token::RawValue(raw_value));
            }

            // Word
            char if char.is_alphanumeric() || char == '-' => {
                let mut word = String::from(char);
                while let Some(char) = chars.peek() {
                    if char.is_alphanumeric() || *char == '-' || *char == '.' || *char == '/' {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Word(word));
            }

            c if c.is_whitespace() => {
                tokens.push(Token::Whitespace);
            }

            _ => panic!("Unknown char: {}", char),
        }
    }

    tokens
}
