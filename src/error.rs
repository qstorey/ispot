use std::error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// This error occurs if the user fails to authenticate with Spotify.
    AuthenticationFailure(String),
    /// This error occurs if the file is not found.
    FileNotFound(String),
    /// This error occurs if multiple results are returned and we expected a different amount.
    MultipleResults(u32),
    /// This error occurs if a directory is found instead of a file.
    NotAFile(String),
    /// This error occurs if no results are returned from the Spotify API.
    NoResults,
    /// This error occurs when trying to load a plist (iTunes playlist).
    PlistError(String),
    /// This error occurs if an API calls is not authorized.
    Unauthorized,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::AuthenticationFailure(_) => "failure authenticating with Spotify",
            ErrorKind::FileNotFound(_) => "file not found",
            ErrorKind::MultipleResults(_) => "multiple results returned",
            ErrorKind::NotAFile(_) => "not a file",
            ErrorKind::NoResults => "no results returned",
            ErrorKind::PlistError(_) => "an error occurred loading a plist",
            ErrorKind::Unauthorized => "unauthorized api call",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::AuthenticationFailure(ref s) => write!(f, "{}", s),
            ErrorKind::FileNotFound(ref s) => write!(f, "{}", s),
            ErrorKind::MultipleResults(count) => write!(f, "expected 1 result, found {}", count),
            ErrorKind::NotAFile(ref s) => write!(f, "{}", s),
            ErrorKind::NoResults => write!(f, "no results returned"),
            ErrorKind::PlistError(ref s) => write!(f, "{}", s),
            ErrorKind::Unauthorized => write!(f, "unauthorized api call"),
        }
    }
}
