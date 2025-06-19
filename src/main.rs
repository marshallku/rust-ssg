use std::fs;

mod parser;

fn main() {
    let file_path = "fixtures/example.md";
    let file_content = fs::read_to_string(file_path).expect("Failed to read file");

    let parser = parser::markdown::MarkdownParser::tokenize(&file_content);
    println!("{:?}", parser);
}
