use crate::{ErrorKind, Result};
use std::borrow::Cow;
use std::fmt::Display;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Ident<'a>(pub Cow<'a, str>);
impl<'a> Ident<'a> {
    pub fn new(s: Cow<'a, str>) -> Self {
        Self(s)
    }
}
impl Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}
impl<'a, T> From<T> for Ident<'a>
where
    Cow<'a, str>: From<T>,
{
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

#[derive(Debug, Clone)]
pub struct GenericParams<'a>(Vec<Cow<'a, str>>);
impl<'a> GenericParams<'a> {
    pub fn new_empty() -> Self {
        Self(vec![])
    }

    pub fn bind_checked(&self, from: &str, to: Cow<'a, str>) -> Result<Self> {
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

    pub fn bind_unchecked(&self, from: &str, to: Cow<'a, str>) -> Cow<'_, Self> {
        match self.bind_checked(from, to) {
            Ok(gp) => Cow::Owned(gp),
            Err(_) => Cow::Borrowed(self),
        }
    }
}
impl Display for GenericParams<'_> {
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
impl<'a, T> FromIterator<T> for GenericParams<'a>
where
    Cow<'a, str>: From<T>,
{
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        Self(iter.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone)]
pub struct PathItem<'a> {
    ident: Ident<'a>,
    gp: GenericParams<'a>,
}
impl<'a> PathItem<'a> {
    pub fn new(ident: Ident<'a>, gp: GenericParams<'a>) -> Self {
        Self { ident, gp }
    }
    pub fn bind_checked(&self, from: &str, to: Cow<'a, str>) -> Result<Self> {
        Ok(Self {
            ident: self.ident.clone(),
            gp: self.gp.bind_checked(from, to)?,
        })
    }
    pub fn bind_unchecked(&self, from: &str, to: Cow<'a, str>) -> Cow<'_, Self> {
        match self.bind_checked(from, to) {
            Ok(item) => Cow::Owned(item),
            Err(_) => Cow::Borrowed(self),
        }
    }
}
impl<'a> From<Ident<'a>> for PathItem<'a> {
    fn from(ident: Ident<'a>) -> Self {
        Self {
            ident,
            gp: GenericParams::new_empty(),
        }
    }
}
impl Display for PathItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.ident))?;
        if !self.gp.0.is_empty() {
            f.write_fmt(format_args!("::{}", self.gp))?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PathExpr<'a>(Vec<PathItem<'a>>);
impl<'a> PathExpr<'a> {
    pub fn bind_checked(&self, from: &str, to: Cow<'a, str>) -> Result<Self> {
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
    pub fn bind_unchecked(&self, from: &str, to: Cow<'a, str>) -> Cow<'_, Self> {
        match self.bind_checked(from, to) {
            Ok(path) => Cow::Owned(path),
            Err(_) => Cow::Borrowed(self),
        }
    }
}
impl Display for PathExpr<'_> {
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
impl<'a> FromIterator<PathItem<'a>> for PathExpr<'a> {
    fn from_iter<T: IntoIterator<Item = PathItem<'a>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}
