#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::parser::matter::MatterParser;

    #[test]
    fn test_empty_frontmatter() {
        let input = "---\n---\n# Hello World";
        let parser = MatterParser::from_str(input).unwrap();
        assert!(parser.frontmatter().fields.is_empty());
        assert_eq!(parser.content(), "# Hello World");
    }

    #[test]
    fn test_simple_frontmatter() {
        let input = "---\ntitle: My Post\ndate: 2023-01-01\n---\n# Hello World";
        let parser = MatterParser::from_str(input).unwrap();

        assert_eq!(
            parser.frontmatter().get_string("title"),
            Some("My Post".to_string())
        );
        assert_eq!(
            parser.frontmatter().get_string("date"),
            Some("2023-01-01".to_string())
        );
        assert_eq!(parser.content(), "# Hello World");
    }

    #[test]
    fn test_no_frontmatter() {
        let input = "# Hello World\nThis is content.";
        let parser = MatterParser::from_str(input).unwrap();
        assert!(parser.frontmatter().fields.is_empty());
        assert_eq!(parser.content(), "# Hello World\nThis is content.");
    }

    #[test]
    fn test_array_frontmatter() {
        let input = r#"---
bullets:
  - rust
  - programming
  - tutorial
array: [rust, programming, tutorial]
  ---
# Hello World"#;
        let parser = MatterParser::from_str(input).unwrap();
        let bullets = parser.frontmatter().get_array("bullets").unwrap();
        assert_eq!(bullets.len(), 3);
        assert_eq!(bullets[0], "rust");
        assert_eq!(bullets[1], "programming");
        assert_eq!(bullets[2], "tutorial");

        let array = parser.frontmatter().get_array("array").unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], "rust");
        assert_eq!(array[1], "programming");
        assert_eq!(array[2], "tutorial");
    }

    #[test]
    fn test_complex_frontmatter() {
        let input = r#"---
title: My Post
tags:
  - rust
  - programming
  - tutorial
published: true
author:
  name: John Doe
  email: john@example.com
---
# Hello World"#;
        let parser = MatterParser::from_str(input).unwrap();

        assert_eq!(
            parser.frontmatter().get_string("title"),
            Some("My Post".to_string())
        );
        assert_eq!(parser.frontmatter().get_bool("published"), Some(true));

        let tags = parser.frontmatter().get_array("tags").unwrap();
        assert_eq!(tags.len(), 3);

        let author = parser
            .frontmatter()
            .get::<HashMap<String, Value>>("author")
            .unwrap();
        assert_eq!(
            author.get("name").and_then(|v| v.as_str()),
            Some("John Doe")
        );
    }
}
