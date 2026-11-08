use crate::services::{
    FileSystemReadService, FileSystemWatchService, FileSystemWriteService, FsEntityService,
};

pub mod domain;
pub mod managers;
pub mod services;
pub mod traits;
pub mod values;

pub const FS_READ_SERVICE: FileSystemReadService = FileSystemReadService;
pub const FS_WRITE_SERVICE: FileSystemWriteService = FileSystemWriteService;
pub const FS_WATCH_SERVICE: FileSystemWatchService = FileSystemWatchService;
pub const FS_ENTITY_SERVICE: FsEntityService = FsEntityService;
