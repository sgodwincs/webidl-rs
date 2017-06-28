#![allow(missing_docs)]

use super::ast::*;

pub trait Visitor<'ast>: Sized {
    fn visit(&mut self, definitions: &'ast Vec<Definition>) {
        for definition in definitions {
            self.visit_definition(definition);
        }
    }

    fn visit_argument(&mut self, argument: &'ast Argument) {
        walk_argument(self, argument);
    }

    fn visit_argument_list_extended_attribute(&mut self, ex: &'ast ArgumentListExtendedAttribute) {
        walk_argument_list_extended_attribute(self, ex);
    }

    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        walk_attribute(self, attribute);
    }

    fn visit_callback(&mut self, callback: &'ast Callback) {
        walk_callback(self, callback);
    }

    fn visit_callback_interface(&mut self, callback_interface: &'ast CallbackInterface) {
        walk_callback_interface(self, callback_interface);
    }

    fn visit_const(&mut self, const_: &'ast Const) {
        walk_const(self, const_);
    }

    fn visit_const_type(&mut self, const_type: &'ast ConstType) {
        walk_const_type(self, const_type);
    }

    fn visit_const_value(&mut self, _const_value: &'ast ConstValue) {}

    fn visit_default_value(&mut self, default_value: &'ast DefaultValue) {
        walk_default_value(self, default_value);
    }

    fn visit_definition(&mut self, definition: &'ast Definition) {
        walk_definition(self, definition);
    }

    fn visit_dictionary(&mut self, dictionary: &'ast Dictionary) {
        walk_dictionary(self, dictionary);
    }

    fn visit_dictionary_member(&mut self, dictionary_member: &'ast DictionaryMember) {
        walk_dictionary_member(self, dictionary_member);
    }

    fn visit_enum(&mut self, enum_: &'ast Enum) {
        walk_enum(self, enum_);
    }

    fn visit_explicit_stringifier_operation(&mut self,
                                            operation: &'ast ExplicitStringifierOperation) {
        walk_explicit_stringifier_operation(self, operation);
    }

    fn visit_extended_attribute(&mut self, extended_attribute: &'ast ExtendedAttribute) {
        walk_extended_attribute(self, extended_attribute);
    }

    fn visit_identifier(&mut self, _identifier: &'ast Identifier) {}

    fn visit_identifier_extended_attribute(&mut self,
                                           extended_attribute: &'ast IdentifierExtendedAttribute) {
        walk_identifier_extended_attribute(self, extended_attribute);
    }

    fn visit_identifier_list_extended_attribute(&mut self,
                                                ex: &'ast IdentifierListExtendedAttribute) {
        walk_identifier_list_extended_attribute(self, ex);
    }

    fn visit_implements(&mut self, implements: &'ast Implements) {
        walk_implements(self, implements);
    }

    fn visit_implicit_stringifier_operation(&mut self,
                                            operation: &'ast ImplicitStringifierOperation) {
        walk_implicit_stringifier_operation(self, operation);
    }

    fn visit_interface(&mut self, interface: &'ast Interface) {
        walk_interface(self, interface);
    }

    fn visit_interface_member(&mut self, interface_member: &'ast InterfaceMember) {
        walk_interface_member(self, interface_member);
    }

    fn visit_iterable(&mut self, iterable: &'ast Iterable) {
        walk_iterable(self, iterable);
    }

    fn visit_maplike(&mut self, maplike: &'ast Maplike) {
        walk_maplike(self, maplike);
    }

    fn visit_named_argument_list_extended_attribute(&mut self,
                                                    ex: &'ast NamedArgumentListExtendedAttribute) {
        walk_named_argument_list_extended_attribute(self, ex);
    }

    fn visit_namespace(&mut self, namespace: &'ast Namespace) {
        walk_namespace(self, namespace);
    }

    fn visit_namespace_member(&mut self, namespace_member: &'ast NamespaceMember) {
        walk_namespace_member(self, namespace_member);
    }

    fn visit_non_partial_dictionary(&mut self,
                                    non_partial_dictionary: &'ast NonPartialDictionary) {
        walk_non_partial_dictionary(self, non_partial_dictionary);
    }

    fn visit_non_partial_interface(&mut self, non_partial_interface: &'ast NonPartialInterface) {
        walk_non_partial_interface(self, non_partial_interface);
    }

    fn visit_non_partial_namespace(&mut self, non_partial_namespace: &'ast NonPartialNamespace) {
        walk_non_partial_namespace(self, non_partial_namespace);
    }

    fn visit_operation(&mut self, operation: &'ast Operation) {
        walk_operation(self, operation);
    }

    fn visit_other(&mut self, other: &'ast Other) {
        walk_other(self, other);
    }

    fn visit_other_extended_attribute(&mut self,
                                      extended_attribute: &'ast OtherExtendedAttribute) {
        walk_other_extended_attribute(self, extended_attribute);
    }

    fn visit_partial_dictionary(&mut self, partial_dictionary: &'ast PartialDictionary) {
        walk_partial_dictionary(self, partial_dictionary);
    }

    fn visit_partial_interface(&mut self, partial_interface: &'ast PartialInterface) {
        walk_partial_interface(self, partial_interface);
    }

    fn visit_partial_namespace(&mut self, partial_namespace: &'ast PartialNamespace) {
        walk_partial_namespace(self, partial_namespace);
    }

    fn visit_regular_attribute(&mut self, regular_attribute: &'ast RegularAttribute) {
        walk_regular_attribute(self, regular_attribute);
    }

    fn visit_regular_operation(&mut self, regular_operation: &'ast RegularOperation) {
        walk_regular_operation(self, regular_operation);
    }

    fn visit_return_type(&mut self, return_type: &'ast ReturnType) {
        walk_return_type(self, return_type);
    }

    fn visit_setlike(&mut self, setlike: &'ast Setlike) {
        walk_setlike(self, setlike);
    }

    fn visit_special(&mut self, _special: &'ast Special) {}

    fn visit_special_operation(&mut self, special_operation: &'ast SpecialOperation) {
        walk_special_operation(self, special_operation);
    }

    fn visit_static_attribute(&mut self, static_attribute: &'ast StaticAttribute) {
        walk_static_attribute(self, static_attribute);
    }

    fn visit_static_operation(&mut self, static_operation: &'ast StaticOperation) {
        walk_static_operation(self, static_operation);
    }

    fn visit_string_type(&mut self, _string_type: &'ast StringType) {}

    fn visit_stringifier_attribute(&mut self, stringifier_attribute: &'ast StringifierAttribute) {
        walk_stringifier_attribute(self, stringifier_attribute);
    }

    fn visit_stringifier_operation(&mut self, stringifier_operation: &'ast StringifierOperation) {
        walk_stringifier_operation(self, stringifier_operation);
    }

    fn visit_type(&mut self, type_: &'ast Type) {
        walk_type(self, type_);
    }

    fn visit_type_kind(&mut self, type_kind: &'ast TypeKind) {
        walk_type_kind(self, type_kind);
    }

    fn visit_typedef(&mut self, typedef: &'ast Typedef) {
        walk_typedef(self, typedef);
    }
}

#[macro_export]
macro_rules! walk_list {
    ($visitor: expr, $method: ident, $list: expr) => {
        for elem in $list.iter() {
            $visitor.$method(&elem)
        }
    }
}

pub fn walk_argument<'ast, V: Visitor<'ast>>(visitor: &mut V, argument: &'ast Argument) {
    walk_list!(visitor,
               visit_extended_attribute,
               &argument.extended_attributes);
    visitor.visit_type(&argument.type_);
    visitor.visit_identifier(&argument.name);

    if let Some(ref default_value) = argument.default {
        visitor.visit_default_value(default_value);
    }
}

pub fn walk_argument_list_extended_attribute<'ast, V: Visitor<'ast>>(visitor: &mut V,
ex: &'ast ArgumentListExtendedAttribute){
    visitor.visit_identifier(&ex.name);
    walk_list!(visitor, visit_argument, &ex.arguments);
}

pub fn walk_attribute<'ast, V: Visitor<'ast>>(visitor: &mut V, attribute: &'ast Attribute) {
    match *attribute {
        Attribute::Regular(ref attribute) => visitor.visit_regular_attribute(&attribute),
        Attribute::Static(ref attribute) => visitor.visit_static_attribute(&attribute),
        Attribute::Stringifier(ref attribute) => visitor.visit_stringifier_attribute(&attribute),
    }
}

pub fn walk_callback<'ast, V: Visitor<'ast>>(visitor: &mut V, callback: &'ast Callback) {
    walk_list!(visitor,
               visit_extended_attribute,
               &callback.extended_attributes);
    visitor.visit_identifier(&callback.name);
    visitor.visit_return_type(&callback.return_type);
    walk_list!(visitor, visit_argument, &callback.arguments);
}

pub fn walk_callback_interface<'ast, V>(visitor: &mut V,
                                        callback_interface: &'ast CallbackInterface)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &callback_interface.extended_attributes);
    visitor.visit_identifier(&callback_interface.name);

    if let Some(ref inherits) = callback_interface.inherits {
        visitor.visit_identifier(inherits);
    }

    walk_list!(visitor, visit_interface_member, &callback_interface.members);
}

pub fn walk_const<'ast, V: Visitor<'ast>>(visitor: &mut V, const_: &'ast Const) {
    walk_list!(visitor,
               visit_extended_attribute,
               &const_.extended_attributes);
    visitor.visit_const_type(&const_.type_);
    visitor.visit_identifier(&const_.name);
    visitor.visit_const_value(&const_.value);
}

pub fn walk_const_type<'ast, V: Visitor<'ast>>(visitor: &mut V, const_type: &'ast ConstType) {
    if let ConstType::Identifier(ref identifier) = *const_type {
        visitor.visit_identifier(identifier);
    }
}

pub fn walk_default_value<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                  default_value: &'ast DefaultValue) {
    if let DefaultValue::ConstValue(ref const_value) = *default_value {
        visitor.visit_const_value(const_value);
    }
}

pub fn walk_definition<'ast, V: Visitor<'ast>>(visitor: &mut V, definition: &'ast Definition) {
    match *definition {
        Definition::Callback(ref callback) => visitor.visit_callback(callback),
        Definition::Dictionary(ref dictionary) => visitor.visit_dictionary(dictionary),
        Definition::Enum(ref enum_) => visitor.visit_enum(enum_),
        Definition::Implements(ref implements) => visitor.visit_implements(implements),
        Definition::Interface(ref interface) => visitor.visit_interface(interface),
        Definition::Namespace(ref namespace) => visitor.visit_namespace(namespace),
        Definition::Typedef(ref typedef) => visitor.visit_typedef(typedef),
    }
}

pub fn walk_dictionary<'ast, V: Visitor<'ast>>(visitor: &mut V, dictionary: &'ast Dictionary) {
    match *dictionary {
        Dictionary::NonPartial(ref dictionary) => visitor.visit_non_partial_dictionary(dictionary),
        Dictionary::Partial(ref dictionary) => visitor.visit_partial_dictionary(dictionary),
    }
}

pub fn walk_dictionary_member<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                      dictionary_member: &'ast DictionaryMember) {
    walk_list!(visitor,
               visit_extended_attribute,
               &dictionary_member.extended_attributes);
    visitor.visit_type(&dictionary_member.type_);
    visitor.visit_identifier(&dictionary_member.name);

    if let Some(ref default_value) = dictionary_member.default {
        visitor.visit_default_value(default_value);
    }
}

pub fn walk_enum<'ast, V: Visitor<'ast>>(visitor: &mut V, enum_: &'ast Enum) {
    walk_list!(visitor,
               visit_extended_attribute,
               &enum_.extended_attributes);
    visitor.visit_identifier(&enum_.name);
}

pub fn walk_explicit_stringifier_operation<'ast, V>(visitor: &mut V,
                                                    operation: &'ast ExplicitStringifierOperation)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &operation.extended_attributes);
    visitor.visit_return_type(&operation.return_type);

    if let Some(ref name) = operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &operation.arguments);
}

pub fn walk_extended_attribute<'ast, V>(visitor: &mut V,
                                        extended_attribute: &'ast ExtendedAttribute)
    where V: Visitor<'ast>
{
    match *extended_attribute {
        ExtendedAttribute::ArgumentList(ref extended_attribute) => {
            visitor.visit_argument_list_extended_attribute(extended_attribute);
        }
        ExtendedAttribute::Identifier(ref extended_attribute) => {
            visitor.visit_identifier_extended_attribute(extended_attribute);
        }
        ExtendedAttribute::IdentifierList(ref extended_attribute) => {
            visitor.visit_identifier_list_extended_attribute(extended_attribute);
        }
        ExtendedAttribute::NamedArgumentList(ref extended_attribute) => {
            visitor.visit_named_argument_list_extended_attribute(extended_attribute);
        }
        ExtendedAttribute::NoArguments(ref extended_attribute) => {
            visitor.visit_other(extended_attribute);
        }
    }
}

pub fn walk_identifier_extended_attribute<'ast, V>(visitor: &mut V,
                                                   ex: &'ast IdentifierExtendedAttribute)
    where V: Visitor<'ast>
{
    visitor.visit_identifier(&ex.lhs);
    visitor.visit_other(&ex.rhs);
}

pub fn walk_identifier_list_extended_attribute<'ast, V>(visitor: &mut V,
                                                        ex: &'ast IdentifierListExtendedAttribute)
    where V: Visitor<'ast>
{
    visitor.visit_identifier(&ex.lhs);
    walk_list!(visitor, visit_identifier, &ex.rhs);
}

pub fn walk_implements<'ast, V: Visitor<'ast>>(visitor: &mut V, implements: &'ast Implements) {
    walk_list!(visitor,
               visit_extended_attribute,
               &implements.extended_attributes);
    visitor.visit_identifier(&implements.implementor);
    visitor.visit_identifier(&implements.implementee);
}

pub fn walk_implicit_stringifier_operation<'ast, V>(visitor: &mut V,
                                                    op: &'ast ImplicitStringifierOperation)
    where V: Visitor<'ast>
{
    walk_list!(visitor, visit_extended_attribute, op.extended_attributes);
}

pub fn walk_interface<'ast, V: Visitor<'ast>>(visitor: &mut V, interface: &'ast Interface) {
    match *interface {
        Interface::Callback(ref interface) => visitor.visit_callback_interface(interface),
        Interface::NonPartial(ref interface) => visitor.visit_non_partial_interface(interface),
        Interface::Partial(ref interface) => visitor.visit_partial_interface(interface),
    }
}

pub fn walk_interface_member<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                     interface_member: &'ast InterfaceMember) {
    match *interface_member {
        InterfaceMember::Attribute(ref member) => visitor.visit_attribute(member),
        InterfaceMember::Const(ref member) => visitor.visit_const(member),
        InterfaceMember::Iterable(ref member) => visitor.visit_iterable(member),
        InterfaceMember::Maplike(ref member) => visitor.visit_maplike(member),
        InterfaceMember::Operation(ref member) => visitor.visit_operation(member),
        InterfaceMember::Setlike(ref member) => visitor.visit_setlike(member),
    }
}

pub fn walk_iterable<'ast, V: Visitor<'ast>>(visitor: &mut V, iterable: &'ast Iterable) {
    walk_list!(visitor,
               visit_extended_attribute,
               &iterable.extended_attributes);

    if let Some(ref key_type) = iterable.key_type {
        visitor.visit_type(key_type);
    }

    visitor.visit_type(&iterable.value_type);
}

pub fn walk_maplike<'ast, V: Visitor<'ast>>(visitor: &mut V, maplike: &'ast Maplike) {
    walk_list!(visitor,
               visit_extended_attribute,
               &maplike.extended_attributes);
    visitor.visit_type(&maplike.key_type);
    visitor.visit_type(&maplike.value_type);
}

pub fn walk_named_argument_list_extended_attribute<'ast, V>(visitor: &mut V,
                                                      ex: &'ast NamedArgumentListExtendedAttribute)
    where V: Visitor<'ast>
{
    visitor.visit_identifier(&ex.lhs_name);
    visitor.visit_identifier(&ex.rhs_name);
    walk_list!(visitor, visit_argument, &ex.rhs_arguments);
}

pub fn walk_namespace<'ast, V: Visitor<'ast>>(visitor: &mut V, namespace: &'ast Namespace) {
    match *namespace {
        Namespace::NonPartial(ref namespace) => visitor.visit_non_partial_namespace(namespace),
        Namespace::Partial(ref namespace) => visitor.visit_partial_namespace(namespace),
    }
}

pub fn walk_namespace_member<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                     namespace_member: &'ast NamespaceMember) {
    match *namespace_member {
        NamespaceMember::Attribute(ref member) => visitor.visit_attribute(member),
        NamespaceMember::Operation(ref member) => visitor.visit_operation(member),
    }
}

pub fn walk_non_partial_dictionary<'ast, V>(visitor: &mut V,
                                            non_partial_dictionary: &'ast NonPartialDictionary)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &non_partial_dictionary.extended_attributes);
    visitor.visit_identifier(&non_partial_dictionary.name);

    if let Some(ref inherits) = non_partial_dictionary.inherits {
        visitor.visit_identifier(inherits);
    }

    walk_list!(visitor,
               visit_dictionary_member,
               &non_partial_dictionary.members);
}

pub fn walk_non_partial_interface<'ast, V>(visitor: &mut V,
                                           non_partial_interface: &'ast NonPartialInterface)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &non_partial_interface.extended_attributes);
    visitor.visit_identifier(&non_partial_interface.name);

    if let Some(ref inherits) = non_partial_interface.inherits {
        visitor.visit_identifier(inherits);
    }

    walk_list!(visitor,
               visit_interface_member,
               &non_partial_interface.members);
}

pub fn walk_non_partial_namespace<'ast, V>(visitor: &mut V,
                                           non_partial_namespace: &'ast NonPartialNamespace)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &non_partial_namespace.extended_attributes);
    visitor.visit_identifier(&non_partial_namespace.name);
    walk_list!(visitor,
               visit_namespace_member,
               &non_partial_namespace.members);
}

pub fn walk_operation<'ast, V: Visitor<'ast>>(visitor: &mut V, operation: &'ast Operation) {
    match *operation {
        Operation::Regular(ref operation) => visitor.visit_regular_operation(operation),
        Operation::Special(ref operation) => visitor.visit_special_operation(operation),
        Operation::Static(ref operation) => visitor.visit_static_operation(operation),
        Operation::Stringifier(ref operation) => visitor.visit_stringifier_operation(operation),
    }
}

pub fn walk_other<'ast, V: Visitor<'ast>>(visitor: &mut V, other: &'ast Other) {
    if let Other::Identifier(ref identifier) = *other {
        visitor.visit_identifier(identifier);
    }
}

pub fn walk_other_extended_attribute<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                             ex: &'ast OtherExtendedAttribute) {
    match *ex {
        OtherExtendedAttribute::Nested {
            ref inner,
            ref rest,
            ..
        } => {
            if let Some(ref inner) = *inner {
                visitor.visit_extended_attribute(inner);
            }

            if let Some(ref rest) = *rest {
                visitor.visit_extended_attribute(rest);
            }
        }
        OtherExtendedAttribute::Other {
            ref other,
            ref rest,
            ..
        } => {
            if let Some(ref other) = *other {
                visitor.visit_other(other);
            }

            if let Some(ref rest) = *rest {
                visitor.visit_extended_attribute(rest);
            }
        }
    }
}

pub fn walk_partial_dictionary<'ast, V>(visitor: &mut V,
                                        partial_dictionary: &'ast PartialDictionary)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &partial_dictionary.extended_attributes);
    visitor.visit_identifier(&partial_dictionary.name);
    walk_list!(visitor,
               visit_dictionary_member,
               &partial_dictionary.members);
}

pub fn walk_partial_interface<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                      partial_interface: &'ast PartialInterface) {
    walk_list!(visitor,
               visit_extended_attribute,
               &partial_interface.extended_attributes);
    visitor.visit_identifier(&partial_interface.name);
    walk_list!(visitor, visit_interface_member, &partial_interface.members);
}

pub fn walk_partial_namespace<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                      partial_namespace: &'ast PartialNamespace) {
    walk_list!(visitor,
               visit_extended_attribute,
               &partial_namespace.extended_attributes);
    visitor.visit_identifier(&partial_namespace.name);
    walk_list!(visitor, visit_namespace_member, &partial_namespace.members);
}

pub fn walk_regular_attribute<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                      regular_attribute: &'ast RegularAttribute) {
    walk_list!(visitor,
               visit_extended_attribute,
               &regular_attribute.extended_attributes);
    visitor.visit_type(&regular_attribute.type_);
    visitor.visit_identifier(&regular_attribute.name);
}

pub fn walk_regular_operation<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                      regular_operation: &'ast RegularOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               &regular_operation.extended_attributes);
    visitor.visit_return_type(&regular_operation.return_type);

    if let Some(ref name) = regular_operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &regular_operation.arguments);
}

pub fn walk_return_type<'ast, V: Visitor<'ast>>(visitor: &mut V, return_type: &'ast ReturnType) {
    if let ReturnType::NonVoid(ref type_) = *return_type {
        visitor.visit_type(type_);
    }
}

pub fn walk_setlike<'ast, V: Visitor<'ast>>(visitor: &mut V, setlike: &'ast Setlike) {
    walk_list!(visitor,
               visit_extended_attribute,
               &setlike.extended_attributes);
    visitor.visit_type(&setlike.type_);
}

pub fn walk_special_operation<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                      special_operation: &'ast SpecialOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               &special_operation.extended_attributes);
    walk_list!(visitor, visit_special, &special_operation.special_keywords);
    visitor.visit_return_type(&special_operation.return_type);

    if let Some(ref name) = special_operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &special_operation.arguments);
}

pub fn walk_static_attribute<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                     static_attribute: &'ast StaticAttribute) {
    walk_list!(visitor,
               visit_extended_attribute,
               &static_attribute.extended_attributes);
    visitor.visit_type(&static_attribute.type_);
    visitor.visit_identifier(&static_attribute.name);
}

pub fn walk_static_operation<'ast, V: Visitor<'ast>>(visitor: &mut V,
                                                     static_operation: &'ast StaticOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               &static_operation.extended_attributes);
    visitor.visit_return_type(&static_operation.return_type);

    if let Some(ref name) = static_operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &static_operation.arguments);
}

pub fn walk_stringifier_attribute<'ast, V>(visitor: &mut V,
                                           stringifier_attribute: &'ast StringifierAttribute)
    where V: Visitor<'ast>
{
    walk_list!(visitor,
               visit_extended_attribute,
               &stringifier_attribute.extended_attributes);
    visitor.visit_type(&stringifier_attribute.type_);
    visitor.visit_identifier(&stringifier_attribute.name);
}

pub fn walk_stringifier_operation<'ast, V>(visitor: &mut V,
                                           stringifier_operation: &'ast StringifierOperation)
    where V: Visitor<'ast>
{
    match *stringifier_operation {
        StringifierOperation::Explicit(ref operation) => {
            visitor.visit_explicit_stringifier_operation(operation);
        }
        StringifierOperation::Implicit(ref operation) => {
            visitor.visit_implicit_stringifier_operation(operation);
        }
    }
}

pub fn walk_type<'ast, V: Visitor<'ast>>(visitor: &mut V, type_: &'ast Type) {
    walk_list!(visitor,
               visit_extended_attribute,
               &type_.extended_attributes);
    visitor.visit_type_kind(&type_.kind);
}

pub fn walk_type_kind<'ast, V: Visitor<'ast>>(visitor: &mut V, type_kind: &'ast TypeKind) {
    match *type_kind {
        TypeKind::FrozenArray(ref type_) => visitor.visit_type(type_),
        TypeKind::Identifier(ref identifier) => visitor.visit_identifier(identifier),
        TypeKind::Promise(ref return_type) => visitor.visit_return_type(return_type),
        TypeKind::Record(_, ref type_) => visitor.visit_type(type_),
        TypeKind::Sequence(ref type_) => visitor.visit_type(type_),
        TypeKind::Union(ref types) => walk_list!(visitor, visit_type, types),
        _ => (),
    }
}

pub fn walk_typedef<'ast, V: Visitor<'ast>>(visitor: &mut V, typedef: &'ast Typedef) {
    walk_list!(visitor,
               visit_extended_attribute,
               &typedef.extended_attributes);
    visitor.visit_type(&typedef.type_);
    visitor.visit_identifier(&typedef.name);
}
