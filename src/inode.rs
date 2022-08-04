use fuse::FileAttr;

#[derive(Debug)]
pub struct Inode {
    pub attr: FileAttr
}
