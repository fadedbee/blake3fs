pub mod block;
pub mod constants;
pub mod store;
pub mod misc;
pub mod blake3fs;
pub mod inode;

use std::env;
use std::ffi::OsStr;
use blake3fs::Blake3FS;

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=blake3fs"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();
    let blake3fs = Blake3FS::new();

    fuse::mount(blake3fs, &mountpoint, &options).unwrap();
}
