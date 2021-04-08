#[derive(Debug)]
pub(crate) struct GeneratorError {
    kind: ErrorKind,
    #[cfg(feature = "nightly")]
    backtrace: std::backtrace::Backtrace,
}
impl std::error::Error for GeneratorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind {
            ErrorKind::WriteError { source: ref e } => Some(e),
            _ => None,
        }
    }

    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        Some(&self.backtrace)
    }
}
impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
#[derive(Debug)]
pub(crate) enum ErrorKind {
    ConflictedName { name: String },
    UnknownFieldTypeId { id: i32 },
    UnknownLabelId { id: i32 },
    WriteError { source: std::fmt::Error },
    PuroroError { source: ::puroro::PuroroError },
}
impl From<ErrorKind> for GeneratorError {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
impl From<std::fmt::Error> for GeneratorError {
    fn from(e: std::fmt::Error) -> Self {
        Self {
            kind: ErrorKind::WriteError { source: e },
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
impl From<::puroro::PuroroError> for GeneratorError {
    fn from(e: ::puroro::PuroroError) -> Self {
        Self {
            kind: ErrorKind::PuroroError { source: e },
            #[cfg(feature = "nightly")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
