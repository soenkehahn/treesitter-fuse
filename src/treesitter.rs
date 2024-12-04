use crate::tree::Contents;
use crate::tree::Tree;
use std::fs;
use std::path::Path;
use tree_sitter::Node;
use tree_sitter::Parser;

pub fn get_tree(path: &Path) -> Tree {
    let mut parser = Parser::new();
    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Rust parser");
    let code = fs::read_to_string(path).unwrap();
    let tree = parser.parse(&code, None).unwrap();
    let mut tree = to_tree(&code, tree.root_node());
    tree.uniquify_names();
    tree
}

fn to_tree(code: &str, node: Node) -> Tree {
    let mut children = vec![];
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        children.push(to_tree(code, child));
    }
    Tree {
        id: node.id().try_into().unwrap(),
        name: node.grammar_name().to_owned(),
        contents: if children.is_empty() {
            Contents::Leaf(node.utf8_text(code.as_bytes()).unwrap().to_owned())
        } else {
            Contents::Node(children)
        },
    }
}
