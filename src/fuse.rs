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

pub fn run() {
    eprintln!("mounting on /tmp/mnt");
    fuser::mount2(
        HelloFS,
        "/tmp/mnt",
        &vec![MountOption::AutoUnmount, MountOption::DefaultPermissions],
    )
    .unwrap();
}

struct HelloFS;

impl Filesystem for HelloFS {
    fn getattr(&mut self, _req: &Request<'_>, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        eprintln!("getattr: {:?}", ino);
        let attr = FileAttr {
            ino,
            size: 13,
            blocks: 1,
            atime: UNIX_EPOCH,
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: match ino {
                1 => FileType::Directory,
                42 => FileType::RegularFile,
                _ => FileType::RegularFile,
            },
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
        eprintln!("readdir");
        dbg!(offset);
        match ino {
            1 => match offset {
                0 => {
                    reply.add(42, 1, FileType::RegularFile, "foo");
                    reply.ok();
                }
                _ => {
                    reply.ok();
                }
            },
            _ => reply.error(libc::ENOENT),
        }
    }

    fn lookup(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEntry) {
        eprintln!("lookup: {:?} / {:?}", parent, name);
        let attr = FileAttr {
            ino: 42,
            size: 13,
            blocks: 1,
            atime: UNIX_EPOCH,
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::RegularFile,
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

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        eprintln!("read");
        dbg!((ino, offset, size));
        reply.data("huhu".as_bytes());
    }
}
