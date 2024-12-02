mod fuse;
mod tree;
mod treesitter;

fn main() {
    let tree = crate::treesitter::get_tree();
    dbg!(&tree);
    crate::fuse::run(tree);
}
