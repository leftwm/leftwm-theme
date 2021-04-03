use std::fmt;
pub type Result<T> = std::result::Result<T, LeftError>;

#[derive(Debug)]
pub struct LeftError {
    pub inner: LeftErrorKind,
}

pub fn friendly_message(msg: &str) -> LeftError {
    LeftError {
        inner: LeftErrorKind::UserFriendlyError(msg.to_string()),
    }
}

#[derive(Debug)]
pub enum LeftErrorKind {
    SerdeParse(serde_json::error::Error),
    IoError(std::io::Error),
    XdgBaseDirError(xdg::BaseDirectoriesError),
    TomlParse(toml::de::Error),
    ReqwestError(reqwest::Error),
    StreamError(),
    NoneError(),
    UserFriendlyError(String),
    GitError(git2::Error),
    Generic(String),
    ParseIntError(core::num::ParseIntError),
    ReqParseError(semver::ReqParseError),
    SemVerError(semver::SemVerError),
}

impl fmt::Display for LeftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Display for LeftErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeftErrorKind::SerdeParse(ref err) => write!(f, "{}", err),
            LeftErrorKind::UserFriendlyError(ref err) => write!(f, "{}", err),
            LeftErrorKind::IoError(ref err) => write!(f, "{}", err),
            LeftErrorKind::XdgBaseDirError(ref err) => write!(f, "{}", err),
            LeftErrorKind::TomlParse(ref err) => write!(f, "{}", err),
            LeftErrorKind::StreamError() => write!(f, "Stream Error"),
            LeftErrorKind::NoneError() => write!(f, "None Error"),
            LeftErrorKind::ReqwestError(ref err) => write!(f, "Request Error: {}", err),
            LeftErrorKind::GitError(ref err) => write!(f, "{}", err),
            LeftErrorKind::Generic(ref err) => write!(f, "{}", err),
            LeftErrorKind::ParseIntError(ref err) => write!(f, "{}", err),
            LeftErrorKind::ReqParseError(ref err) => write!(f, "{}", err),
            LeftErrorKind::SemVerError(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<LeftErrorKind> for LeftError {
    fn from(inner: LeftErrorKind) -> LeftError {
        LeftError { inner }
    }
}

impl From<serde_json::error::Error> for LeftError {
    fn from(inner: serde_json::error::Error) -> LeftError {
        LeftErrorKind::SerdeParse(inner).into()
    }
}

impl From<std::io::Error> for LeftError {
    fn from(inner: std::io::Error) -> LeftError {
        LeftErrorKind::IoError(inner).into()
    }
}

impl From<xdg::BaseDirectoriesError> for LeftError {
    fn from(inner: xdg::BaseDirectoriesError) -> LeftError {
        LeftErrorKind::XdgBaseDirError(inner).into()
    }
}

impl From<toml::de::Error> for LeftError {
    fn from(inner: toml::de::Error) -> LeftError {
        LeftErrorKind::TomlParse(inner).into()
    }
}

impl From<reqwest::Error> for LeftError {
    fn from(inner: reqwest::Error) -> LeftError {
        LeftErrorKind::ReqwestError(inner).into()
    }
}

impl From<&str> for LeftError {
    fn from(_s: &str) -> LeftError {
        LeftErrorKind::NoneError().into()
    }
}

impl From<git2::Error> for LeftError {
    fn from(inner: git2::Error) -> LeftError {
        LeftErrorKind::GitError(inner).into()
    }
}

impl From<core::num::ParseIntError> for LeftError {
    fn from(inner: core::num::ParseIntError) -> LeftError {
        LeftErrorKind::ParseIntError(inner).into()
    }
}

impl From<semver::SemVerError> for LeftError {
    fn from(inner: semver::SemVerError) -> LeftError {
        LeftErrorKind::SemVerError(inner).into()
    }
}

impl From<semver::ReqParseError> for LeftError {
    fn from(inner: semver::ReqParseError) -> LeftError {
        LeftErrorKind::ReqParseError(inner).into()
    }
}
