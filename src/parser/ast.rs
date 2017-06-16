#![allow(missing_docs)]

#[derive(Clone, Debug, PartialEq)]
pub struct Const {
    pub const_type: ConstType,
    pub name: String,
    pub nullable: bool,
    pub value: ConstValue,
}

#[allow(variant_size_differences)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ConstType {
    Boolean,
    Byte,
    ExtendedFloatType(ExtendedFloatType),
    ExtendedIntegerType(ExtendedIntegerType),
    Identifier(String),
    Octet,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstValue {
    BooleanLiteral(bool),
    FloatLiteral(f64),
    IntegerLiteral(i64),
    Null,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExtendedFloatType {
    pub float_type: FloatType,
    pub restricted: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExtendedIntegerType {
    pub integer_type: IntegerType,
    pub unsigned: bool,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FloatType {
    Double,
    Float,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IntegerType {
    Long,
    LongLong,
    Short,
}
