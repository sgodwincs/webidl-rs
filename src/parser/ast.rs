#![allow(missing_docs)]

pub type Identifier = String;

// The following structures are used to help simplify building the AST in the grammar. Ideally
// these would not be exposed outside the crate, but the compiler seems to think they are exposed
// as private types when `pub(super)` is used. This is not the case since all of their variants are
// matched into other structures, but I suppose it is not a big deal.

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BufferRelatedType {
    ArrayBuffer,
    DataView,
    Float32Array,
    Float64Array,
    Int16Array,
    Int32Array,
    Int8Array,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Octet,
    UnrestrictedFloat(UnrestrictedFloatType),
    UnsignedInteger(UnsignedIntegerType),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnrestrictedFloatType {
    RestrictedDouble,
    RestrictedFloat,
    UnrestrictedDouble,
    UnrestrictedFloat,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnsignedIntegerType {
    SignedLong,
    SignedLongLong,
    SignedShort,
    UnsignedLong,
    UnsignedLongLong,
    UnsignedShort,
}

// Publically available AST structures

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub default: Option<DefaultValue>,
    pub name: Identifier,
    pub optional: bool,
    pub type_: Box<Type>,
    pub variadic: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArgumentListExtendedAttribute {
    pub arguments: Vec<Argument>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Regular(RegularAttribute),
    Static(StaticAttribute),
    Stringifier(StringifierAttribute),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Callback {
    pub arguments: Vec<Argument>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallbackInterface {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub inherits: Option<Identifier>,
    pub members: Vec<InterfaceMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Const {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub nullable: bool,
    pub type_: ConstType,
    pub value: ConstValue,
}

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ConstType {
    Boolean,
    Byte,
    Identifier(Identifier),
    Octet,
    RestrictedDouble,
    RestrictedFloat,
    SignedLong,
    SignedLongLong,
    SignedShort,
    UnrestrictedDouble,
    UnrestrictedFloat,
    UnsignedLong,
    UnsignedLongLong,
    UnsignedShort,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstValue {
    BooleanLiteral(bool),
    FloatLiteral(f64),
    IntegerLiteral(i64),
    Null,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultValue {
    ConstValue(ConstValue),
    EmptySequence,
    StringLiteral(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Definition {
    Callback(Callback),
    Dictionary(Dictionary),
    Enum(Enum),
    Implements(Implements),
    Interface(Interface),
    Namespace(Namespace),
    Typedef(Typedef),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Dictionary {
    NonPartial(NonPartialDictionary),
    Partial(PartialDictionary),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DictionaryMember {
    pub default: Option<DefaultValue>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub required: bool,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExplicitStringifierOperation {
    pub arguments: Vec<Argument>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtendedAttribute {
    ArgumentList(ArgumentListExtendedAttribute),
    Identifier(IdentifierExtendedAttribute),
    IdentifierList(IdentifierListExtendedAttribute),
    NamedArgumentList(NamedArgumentListExtendedAttribute),
    NoArguments(Other),
    //Other(OtherExtendedAttribute),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifierExtendedAttribute {
    pub lhs: Identifier,
    pub rhs: Other,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdentifierListExtendedAttribute {
    pub lhs: Identifier,
    pub rhs: Vec<Identifier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Implements {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub implementor: Identifier,
    pub implementee: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplicitStringifierOperation {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Interface {
    Callback(CallbackInterface),
    NonPartial(NonPartialInterface),
    Partial(PartialInterface),
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceMember {
    Attribute(Attribute),
    Const(Const),
    Iterable(Iterable),
    Maplike(Maplike),
    Operation(Operation),
    Setlike(Setlike),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Iterable {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub key_type: Option<Box<Type>>,
    pub value_type: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Maplike {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub key_type: Box<Type>,
    pub read_only: bool,
    pub value_type: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedArgumentListExtendedAttribute {
    pub lhs_name: Identifier,
    pub rhs_arguments: Vec<Argument>,
    pub rhs_name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Namespace {
    NonPartial(NonPartialNamespace),
    Partial(PartialNamespace),
}

#[derive(Clone, Debug, PartialEq)]
pub enum NamespaceMember {
    Attribute(Attribute),
    Operation(Operation),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonPartialDictionary {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub inherits: Option<Identifier>,
    pub members: Vec<DictionaryMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonPartialInterface {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub inherits: Option<Identifier>,
    pub members: Vec<InterfaceMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonPartialNamespace {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub members: Vec<NamespaceMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Regular(RegularOperation),
    Special(SpecialOperation),
    Static(StaticOperation),
    Stringifier(StringifierOperation),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Other {
    Any,
    ArrayBuffer,
    Attribute,
    Boolean,
    Byte,
    ByteString,
    Callback,
    Const,
    DOMString,
    DataView,
    Deleter,
    Dictionary,
    Double,
    Enum,
    False,
    Float,
    Float32Array,
    Float64Array,
    FrozenArray,
    Getter,
    Implements,
    Inherit,
    Int16Array,
    Int32Array,
    Int8Array,
    Interface,
    Iterable,
    LegacyCaller,
    Long,
    Maplike,
    Namespace,
    NegativeInfinity,
    NaN,
    Null,
    Object,
    Octet,
    Optional,
    Or,
    Partial,
    PositiveInfinity,
    Required,
    Sequence,
    Setlike,
    Setter,
    Short,
    Static,
    Stringifier,
    True,
    Typedef,
    USVString,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
    Unrestricted,
    Unsigned,
    Void,

    FloatLiteral(f64),
    Identifier(Identifier),
    IntegerLiteral(i64),
    OtherLiteral(char),
    StringLiteral(String),

    Colon,
    Ellipsis,
    Equals,
    GreaterThan,
    Hyphen,
    LessThan,
    Period,
    QuestionMark,
    Semicolon,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OtherExtendedAttribute {
    Nested {
        group_type: OtherExtendedAttributeGroupType,
        inner: Option<Box<ExtendedAttribute>>,
        rest: Option<Box<ExtendedAttribute>>,
    },
    Other {
        other: Option<Other>,
        rest: Option<Box<ExtendedAttribute>>,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OtherExtendedAttributeGroupType {
    Brace,
    Bracket,
    Parenthesis,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialDictionary {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub members: Vec<DictionaryMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialInterface {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub members: Vec<InterfaceMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartialNamespace {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub members: Vec<NamespaceMember>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegularOperation {
    pub arguments: Vec<Argument>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegularAttribute {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub inherits: bool,
    pub name: Identifier,
    pub read_only: bool,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    NonVoid(Box<Type>),
    Void,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Setlike {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub read_only: bool,
    pub type_: Box<Type>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Special {
    Deleter,
    Getter,
    LegacyCaller,
    Setter,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpecialOperation {
    pub arguments: Vec<Argument>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
    pub special_keywords: Vec<Special>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StaticAttribute {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub read_only: bool,
    pub type_: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StaticOperation {
    pub arguments: Vec<Argument>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StringType {
    ByteString,
    DOMString,
    USVString,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringifierAttribute {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub read_only: bool,
    pub type_: Box<Type>,
}

#[allow(variant_size_differences)]
#[derive(Clone, Debug, PartialEq)]
pub enum StringifierOperation {
    Explicit(ExplicitStringifierOperation),
    Implicit(ImplicitStringifierOperation),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub kind: TypeKind,
    pub nullable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    Any,
    ArrayBuffer,
    Boolean,
    Byte,
    ByteString,
    DOMException,
    DOMString,
    DataView,
    Error,
    Float32Array,
    Float64Array,
    FrozenArray(Box<Type>),
    Identifier(Identifier),
    Int16Array,
    Int32Array,
    Int8Array,
    Octet,
    Object,
    Promise(ReturnType),
    Record(StringType, Box<Type>),
    RestrictedDouble,
    RestrictedFloat,
    Sequence(Box<Type>),
    SignedLong,
    SignedLongLong,
    SignedShort,
    USVString,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
    Union(Vec<Box<Type>>),
    UnrestrictedDouble,
    UnrestrictedFloat,
    UnsignedLong,
    UnsignedLongLong,
    UnsignedShort,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Typedef {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub type_: Box<Type>,
}
