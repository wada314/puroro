#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

pub trait CodeGeneratorResponseTrait: ::std::clone::Clone {
    fn error<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn supported_features<'this>(&'this self) -> ::std::option::Option::<u64>;
    type FileElement<'this>: self::code_generator_response::FileTrait where Self: 'this;
    type FileRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::FileElement::<'this>>>
        where Self: 'this;
    fn file<'this>(&'this self) -> Self::FileRepeated::<'this>;
}
pub mod code_generator_response {
pub trait FileTrait: ::std::clone::Clone {
    fn name<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn insertion_point<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    fn content<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type GeneratedCodeInfoType<'this>: super::super::GeneratedCodeInfoTrait where Self: 'this;
    fn generated_code_info<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::GeneratedCodeInfoType::<'this>>>;
}
} // mod code_generator_response
pub trait CodeGeneratorRequestTrait: ::std::clone::Clone {
    type FileToGenerateRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, str>>
        where Self: 'this;
    fn file_to_generate<'this>(&'this self) -> Self::FileToGenerateRepeated::<'this>;
    fn parameter<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
    type ProtoFileElement<'this>: super::FileDescriptorProtoTrait where Self: 'this;
    type ProtoFileRepeated<'this>: ::puroro::RepeatedField::<::std::borrow::Cow::<'this, Self::ProtoFileElement::<'this>>>
        where Self: 'this;
    fn proto_file<'this>(&'this self) -> Self::ProtoFileRepeated::<'this>;
    type CompilerVersionType<'this>: self::VersionTrait where Self: 'this;
    fn compiler_version<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, Self::CompilerVersionType::<'this>>>;
}
pub trait VersionTrait: ::std::clone::Clone {
    fn major<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn minor<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn patch<'this>(&'this self) -> ::std::option::Option::<i32>;
    fn suffix<'this>(&'this self) -> ::std::option::Option::<::std::borrow::Cow::<'this, str>>;
}
