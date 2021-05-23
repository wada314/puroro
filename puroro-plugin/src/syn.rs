use crate::{ErrorKind, Result};
use std::borrow::Cow;
use std::fmt::Display;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct GenericParams(Vec<Cow<'static, str>>);
impl GenericParams {
    pub fn bind(&self, from: &str, to: Cow<'static, str>) -> Result<GenericParams> {
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
