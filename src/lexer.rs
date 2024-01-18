use std::fmt;

use crate::KEYWORDS;

#[derive(Clone)]
pub enum LexerTokenType {
    ExpressionStatement,
    StringLiteral,
    EndOfStatement,
    Unknown,
}

impl fmt::Display for LexerTokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerTokenType::ExpressionStatement => write!(f, "ExpressionStatement"),
            LexerTokenType::StringLiteral => write!(f, "StringLiteral"),
            LexerTokenType::EndOfStatement => write!(f, "EndOfStatement"),
            LexerTokenType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Clone)]
pub struct LexerToken {
    pub token_type: LexerTokenType,
    pub value: String,
}

impl LexerToken {
    fn new(token_type: LexerTokenType, value: String) -> LexerToken {
        LexerToken { token_type, value }
    }
}

impl fmt::Display for LexerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.token_type, self.value)
    }
}

pub fn lex(source: String) -> Vec<LexerToken> {
    let keywords = KEYWORDS.to_vec();
    let mut tokens: Vec<LexerToken> = Vec::new();

    let mut current_token = String::new();
    let mut is_string = false; // inside a string flag
    let mut chars = source.chars().peekable(); // remove leading and trailing whitespaces

    while let Some(&c) = chars.peek() {
        match c {
            // a quote
            '"' => {
                current_token.push(c);

                if is_string {
                    tokens.push(token_with_type(current_token));
                    current_token = String::new();
                }

                is_string = !is_string;
                chars.next();
            }
            // comments
            '/' => {}
            ';' => {
                if is_string {
                    current_token.push(c);
                } else {
                    tokens.push(token_with_type(current_token));
                    current_token = String::new();
                }
                chars.next();
            }
            // whitespace types
            ' ' | '\n' | '\t' => {
                if keywords.contains(&current_token.as_str()) {
                    tokens.push(token_with_type(current_token));

                    current_token = String::new();
                }
                chars.next();
            }
            // characters
            _ if is_string || !c.is_whitespace() => {
                current_token.push(c);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    return tokens;
}

// Also, it doesn't handle the last token if it's not followed by whitespace.

fn token_with_type(token: String) -> LexerToken {
    match token.as_str() {
        "print" => LexerToken::new(LexerTokenType::ExpressionStatement, token),
        ";" => LexerToken::new(LexerTokenType::EndOfStatement, token),
        _ if token.chars().next() == Some('"') && token.chars().last() == Some('"') => {
            LexerToken::new(LexerTokenType::StringLiteral, token)
        }
        _ => LexerToken::new(LexerTokenType::Unknown, token),
    }
}
