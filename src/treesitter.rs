use std::fs;
use tree_sitter::Parser;

pub fn run() {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Rust parser");
    let code = fs::read_to_string("./src/main.rs").unwrap();
    let tree = parser.parse(&code, None).unwrap();
    dbg!(&tree);
    let mut cursor = tree.root_node().walk();
    for c in tree.root_node().children(&mut cursor) {
        dbg!(c);
        eprintln!("{}", c.utf8_text(code.as_bytes()).unwrap());
    }
}
