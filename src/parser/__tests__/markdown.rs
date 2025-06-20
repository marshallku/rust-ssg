#[cfg(test)]
mod tests {
    use crate::parser::{markdown::MarkdownParser, types::Token};

    #[test]
    fn test_tokenize() {
        let input = "# Hello World\nThis is content.";
        let tokens = MarkdownParser::tokenize(input);
        assert_eq!(tokens[0], Token::Hash(1));
        assert_eq!(tokens[1], Token::Text("Hello World".to_string()));
        assert_eq!(tokens[2], Token::Newline);
        assert_eq!(tokens[3], Token::Text("This is content.".to_string()));
        assert_eq!(tokens.len(), 4);
    }
}
