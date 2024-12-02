use crate::tree::Tree;
use fuser::FileAttr;
use fuser::FileType;
use fuser::Filesystem;
use fuser::MountOption;
use fuser::ReplyAttr;
use fuser::ReplyData;
use fuser::ReplyDirectory;
use fuser::ReplyEntry;
use fuser::Request;
use std::ffi::OsStr;
use std::time::Duration;
use std::time::UNIX_EPOCH;

pub fn run(tree: Tree) {
    eprintln!("mounting on /tmp/mnt");
    fuser::mount2(
        TreeFs(Tree::Node {
            id: 1,
            name: "root".to_owned(),
            children: vec![tree],
        }),
        "/tmp/mnt",
        &vec![MountOption::AutoUnmount, MountOption::DefaultPermissions],
    )
    .unwrap();
}

#[derive(Debug)]
struct TreeFs(Tree);

impl TreeFs {
    fn tree_to_file_type(tree: &Tree) -> FileType {
        match tree {
            Tree::Node { .. } => FileType::Directory,
            Tree::Leaf { .. } => FileType::RegularFile,
        }
    }
}

impl Filesystem for TreeFs {
    fn getattr(&mut self, _req: &Request<'_>, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        eprintln!("getattr: {:?}", ino);
        let file_type = match self.0.get_by_id(ino) {
            None => {
                todo!();
            }
            Some(tree) => TreeFs::tree_to_file_type(&tree),
        };
        let attr = FileAttr {
            ino,
            size: 13,
            blocks: 1,
            atime: UNIX_EPOCH,
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: file_type,
            perm: 0o755,
            nlink: 1,
            uid: 1000,
            gid: 1000,
            rdev: 0,
            flags: 0,
            blksize: 512,
        };
        reply.attr(&Duration::from_secs(0), &attr);
    }

    fn readdir(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        eprintln!("readdir - offset: {:?}", offset);
        match offset {
            0 => match self.0.get_by_id(ino) {
                Some(Tree::Node { children, .. }) => {
                    for child in children {
                        assert!(!reply.add(
                            child.id(),
                            1,
                            TreeFs::tree_to_file_type(child),
                            child.name()
                        ));
                    }
                    reply.ok();
                }
                _ => todo!(),
            },
            _ => reply.ok(),
        }
    }

    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        eprintln!("lookup: {:?} / {:?}", parent, name);
        match self.0.get_by_id(parent) {
            Some(Tree::Node { children, .. }) => {
                let child = children
                    .iter()
                    .find(|child| child.name() == &name.to_string_lossy());
                match child {
                    Some(child) => {
                        let attr = FileAttr {
                            ino: child.id(),
                            size: 13,
                            blocks: 1,
                            atime: UNIX_EPOCH,
                            mtime: UNIX_EPOCH,
                            ctime: UNIX_EPOCH,
                            crtime: UNIX_EPOCH,
                            kind: TreeFs::tree_to_file_type(child),
                            perm: 0o755,
                            nlink: 1,
                            uid: 1000,
                            gid: 1000,
                            rdev: 0,
                            flags: 0,
                            blksize: 512,
                        };
                        reply.entry(&Duration::from_secs(0), &attr, 0);
                    }
                    None => reply.error(libc::ENOENT),
                }
            }
            _ => todo!(),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        _offset: i64,
        _size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        eprintln!("read");
        match self.0.get_by_id(ino) {
            Some(Tree::Leaf { contents, .. }) => reply.data(contents.as_bytes()),
            _ => todo!(),
        }
    }
}
