#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Text(String),
    Newline,
    Hash(usize),
    Star(usize),
    Underscore(usize),
    Backtick(usize),
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Bang,
    Dash(usize),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Block {
    Heading(usize, Vec<Inline>),
    Paragraph(Vec<Inline>),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Inline {
    Text(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Code(String),
    Link(String, String),
    Image(String, String),
}
