mod fuse;
mod tree;
mod treesitter;

fn main() {
    dbg!(crate::treesitter::get_tree());
    crate::fuse::run();
}
