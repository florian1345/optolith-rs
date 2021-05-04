use std::io;
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;

/// An enumeration of all kinds errors that can occur in this program.
#[derive(Debug)]
pub enum OptolithDataErrorKind {
    IOError(io::Error),
    JSONError(serde_json::Error),
    YAMLError(serde_yaml::Error)
}

impl Display for OptolithDataErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OptolithDataErrorKind::IOError(e) => e.fmt(f),
            OptolithDataErrorKind::JSONError(e) => e.fmt(f),
            OptolithDataErrorKind::YAMLError(e) => e.fmt(f)
        }
    }
}

/// Wraps an error kind with the file where the error occurred.
#[derive(Debug)]
pub struct OptolithDataError {
    kind: OptolithDataErrorKind,
    file: Option<OsString>
}

impl OptolithDataError {
    fn set_file(&mut self, file: OsString) {
        self.file = Some(file);
    }

    pub fn kind(&self) -> &OptolithDataErrorKind {
        &self.kind
    }

    pub fn file(&self) -> &Option<OsString> {
        &self.file
    }
}

impl From<io::Error> for OptolithDataError {
    fn from(e: io::Error) -> Self {
        OptolithDataError {
            kind: OptolithDataErrorKind::IOError(e),
            file: None
        }
    }
}

impl From<serde_json::Error> for OptolithDataError {
    fn from(e: serde_json::Error) -> Self {
        OptolithDataError {
            kind: OptolithDataErrorKind::JSONError(e),
            file: None
        }
    }
}

impl From<serde_yaml::Error> for OptolithDataError {
    fn from(e: serde_yaml::Error) -> Self {
        OptolithDataError {
            kind: OptolithDataErrorKind::YAMLError(e),
            file: None
        }
    }
}

/// Syntactic sugar for a result value that can also be an [OptolithDataError].
pub type OptolithDataResult<T> = Result<T, OptolithDataError>;

pub fn set_file<T>(r: &mut OptolithDataResult<T>, file: &PathBuf) {
    match r {
        Ok(_) => { },
        Err(e) => e.set_file(OsString::from(file.as_os_str()))
    }
}
