use std::fs;

mod parser;

fn main() {
    let file_path = "fixtures/example.md";
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");

    let matter = parser::matter::MatterParser::from_str(&file_content).unwrap();
    let markdown = parser::markdown::MarkdownParser::tokenize(&matter.content());

    println!("{:?}", matter.frontmatter());
    println!("{:?}", markdown);
}
