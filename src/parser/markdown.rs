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
                // TODO: Add more token patterns...
                _ => {
                    // Collect text until special character
                    let mut text = String::from(ch);
                    while let Some(&next_ch) = chars.peek() {
                        if "\\n#*_`[]()!-".contains(next_ch) {
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
