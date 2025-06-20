use crate::parser::types::Token;

#[allow(dead_code)]
pub struct MarkdownParser {
    tokens: Vec<Token>,
    position: usize,
}

impl MarkdownParser {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '\n' => tokens.push(Token::Newline),
                '#' => {
                    let mut count = 1;
                    while chars.peek() == Some(&'#') {
                        chars.next();
                        count += 1;
                    }
                    tokens.push(Token::Hash(count));
                }
                '*' => {
                    let mut count = 1;
                    while chars.peek() == Some(&'*') {
                        chars.next();
                        count += 1;
                    }
                    tokens.push(Token::Star(count));
                }
                ' ' => {
                    // Skip leading spaces after special tokens
                    if let Some(last_token) = tokens.last() {
                        match last_token {
                            Token::Hash(_) | Token::Star(_) => {
                                continue;
                            }
                            _ => {
                                // Collect consecutive spaces
                                let mut spaces = String::from(ch);
                                while let Some(&next_ch) = chars.peek() {
                                    if next_ch == ' ' {
                                        spaces.push(chars.next().unwrap());
                                    } else {
                                        break;
                                    }
                                }
                                tokens.push(Token::Text(spaces));
                            }
                        }
                    } else {
                        // Leading space at the beginning
                        tokens.push(Token::Text(String::from(ch)));
                    }
                }
                // TODO: Add more token patterns...
                _ => {
                    // Collect text until special character or newline
                    let mut text = String::from(ch);
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' || "#*_`[]()!-".contains(next_ch) {
                            break;
                        }
                        text.push(chars.next().unwrap());
                    }
                    tokens.push(Token::Text(text));
                }
            }
        }
        tokens
    }
}
