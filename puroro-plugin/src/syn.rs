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
        let mut newvec = self.0.clone();
        let mut found = false;
        for i in 0..newvec.len() {
            if newvec[i] == from {
                newvec[i] = to;
                found = true;
                break;
            }
        }
        if found {
            Ok(GenericParams(newvec))
        } else {
            Err(ErrorKind::InternalError {
                detail: "failed to bind generic params".to_string(),
            })?
        }
    }

    pub fn bind_unchecked(&self, from: &str, to: Cow<'static, str>) -> Cow<'_, GenericParams> {
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
            f.write_str("::")?;
            f.write_fmt(format_args!("{}", self.gp))?;
        }
        Ok(())
    }
}
