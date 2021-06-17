#[derive(Debug, ::thiserror::Error)]
#[error(r#"GeneratorError. kind="{kind}""#)]
pub struct GeneratorError {
    #[from]
    kind: ErrorKind,
    #[cfg(feature = "nightly")]
    backtrace: std::backtrace::Backtrace,
}

#[derive(Debug, ::thiserror::Error)]
pub enum ErrorKind {
    #[error(r#"A conflicted identifier was found: "{name}""#)]
    ConflictedName { name: String },
    #[error(r#"Unknown enum value ({id}) was found in the field descriptor's type field."#)]
    UnknownFieldTypeId { id: i32 },
    #[error(r#"Unknown enum vvalue({id}) was found in the field descriptor's label field."#)]
    UnknownLabelId { id: i32 },
    #[error(r#"The type name "{name}" is not found in any other input .proto files."#)]
    UnknownTypeName { name: String },
    #[error(r#"The group feature is not yet supported. GIVE ME A DOCUMENT!!!"#)]
    GroupNotSupported,
    #[error(r#"Unknown value for proto file's syntax: "{name}"."#)]
    UnknownProtoSyntax { name: String },
    #[error(r#"The type "{name}" is marked as a map entry type, but it doesn't fulfill the requirement."#)]
    InvalidMapEntry { name: String },
    #[error(r#"The type "{name}" is specified as a map key type, but it doesn't fulfill the requirement."#)]
    InvalidMapKey { name: String },
    #[error(r#"The enum type "{name}" has no values. The empty enum is not allowed."#)]
    EmptyEnum { name: String },
    #[error(r#"An error from formatter: "{source}""#)]
    WriteError { source: std::fmt::Error },
    #[error(r#"An error from puroro: "{source}""#)]
    PuroroError { source: ::puroro::PuroroError },
    #[error(r#"Something went wrong: "{detail}""#)]
    InternalError { detail: String },
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
