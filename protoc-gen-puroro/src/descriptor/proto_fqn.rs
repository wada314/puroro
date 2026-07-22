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

    /// FQN of a protobuf `package` namespace (not a type).
    ///
    /// Empty package uses the synthetic root `"."`, so [`Self::append`] can build
    /// top-level names like `.Empty` uniformly with nested names.
    pub fn from_package(package: &str) -> Self {
        if package.is_empty() {
            Self(".".into())
        } else {
            Self(format!(".{package}"))
        }
    }

    /// Build an FQN from a protobuf `package` and nested simple-name path.
    ///
    /// `package` may be empty. `path_from_package` is e.g. `["Outer", "Inner"]`.
    pub fn from_package_path(package: &str, path_from_package: &[&str]) -> Self {
        let mut fqn = Self::from_package(package);
        for segment in path_from_package {
            fqn = fqn.append(segment);
        }
        fqn
    }

    /// Append a simple type name under this package or enclosing-type FQN.
    ///
    /// - parent `.example.v1` + `Task` → `.example.v1.Task`
    /// - parent `.` (empty package) + `Empty` → `.Empty`
    pub fn append(&self, simple_name: &str) -> Self {
        if self.0 == "." {
            Self(format!(".{simple_name}"))
        } else {
            Self(format!("{}.{}", self.0, simple_name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Whether this is the synthetic empty-package root (`"."`), not a real type.
    pub fn is_package_root(&self) -> bool {
        self.0 == "."
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
    fn from_package_and_append() {
        assert_eq!(ProtoFqn::from_package("").as_str(), ".");
        assert!(ProtoFqn::from_package("").is_package_root());
        assert_eq!(ProtoFqn::from_package("example.v1").as_str(), ".example.v1");
        assert_eq!(
            ProtoFqn::from_package("").append("Empty").as_str(),
            ".Empty"
        );
        assert_eq!(
            ProtoFqn::from_package("example.v1")
                .append("Outer")
                .append("Inner")
                .as_str(),
            ".example.v1.Outer.Inner"
        );
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
