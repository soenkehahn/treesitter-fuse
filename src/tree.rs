#[derive(Debug)]
pub enum Tree {
    Leaf { contents: String },
    Node { name: String, children: Vec<Tree> },
}
