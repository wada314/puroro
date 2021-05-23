use crate::{ErrorKind, Result};
use std::borrow::Cow;
use std::fmt::Display;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Ident(Cow<'static, str>);
impl Ident {
    pub fn new(s: Cow<'static, str>) -> Self {
        Self(s)
    }
}
impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct GenericParams(Vec<Cow<'static, str>>);
impl GenericParams {
    pub fn new_empty() -> Self {
        Self(vec![])
    }

    pub fn bind_checked(&self, from: &str, to: Cow<'static, str>) -> Result<GenericParams> {
        let mut newvec = Cow::Borrowed(&self.0);
        for i in 0..newvec.len() {
            if newvec[i] == from {
                newvec.to_mut()[i] = to;
                break;
            }
        }
        match newvec {
            Cow::Borrowed(_) => Err(ErrorKind::InternalError {
                detail: "failed to bind generic params".to_string(),
            })?,
            Cow::Owned(vec) => Ok(GenericParams(vec)),
        }
    }

    pub fn bind_unchecked(&self, from: &str, to: Cow<'static, str>) -> Cow<'_, Self> {
        match self.bind_checked(from, to) {
            Ok(gp) => Cow::Owned(gp),
            Err(_) => Cow::Borrowed(self),
        }
    }
}
impl Display for GenericParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ::std::fmt::Write;
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            f.write_char('<')?;
            f.write_str(first)?;
            for param in iter {
                f.write_str(", ")?;
                f.write_str(param)?;
            }
            f.write_char('>')?;
        }
        Ok(())
    }
}
impl<T> FromIterator<T> for GenericParams
where
    Cow<'static, str>: From<T>,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self(iter.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone)]
struct PathItem {
    ident: Ident,
    gp: GenericParams,
}
impl PathItem {
    pub fn new(ident: Ident, gp: GenericParams) -> Self {
        Self { ident, gp }
    }
    pub fn bind_checked(&self, from: &str, to: Cow<'static, str>) -> Result<Self> {
        Ok(Self {
            ident: self.ident.clone(),
            gp: self.gp.bind_checked(from, to)?,
        })
    }
    pub fn bind_unchecked(&self, from: &str, to: Cow<'static, str>) -> Cow<'_, Self> {
        match self.bind_checked(from, to) {
            Ok(item) => Cow::Owned(item),
            Err(_) => Cow::Borrowed(self),
        }
    }
}
impl From<Ident> for PathItem {
    fn from(ident: Ident) -> Self {
        Self {
            ident,
            gp: GenericParams::new_empty(),
        }
    }
}
impl Display for PathItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.ident))?;
        if !self.gp.0.is_empty() {
            f.write_fmt(format_args!("::{}", self.gp))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct PathExpr(Vec<PathItem>);
impl PathExpr {
    pub fn bind_checked(&self, from: &str, to: Cow<'static, str>) -> Result<Self> {
        let mut newvec = Cow::Borrowed(&self.0);
        for i in 0..self.0.len() {
            if let Ok(new_item) = self.0[i].bind_checked(from, to.clone()) {
                newvec.to_mut()[i] = new_item;
            }
        }
        match newvec {
            Cow::Borrowed(_) => Err(ErrorKind::InternalError {
                detail: "failed to bind generic params".to_string(),
            })?,
            Cow::Owned(vec) => Ok(Self(vec)),
        }
    }
    pub fn bind_unchecked(&self, from: &str, to: Cow<'static, str>) -> Cow<'_, Self> {
        match self.bind_checked(from, to) {
            Ok(path) => Cow::Owned(path),
            Err(_) => Cow::Borrowed(self),
        }
    }
}
impl Display for PathExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            f.write_fmt(format_args!("{}", first))?;
            for item in iter {
                f.write_fmt(format_args!("::{}", item))?;
            }
        }
        Ok(())
    }
}
impl FromIterator<PathItem> for PathExpr {
    fn from_iter<T: IntoIterator<Item = PathItem>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}
