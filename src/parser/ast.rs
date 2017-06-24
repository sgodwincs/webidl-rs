#![allow(missing_docs)]

pub type Identifier = String;

#[derive(Clone, Debug, PartialEq)]
pub enum Argument {
    NonOptional(NonOptionalArgument),
    Optional(OptionalArgument),
}

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ArgumentName {
    Identifier(Identifier),
    Keyword(ArgumentNameKeyword),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArgumentNameKeyword {
    Attribute,
    Callback,
    Const,
    Deleter,
    Dictionary,
    Enum,
    Getter,
    Implements,
    Inherit,
    Interface,
    Iterable,
    LegacyCaller,
    Maplike,
    Namespace,
    Partial,
    Required,
    Setlike,
    Setter,
    Static,
    Stringifier,
    Typedef,
    Unrestricted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Attribute {
    pub inherit: bool,
    pub name: AttributeName,
    pub read_only: bool,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum AttributeName {
    Identifier(Identifier),
    Required,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BufferRelatedType {
    ArrayBuffer,
    DataView,
    Int16Array,
    Int32Array,
    Int8Array,
    Uint16Array,
    Uint32Array,
    Uint8Array,
    Uint8ClampedArray,
    Float32Array,
    Float64Array,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Callback {
    pub arguments: Vec<Argument>,
    pub name: Identifier,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Const {
    pub name: Identifier,
    pub type_: ConstType,
    pub value: ConstValue,
}

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ConstType {
    Identifier(Nullable<Identifier>),
    PrimitiveType(Nullable<PrimitiveType>),
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
pub struct Definition {
    pub definition_type: DefinitionType,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefinitionType {
    Callback(Callback),
    Dictionary(Dictionary),
    Enum(Enum),
    Implements(Implements),
    Interface(Interface),
    Namespace(Namespace),
    Typedef(Typedef),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    pub members: Vec<DictionaryMember>,
    pub name: Identifier,
    pub type_: DictionaryType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DictionaryMember {
    NonRequired(NonRequiredDictionaryMember),
    Required(RequiredDictionaryMember),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DictionaryType {
    NonPartial(Option<Identifier>),
    Partial,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtendedAttribute {
    Nested {
        group_type: ExtendedAttributeGroupType,
        inner: Option<Box<ExtendedAttributeInner>>,
        rest: Option<Box<ExtendedAttribute>>,
    },
    Other {
        other: Other,
        rest: Option<Box<ExtendedAttribute>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtendedAttributePattern {
    ArgList(Identifier, Vec<Argument>),
    Identifier(Identifier, Identifier),
    IdentifierList(Identifier, Vec<Identifier>),
    NamedArgList(Identifier, Identifier, Vec<Argument>),
    NoArgs(Identifier),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExtendedAttributeGroupType {
    Brace,
    Bracket,
    Parenthesis,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExtendedAttributeInner {
    Nested {
        group_type: ExtendedAttributeGroupType,
        inner: Option<Box<ExtendedAttributeInner>>,
        rest: Option<Box<ExtendedAttributeInner>>,
    },
    Other {
        inner: Option<Box<ExtendedAttributeInner>>,
        other: Option<Other>,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FloatType {
    Double,
    Float,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Implements {
    pub lhs: Identifier,
    pub rhs: Identifier,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IntegerType {
    Long,
    LongLong,
    Short,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    pub members: Vec<InterfaceMember>,
    pub name: Identifier,
    pub type_: InterfaceType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceMember {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub type_: InterfaceMemberType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InterfaceMemberType {
    Attribute(Attribute),
    Const(Const),
    Iterable(Iterable),
    Maplike(Maplike),
    Operation(Operation),
    Setlike(Setlike),
    StaticMember(StaticMember),
    Stringifier(Stringifier),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InterfaceType {
    Callback(Option<Identifier>),
    NonPartial(Option<Identifier>),
    Partial,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Iterable {
    pub key_type: Option<Box<TypeWithExtendedAttributes>>,
    pub value_type: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Maplike {
    pub key_type: Box<TypeWithExtendedAttributes>,
    pub read_only: bool,
    pub value_type: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Namespace {
    pub members: Vec<NamespaceMember>,
    pub name: Identifier,
    pub partial: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamespaceMember {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub type_: NamespaceMemberType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NamespaceMemberType {
    Attribute(NamespaceMemberAttribute),
    Operation(NamespaceMemberOperation),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamespaceMemberAttribute {
    pub name: AttributeName,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamespaceMemberOperation {
    pub arguments: Vec<Argument>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NonAnyType {
    BufferRelatedType(Nullable<BufferRelatedType>),
    Error(bool),
    DOMException(bool),
    FrozenArray(Nullable<Box<TypeWithExtendedAttributes>>),
    Identifier(Nullable<Identifier>),
    Object(bool),
    PrimitiveType(Nullable<PrimitiveType>),
    PromiseType(ReturnType),
    RecordType(Nullable<RecordType>),
    Sequence(Nullable<Box<TypeWithExtendedAttributes>>),
    StringType(Nullable<StringType>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonAnyUnionMemberType {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub type_: NonAnyType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonOptionalArgument {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: ArgumentName,
    pub type_: Box<Type>,
    pub variadic: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NonRequiredDictionaryMember {
    pub default: Option<DefaultValue>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub type_: Box<Type>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Nullable<T> {
    pub nullable: bool,
    pub type_: T,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operation {
    pub arguments: Vec<Argument>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
    pub specials: Vec<Special>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OptionalArgument {
    pub default: Option<DefaultValue>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: ArgumentName,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Other {
    Any,
    Boolean,
    Byte,
    FrozenArray,
    Null,
    Object,
    Octet,
    Optional,
    Or,
    Sequence,
    Unsigned,
    Void,

    ArgumentNameKeyword(ArgumentNameKeyword),
    BooleanLiteral(bool),
    BufferRelatedType(BufferRelatedType),
    FloatLiteral(f64),
    FloatType(FloatType),
    Identifier(String),
    IntegerLiteral(i64),
    IntegerType(IntegerType),
    OtherLiteral(char),
    StringLiteral(String),
    StringType(StringType),

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PrimitiveType {
    Boolean,
    Byte,
    Octet,
    UnrestrictedFloatType(UnrestrictedFloatType),
    UnsignedIntegerType(UnsignedIntegerType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordType {
    pub key_type: StringType,
    pub value_type: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequiredDictionaryMember {
    pub default: Option<DefaultValue>,
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub name: Identifier,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    Type(Box<Type>),
    Void,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Setlike {
    pub read_only: bool,
    pub value_type: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SingleType {
    Any,
    NonAnyType(NonAnyType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SingleTypeWithExtendedAttributes {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub type_: SingleType,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Special {
    Deleter,
    Getter,
    LegacyCaller,
    Setter,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StaticMember {
    Attribute(StaticMemberAttribute),
    Operation(StaticMemberOperation),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StaticMemberAttribute {
    pub name: AttributeName,
    pub read_only: bool,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StaticMemberOperation {
    pub arguments: Vec<Argument>,
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
pub enum Stringifier {
    Attribute(StringifierAttribute),
    Default,
    Operation(StringifierOperation),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringifierAttribute {
    pub name: AttributeName,
    pub read_only: bool,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StringifierOperation {
    pub arguments: Vec<Argument>,
    pub name: Option<Identifier>,
    pub return_type: ReturnType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    SingleType(SingleType),
    UnionType(Nullable<Vec<UnionMemberType>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeWithExtendedAttributes {
    SingleType(SingleTypeWithExtendedAttributes),
    UnionType(UnionTypeWithExtendedAttributes),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Typedef {
    pub name: Identifier,
    pub type_: Box<TypeWithExtendedAttributes>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnionMemberType {
    NonAnyType(NonAnyUnionMemberType),
    UnionType(Nullable<Vec<UnionMemberType>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnionTypeWithExtendedAttributes {
    pub extended_attributes: Vec<Box<ExtendedAttribute>>,
    pub nullable: bool,
    pub type_: Vec<UnionMemberType>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnrestrictedFloatType {
    pub float_type: FloatType,
    pub restricted: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UnsignedIntegerType {
    pub integer_type: IntegerType,
    pub unsigned: bool,
}
