//! Absolute protobuf type name (fully-qualified, leading `.`).
//!
//! protoc's `CodeGeneratorRequest` always supplies absolute type names for
//! fields and extensions. Relative names are out of scope for this crate.

use ::std::borrow::Borrow;
use ::std::fmt;
use ::std::ops::Deref;

/// Absolute protobuf FQN, always stored with a leading `.`
/// (e.g. `.example.v1.Task`, `.Outer.Inner`).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProtoFqn(String);

impl ProtoFqn {
    /// Parse a type name from protoc / tests, normalising a missing leading `.`.
    pub fn parse(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        if name.starts_with('.') {
            Self(name.to_owned())
        } else {
            Self(format!(".{name}"))
        }
    }

    /// Build an FQN from a protobuf `package` and nested simple-name path.
    ///
    /// `package` may be empty. `path_from_package` is e.g. `["Outer", "Inner"]`.
    pub fn from_package_path(package: &str, path_from_package: &[&str]) -> Self {
        let mut out = String::from(".");
        if !package.is_empty() {
            out.push_str(package);
            out.push('.');
        }
        for (i, segment) in path_from_package.iter().enumerate() {
            if i != 0 {
                out.push('.');
            }
            out.push_str(segment);
        }
        Self(out)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for ProtoFqn {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ProtoFqn {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for ProtoFqn {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProtoFqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Debug for ProtoFqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl From<ProtoFqn> for String {
    fn from(fqn: ProtoFqn) -> Self {
        fqn.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_adds_leading_dot() {
        assert_eq!(ProtoFqn::parse("example.Task").as_str(), ".example.Task");
        assert_eq!(ProtoFqn::parse(".example.Task").as_str(), ".example.Task");
    }

    #[test]
    fn from_package_path_joins_segments() {
        assert_eq!(
            ProtoFqn::from_package_path("", &["Empty"]).as_str(),
            ".Empty"
        );
        assert_eq!(
            ProtoFqn::from_package_path("example.v1", &["Task"]).as_str(),
            ".example.v1.Task"
        );
        assert_eq!(
            ProtoFqn::from_package_path("example", &["Outer", "Inner"]).as_str(),
            ".example.Outer.Inner"
        );
    }
}
