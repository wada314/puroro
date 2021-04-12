use crate::utils::Indentor;
use crate::Result;
use std::collections::VecDeque;
use std::{borrow::Cow, fmt::Write};

pub trait TupleOfIntoFragments<'w, W: 'w>: Sized {
    type Iter: Iterator<Item = Fragment<'w, W>>;
    fn into_frag_iter(self) -> Self::Iter;

    fn write_into(self, output: &'w mut Indentor<W>) -> Result<()>
    where
        W: std::fmt::Write,
    {
        tuple_write_impl(output, self)
    }
}

macro_rules! impl_tuple_into_fragments {
    ($len:expr) => {};
    ($len:expr, $a:ident $(, $rest:ident)*) => {
        #[allow(non_snake_case)]
        impl<'w, W: 'w, $a $(, $rest)*> TupleOfIntoFragments<'w, W> for ($a, $($rest),*)
        where
            Fragment<'w, W>: From<$a> $(+ From<$rest>)*,
        {
            type Iter = core::array::IntoIter<Fragment<'w, W>, {{$len}}>;
            fn into_frag_iter(self) -> Self::Iter {
                let ($a, $($rest),*) = self;
                Self::Iter::new([
                    <Fragment<'w, W> as From::<$a>>::from($a)
                    $(, <Fragment<'w, W> as From::<$rest>>::from($rest))*
                ])
            }
        }
        impl_tuple_into_fragments!($len-1 $(, $rest)*);
    }
}
impl_tuple_into_fragments!(8, A, B, C, D, E, F, G, H);

pub enum Fragment<'w, W: 'w> {
    Str(&'static str),
    String(String),
    Cow(Cow<'static, str>),
    Iter(Box<dyn 'w + Iterator<Item = Result<Fragment<'w, W>>>>),
    Functor(Box<dyn 'w + FnOnce(&mut Indentor<W>) -> Result<()>>),
    Indent(
        usize,
        Box<dyn 'w + Iterator<Item = Result<Fragment<'w, W>>>>,
    ),
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
pub fn indent<'w, T, W: 'w>(tuple: T) -> Fragment<'w, W>
where
    T: TupleOfIntoFragments<'w, W>,
    <T as TupleOfIntoFragments<'w, W>>::Iter: 'w,
{
    Fragment::Indent(
        1,
        Box::new(tuple.into_frag_iter().map(|v| Ok(v)))
            as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>,
    )
}
pub fn indent_n<'w, T, W: 'w>(n: usize, tuple: T) -> Fragment<'w, W>
where
    T: TupleOfIntoFragments<'w, W>,
    <T as TupleOfIntoFragments<'w, W>>::Iter: 'w,
{
    Fragment::Indent(
        n,
        Box::new(tuple.into_frag_iter().map(|v| Ok(v)))
            as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>,
    )
}
pub fn fr<'w, T, W>(from: T) -> Fragment<'w, W>
where
    Fragment<'w, W>: From<T>,
{
    from.into()
}
pub fn iter<'w, W, I, F>(iter: I) -> Fragment<'w, W>
where
    I: 'w + Iterator<Item = Result<F>>,
    F: Into<Fragment<'w, W>>,
{
    Fragment::Iter(
        Box::new(iter.map(|rv| rv.map(|v| <F as Into<Fragment<'w, W>>>::into(v))))
            as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>,
    )
}
pub fn func<'w, 'p, W, F>(f: F) -> Fragment<'w, W>
where
    F: 'w + FnOnce(&mut Indentor<W>) -> Result<()>,
{
    Fragment::Functor(Box::new(f) as Box<dyn FnOnce(&mut Indentor<W>) -> Result<()>>)
}
pub fn seq<'w, W, T>(tuple: T) -> Fragment<'w, W>
where
    T: TupleOfIntoFragments<'w, W>,
    <T as TupleOfIntoFragments<'w, W>>::Iter: 'w,
    W: 'w,
{
    Fragment::Iter(Box::new(tuple.into_frag_iter().map(|v| Ok(v)))
        as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>)
}

fn tuple_write_impl<'w, T, W>(w: &'w mut Indentor<W>, tuple: T) -> Result<()>
where
    W: Write,
    T: TupleOfIntoFragments<'w, W>,
{
    enum Task<'w, W: 'w + std::fmt::Write> {
        WriteFragment(Fragment<'w, W>),
        ProgressIterator(Box<dyn 'w + Iterator<Item = Result<Fragment<'w, W>>>>),
        CallFunctor(Box<dyn 'w + FnOnce(&mut Indentor<W>) -> Result<()>>),
        Indent(),
        Unindent(),
    }
    let mut tasks = tuple
        .into_frag_iter()
        .map(|frag| Task::WriteFragment(frag))
        .collect::<VecDeque<_>>();
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
                Fragment::Indent(n, iter) => {
                    for _ in 0..n {
                        tasks.push_front(Task::Unindent());
                    }
                    tasks.push_front(Task::ProgressIterator(iter));
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
