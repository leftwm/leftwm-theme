use std::fmt;
use url::ParseError as urlParseError;
pub type Result<T> = std::result::Result<T, LeftError>;

#[derive(Debug)]
pub struct LeftError {
    pub inner: LeftErrorKind,
}

#[must_use]
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
    TomlSerialize(toml::ser::Error),
    ReqwestError(reqwest::Error),
    StreamError(),
    NoneError(),
    UserFriendlyError(String),
    GitError(git2::Error),
    Generic(String),
    ParseIntError(core::num::ParseIntError),
    SemVerError(semver::Error),
    UrlParseError(url::ParseError),
}

impl fmt::Display for LeftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.inner);
    }
}

impl fmt::Display for LeftErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LeftErrorKind::SerdeParse(ref err) => return write!(f, "{}", err),
            LeftErrorKind::UserFriendlyError(ref err) | LeftErrorKind::Generic(ref err) => {
                return write!(f, "{}", err)
            }
            LeftErrorKind::IoError(ref err) => return write!(f, "{}", err),
            LeftErrorKind::XdgBaseDirError(ref err) => return write!(f, "{}", err),
            LeftErrorKind::TomlParse(ref err) => return write!(f, "{}", err),
            LeftErrorKind::TomlSerialize(ref err) => return write!(f, "{}", err),
            LeftErrorKind::StreamError() => return write!(f, "Stream Error"),
            LeftErrorKind::NoneError() => return write!(f, "None Error"),
            LeftErrorKind::ReqwestError(ref err) => return write!(f, "Request Error: {}", err),
            LeftErrorKind::GitError(ref err) => return write!(f, "{}", err),
            LeftErrorKind::ParseIntError(ref err) => return write!(f, "{}", err),
            LeftErrorKind::SemVerError(ref err) => return write!(f, "{}", err),
            LeftErrorKind::UrlParseError(ref err) => return write!(f, "{}", err),
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

impl From<toml::ser::Error> for LeftError {
    fn from(inner: toml::ser::Error) -> LeftError {
        LeftErrorKind::TomlSerialize(inner).into()
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

impl From<semver::Error> for LeftError {
    fn from(inner: semver::Error) -> LeftError {
        LeftErrorKind::SemVerError(inner).into()
    }
}

impl From<urlParseError> for LeftError {
    fn from(inner: urlParseError) -> LeftError {
        LeftErrorKind::UrlParseError(inner).into()
    }
}
