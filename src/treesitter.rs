use crate::tree::Tree;
use std::fs;
use tree_sitter::Node;
use tree_sitter::Parser;

pub fn get_tree() -> Tree {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Rust parser");
    let code = fs::read_to_string("./src/main.rs").unwrap();
    let tree = parser.parse(&code, None).unwrap();
    to_tree(&code, tree.root_node())
}

fn to_tree(code: &str, node: Node) -> Tree {
    let mut children = vec![];
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        children.push(to_tree(code, child));
    }
    if children.is_empty() {
        Tree::Leaf {
            contents: node.utf8_text(code.as_bytes()).unwrap().to_owned(),
        }
    } else {
        Tree::Node {
            name: node.grammar_name().to_owned(),
            children,
        }
    }
}
