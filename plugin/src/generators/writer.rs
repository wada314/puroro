use crate::utils::Indentor;
use crate::Result;
use std::collections::VecDeque;
use std::{borrow::Cow, fmt::Write};

// TupleOfIntoFragments

pub trait IntoFragment<'w, W: 'w>: Into<Fragment<'w, W>> {
    fn write_into(self, output: &'w mut Indentor<W>) -> Result<()>
    where
        W: std::fmt::Write,
    {
        frag_write_impl(output, self.into())
    }
}
impl<'w, W: 'w, T> IntoFragment<'w, W> for T where T: Into<Fragment<'w, W>> {}

macro_rules! impl_tuple_into_fragments {
    ($len:expr) => {};
    ($len:expr, $a:ident $(, $rest:ident)*) => {
        #[allow(non_snake_case)]
        impl<'w, W: 'w, $a $(, $rest)*> From<($a, $($rest),*)> for Fragment<'w, W>
        where
            Fragment<'w, W>: From<$a> $(+ From<$rest>)*,
        {
            fn from(frags: ($a, $($rest),*)) -> Self {
                let ($a, $($rest),*) = frags;
                Fragment::Iter(Box::new(core::array::IntoIter::new([
                    Ok(<Fragment<'w, W> as From::<$a>>::from($a))
                    $(, Ok(<Fragment<'w, W> as From::<$rest>>::from($rest)))*
                ])))
            }
        }
        impl_tuple_into_fragments!($len-1 $(, $rest)*);
    }
}
impl_tuple_into_fragments!(12, A, B, C, D, E, F, G, H, I, J, K, L);

// Fragment

pub enum Fragment<'w, W: 'w> {
    Str(&'static str),
    String(String),
    Cow(Cow<'static, str>),
    Iter(Box<dyn 'w + Iterator<Item = Result<Fragment<'w, W>>>>),
    Functor(Box<dyn 'w + FnOnce(&mut Indentor<W>) -> Result<()>>),
    Indent(usize, Box<Fragment<'w, W>>),
}
impl<'w, W> From<&'static str> for Fragment<'w, W> {
    fn from(s: &'static str) -> Self {
        Self::Str(s)
    }
}
impl<'w, W> From<String> for Fragment<'w, W> {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}
impl<'w, W> From<Cow<'static, str>> for Fragment<'w, W> {
    fn from(s: Cow<'static, str>) -> Self {
        Self::Cow(s)
    }
}

pub fn indent<'w, T, W: 'w>(frags: T) -> Fragment<'w, W>
where
    Fragment<'w, W>: From<T>,
{
    Fragment::Indent(1, Box::new(frags.into()))
}
pub fn indent_n<'w, T, W: 'w>(n: usize, frags: T) -> Fragment<'w, W>
where
    Fragment<'w, W>: From<T>,
{
    Fragment::Indent(n, Box::new(frags.into()))
}
pub fn iter<'w, W, I, F>(iter: I) -> Fragment<'w, W>
where
    I: 'w + Iterator<Item = Result<F>>,
    Fragment<'w, W>: From<F>,
{
    Fragment::Iter(Box::new(iter.map(|rv| rv.map(|v| v.into())))
        as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>)
}
pub fn func<'w, 'p, W, F>(f: F) -> Fragment<'w, W>
where
    F: 'w + FnOnce(&mut Indentor<W>) -> Result<()>,
{
    Fragment::Functor(Box::new(f) as Box<dyn FnOnce(&mut Indentor<W>) -> Result<()>>)
}

fn frag_write_impl<'w, W>(w: &'w mut Indentor<W>, frag: Fragment<'w, W>) -> Result<()>
where
    W: Write,
{
    enum Task<'w, W: 'w + std::fmt::Write> {
        WriteFragment(Fragment<'w, W>),
        ProgressIterator(Box<dyn 'w + Iterator<Item = Result<Fragment<'w, W>>>>),
        CallFunctor(Box<dyn 'w + FnOnce(&mut Indentor<W>) -> Result<()>>),
        Indent(),
        Unindent(),
    }
    let mut tasks = VecDeque::new();
    tasks.push_back(Task::WriteFragment(frag.into()));
    while let Some(task) = tasks.pop_front() {
        match task {
            Task::WriteFragment(fragment) => match fragment {
                Fragment::Str(s) => {
                    w.write_str(&s.replace("}}", "}").replace("{{", "{"))?;
                }
                Fragment::String(s) => {
                    w.write_str(&s.replace("}}", "}").replace("{{", "{"))?;
                }
                Fragment::Cow(s) => {
                    w.write_str(&s.replace("}}", "}").replace("{{", "{"))?;
                }
                Fragment::Iter(iter) => {
                    tasks.push_front(Task::ProgressIterator(iter));
                }
                Fragment::Functor(f) => {
                    tasks.push_front(Task::CallFunctor(f));
                }
                Fragment::Indent(n, frag) => {
                    for _ in 0..n {
                        tasks.push_front(Task::Unindent());
                    }
                    tasks.push_front(Task::WriteFragment(*frag));
                    for _ in 0..n {
                        tasks.push_front(Task::Indent());
                    }
                }
            },
            Task::ProgressIterator(mut iter) => {
                if let Some(fr) = iter.next() {
                    tasks.push_front(Task::ProgressIterator(iter));
                    tasks.push_front(Task::WriteFragment(fr?));
                }
            }
            Task::CallFunctor(f) => {
                (f)(w)?;
            }
            Task::Indent() => {
                w.indent();
            }
            Task::Unindent() => {
                w.unindent();
            }
        }
    }
    Ok(())
}
