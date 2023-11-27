use thiserror::Error;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("The file \"{0}\" does not exist!")]
    FileDoesNotExist(String),

    #[error("Attempt to treat \"{0}\" as a file failed! The path does exist but it not a file!")]
    PathExistsButNotFile(String),

    #[error("Attempt to get content of a binary file; this is not implemented yet!")]
    BinaryContentNotImplemented(String)
}
