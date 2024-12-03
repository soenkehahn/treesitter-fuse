mod fuse;
mod tree;
mod treesitter;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let tree = crate::treesitter::get_tree(&args.path);
    dbg!(&tree);
    crate::fuse::run(tree);
}
