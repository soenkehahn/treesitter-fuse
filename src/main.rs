mod fuse;
mod treesitter;

fn main() {
    crate::treesitter::run();
    crate::fuse::run();
}
