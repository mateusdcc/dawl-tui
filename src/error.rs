use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub code: &'static str,
    pub message: String,
    pub hint: Option<String>,
    pub kind: ErrorKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    Input,
    Layout,
    Io,
}

impl Error {
    pub fn input(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, message, ErrorKind::Input)
    }

    pub fn layout(code: &'static str, message: impl Into<String>) -> Self {
        Self::new(code, message, ErrorKind::Layout)
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn exit_code(&self) -> i32 {
        match self.kind {
            ErrorKind::Input => 2,
            ErrorKind::Layout => 3,
            ErrorKind::Io => 4,
        }
    }

    fn new(code: &'static str, message: impl Into<String>, kind: ErrorKind) -> Self {
        Self { code, message: message.into(), hint: None, kind }
    }
}

impl Display for Error {
    fn fmt(&self, output: &mut Formatter<'_>) -> std::fmt::Result {
        write!(output, "{}: {}", self.code, self.message)?;
        if let Some(hint) = &self.hint {
            write!(output, "\nhint: {hint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::new("IO_ERROR", error.to_string(), ErrorKind::Io)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::input("JSON_INVALID", error.to_string())
    }
}
