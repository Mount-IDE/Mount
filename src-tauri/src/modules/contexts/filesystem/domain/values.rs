#[derive(Clone)]
pub enum FileType {
    BINARY,
    REGULAR
}

#[derive(Clone)]

pub enum FileWriteAccess{
    WRITE,
    APPEND
}