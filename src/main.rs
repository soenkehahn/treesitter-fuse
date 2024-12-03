mod fuse;
mod tree;
mod treesitter;

fn main() {
    let tree = crate::treesitter::get_tree();
    crate::fuse::run(tree);
}
