use crate::generators::utils::*;
use crate::plugin::*;
use crate::{ErrorKind, Result};
use itertools::Itertools;
use std::collections::VecDeque;
use std::{borrow::Cow, collections::HashMap, fmt::Write};

pub(crate) trait TupleOfIntoFragments<'w, W: 'w> {
    type Iter: Iterator<Item = Fragment<'w, W>>;
    fn into_frag_iter(self) -> Self::Iter;
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

pub(crate) enum Fragment<'w, W: 'w> {
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
pub(crate) fn indent<'w, T, W: 'w>(tuple: T) -> Fragment<'w, W>
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
pub(crate) fn indent_n<'w, T, W: 'w>(n: usize, tuple: T) -> Fragment<'w, W>
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
pub(crate) fn fr<'w, T, W>(from: T) -> Fragment<'w, W>
where
    Fragment<'w, W>: From<T>,
{
    from.into()
}
pub(crate) fn iter<'w, W, I, F>(iter: I) -> Fragment<'w, W>
where
    I: 'w + Iterator<Item = Result<F>>,
    F: Into<Fragment<'w, W>>,
{
    Fragment::Iter(
        Box::new(iter.map(|rv| rv.map(|v| <F as Into<Fragment<'w, W>>>::into(v))))
            as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>,
    )
}
pub(crate) fn func<'w, W, F>(f: F) -> Fragment<'w, W>
where
    F: 'w + FnOnce(&mut Indentor<W>) -> Result<()>,
{
    Fragment::Functor(Box::new(f) as Box<dyn FnOnce(&mut Indentor<W>) -> Result<()>>)
}
pub(crate) fn seq<'w, W, T>(tuple: T) -> Fragment<'w, W>
where
    T: TupleOfIntoFragments<'w, W>,
    <T as TupleOfIntoFragments<'w, W>>::Iter: 'w,
    W: 'w,
{
    Fragment::Iter(Box::new(tuple.into_frag_iter().map(|v| Ok(v)))
        as Box<dyn Iterator<Item = Result<Fragment<'w, W>>>>)
}
pub(crate) fn write<'w, T, W>(w: &'w mut Indentor<W>, tuple: T) -> Result<()>
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

#[derive(Debug, Clone)]
pub(crate) enum TypeOfIdent {
    Message,
    Enum,
}

pub(crate) struct InvocationContext<'p> {
    cgreq: &'p CodeGeneratorRequest,
    type_of_ident_map: HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
}

impl<'p> InvocationContext<'p> {
    pub(crate) fn new(cgreq: &'p CodeGeneratorRequest) -> Result<Self> {
        Ok(Self {
            cgreq,
            type_of_ident_map: Self::generate_type_of_ident_map(cgreq)?,
        })
    }
    pub(crate) fn cgreq(&self) -> &CodeGeneratorRequest {
        self.cgreq
    }
    pub(crate) fn type_of_ident(
        &self,
        mut package: Vec<&'p str>,
        typename: &'p str,
    ) -> Option<TypeOfIdent> {
        let mfqtn = MaybeFullyQualifiedTypeName::from_maybe_fq_typename(typename);
        if let Some(fqtn) = mfqtn.try_to_absolute() {
            return self.type_of_ident_map.get(&fqtn).cloned();
        } else {
            loop {
                let fqtn = mfqtn.with_package(package.clone());
                if let Some(found) = self.type_of_ident_map.get(&fqtn) {
                    return Some(found.clone());
                }
                if package.pop().is_none() {
                    break;
                }
            }
            None
        }
    }
    fn generate_type_of_ident_map(
        cgreq: &'p CodeGeneratorRequest,
    ) -> Result<HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>> {
        struct Visitor<'a, 'p> {
            map: &'a mut HashMap<FullyQualifiedTypeName<'p>, TypeOfIdent>,
            package: Vec<&'p str>,
        }
        impl<'a, 'p> DescriptorVisitor<'p> for Visitor<'a, 'p> {
            fn handle_msg(&mut self, msg: &'p DescriptorProto) -> Result<()> {
                let fqtn = FullyQualifiedTypeName::new(self.package.clone(), &msg.name);
                if let Some(_) = self.map.insert(fqtn.clone(), TypeOfIdent::Message) {
                    Err(ErrorKind::ConflictedName {
                        name: fqtn.to_string(),
                    })?
                }
                Ok(())
            }

            fn handle_enum(&mut self, enume: &'p EnumDescriptorProto) -> Result<()> {
                let fqtn = FullyQualifiedTypeName::new(self.package.clone(), &enume.name);
                if let Some(_) = self.map.insert(fqtn.clone(), TypeOfIdent::Enum) {
                    Err(ErrorKind::ConflictedName {
                        name: fqtn.to_string(),
                    })?
                }
                Ok(())
            }

            fn enter_submodule(&mut self, name: &'p str) -> Result<()> {
                self.package.push(name);
                Ok(())
            }

            fn exit_submodule(&mut self, _name: &'p str) -> Result<()> {
                self.package.pop().unwrap();
                Ok(())
            }
        }

        let mut map = HashMap::new();
        for file in &cgreq.proto_file {
            let package = file.package.split('.').collect();
            let mut visitor = Visitor {
                map: &mut map,
                package,
            };
            visit_in_file(file, &mut visitor)?;
        }
        Ok(map)
    }
}

pub(crate) struct FileGeneratorContext<'p> {
    package: Vec<&'p str>,
    path_to_package_root: String,
}
impl<'p> FileGeneratorContext<'p> {
    pub(crate) fn new(package: Vec<&'p str>) -> Self {
        let path_to_package_root = Self::generate_path_to_package_root(&package);
        Self {
            package,
            path_to_package_root,
        }
    }
    pub(crate) fn package(&self) -> &Vec<&'p str> {
        &self.package
    }
    pub(crate) fn path_to_package_root(&self) -> &str {
        &self.path_to_package_root
    }

    fn generate_path_to_package_root(package: &Vec<&str>) -> String {
        if package.is_empty() {
            "self".into()
        } else {
            let supers = std::iter::repeat("super").take(package.len());
            Itertools::intersperse(supers, "::").collect::<String>()
        }
    }

    pub(crate) fn enter_submessage_namespace<
        F: FnOnce(&mut FileGeneratorContext<'p>) -> Result<()>,
    >(
        &mut self,
        message_name: &'p str,
        f: F,
    ) -> Result<()> {
        self.package.push(message_name);
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
        let ret = (f)(self);
        self.package.pop();
        self.path_to_package_root = Self::generate_path_to_package_root(&self.package);
        ret
    }
}

pub(crate) trait DescriptorVisitor<'p> {
    fn handle_msg(&mut self, msg: &'p DescriptorProto) -> Result<()>;
    fn handle_enum(&mut self, enume: &'p EnumDescriptorProto) -> Result<()>;
    fn enter_submodule(&mut self, name: &'p str) -> Result<()>;
    fn exit_submodule(&mut self, name: &'p str) -> Result<()>;
}

pub(crate) fn visit_in_file<'p, T: DescriptorVisitor<'p>>(
    file: &'p FileDescriptorProto,
    visitor: &mut T,
) -> Result<()> {
    enum Task<'q> {
        HandleMsg(&'q DescriptorProto),
        HandleEnum(&'q EnumDescriptorProto),
        EnterSubmodule(&'q str),
        ExitSubmodule(&'q str),
    }
    let mut tasks = file
        .message_type
        .iter()
        .map(|msg| Task::HandleMsg(msg))
        .chain(file.enum_type.iter().map(|enume| Task::HandleEnum(enume)))
        .collect::<Vec<_>>();

    while let Some(task) = tasks.pop() {
        match task {
            Task::HandleMsg(msg) => {
                visitor.handle_msg(msg)?;
                if !msg.nested_type.is_empty() || !msg.enum_type.is_empty() {
                    tasks.push(Task::ExitSubmodule(&msg.name));
                    tasks.extend(msg.nested_type.iter().map(|submsg| Task::HandleMsg(submsg)));
                    tasks.extend(msg.enum_type.iter().map(|enume| Task::HandleEnum(enume)));
                    tasks.push(Task::EnterSubmodule(&msg.name));
                }
            }
            Task::HandleEnum(enume) => {
                visitor.handle_enum(enume)?;
            }
            Task::EnterSubmodule(name) => {
                visitor.enter_submodule(name)?;
            }
            Task::ExitSubmodule(name) => {
                visitor.exit_submodule(name)?;
            }
        }
    }

    Ok(())
}

pub(crate) trait FileGeneratorHandler {
    fn handle_msg<'p, W: std::fmt::Write>(
        &mut self,
        out: &mut Indentor<W>,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p>,
        msg: &'p DescriptorProto,
    ) -> Result<()>;
    fn handle_enum<'p, W: std::fmt::Write>(
        &mut self,
        out: &mut Indentor<W>,
        context: &InvocationContext,
        fc: &mut FileGeneratorContext<'p>,
        enume: &'p EnumDescriptorProto,
    ) -> Result<()>;
    fn generate_filename<'p>(
        &mut self,
        context: &InvocationContext,
        file: &'p FileDescriptorProto,
    ) -> Result<String>;
}

pub(crate) fn generate_file_with_handler<'p, H>(
    context: &InvocationContext,
    input_file: &'p FileDescriptorProto,
    handler: &mut H,
) -> Result<(String, String)>
where
    H: FileGeneratorHandler,
{
    let package = input_file.package.split('.').collect::<Vec<_>>();
    let mut fc = FileGeneratorContext::new(package);
    let filename = handler.generate_filename(context, input_file)?;

    struct InnerVisitor<'a, 'q, H: FileGeneratorHandler> {
        output: Indentor<String>,
        context: &'a InvocationContext<'q>,
        fc: &'a mut FileGeneratorContext<'q>,
        handler: &'a mut H,
    }
    impl<'a, 'q, H> DescriptorVisitor<'q> for InnerVisitor<'a, 'q, H>
    where
        H: FileGeneratorHandler,
    {
        fn handle_msg(&mut self, msg: &'q DescriptorProto) -> Result<()> {
            self.handler
                .handle_msg(&mut self.output, self.context, self.fc, msg)
        }
        fn handle_enum(&mut self, enume: &'q EnumDescriptorProto) -> Result<()> {
            self.handler
                .handle_enum(&mut self.output, self.context, self.fc, enume)
        }
        fn enter_submodule(&mut self, name: &'q str) -> Result<()> {
            let module_name = to_module_name(name);
            writeln!(&mut self.output, "mod {name} {{", name = module_name)?;
            self.output.indent();
            Ok(())
        }
        fn exit_submodule(&mut self, _name: &'q str) -> Result<()> {
            self.output.unindent();
            writeln!(self.output, "}}")?;
            Ok(())
        }
    }

    let mut inner_visitor = InnerVisitor {
        output: Indentor::new(String::new()),
        context,
        fc: &mut fc,
        handler,
    };
    visit_in_file(&input_file, &mut inner_visitor)?;

    Ok((filename, inner_visitor.output.into_inner()))
}
