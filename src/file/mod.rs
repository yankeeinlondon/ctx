use std::{time::SystemTime, fs::{metadata, read_to_string}};
use serde::{Serialize, Deserialize};
use tracing::instrument;

use crate::{errors::io::IoError, hasher::hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileWithMeta {
    pub meta: FileMeta,
    pub content: String,
    pub hash: u64
}

impl TryFrom<FileMeta> for FileWithMeta {
    type Error = IoError;

    #[instrument]
    fn try_from(value: FileMeta) -> Result<Self, Self::Error> {
        if let Ok(content) = read_to_string(&value.filename) {
            Ok(Self {
                hash: hash(&content),
                content,
                meta: value
            })
        } else {
            Err(IoError::PathExistsButNotFile(value.filename.clone()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMeta {
    filename: String,
    /// whether or not the file is a symlink reference to a file
    is_symlink: bool,
    /// the _last modified_ time of the file if the OS supports providing this
    modified: Option<SystemTime>,
    /// the _created_ time of the file if the OS supports providing this
    created: Option<SystemTime>
}

/// try to convert a string slice -- representing a file path -- into
/// a FileMeta structure.
impl TryFrom<&str> for FileMeta {
    type Error = IoError;

    #[instrument]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // let mut meta: Result<Metadata,Self::Error>;
        
        if let Ok(meta) = metadata(value) {
            if meta.is_file() {
                let mut modified: Option<SystemTime> = None;
                if let Ok(st) = meta.modified() {
                    modified = Some(st);
                }
                let mut created: Option<SystemTime> = None;
                if let Ok(st) = meta.created() {
                    created = Some(st);
                }

                Ok(Self {
                    filename: value.to_string(),
                    is_symlink: meta.is_symlink(),
                    modified,
                    created
                })
            } else {
                Err(IoError::PathExistsButNotFile(value.to_string()))
            }
        } else {
            Err(IoError::FileDoesNotExist(value.to_string()))
        }
    }
}

/// try to convert a &String -- representing a file path -- into
/// a FileMeta structure.
impl TryFrom<&String> for FileMeta {
    type Error = IoError;

    #[instrument]
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let value = value.as_str();
        FileMeta::try_from(value)
    }
}

impl FileMeta {
    /// attempt to upgrade the `FileMeta` to a `FileWithMeta` which
    /// includes the file's contents, a hash of these contents, along
    /// with all the prior metadata preserved.
    pub fn load_content(self) -> Result<FileWithMeta, IoError> {
        FileWithMeta::try_from(self)
    }
}
