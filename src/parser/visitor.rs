#![allow(missing_docs)]

use super::ast::*;

pub trait Visitor: Sized {
    fn visit_argument(&mut self, argument: &Argument) {
        walk_argument(self, argument);
    }

    fn visit_argument_list_extended_attribute(&mut self,
                                              extended_attribute: &ArgumentListExtendedAttribute) {
        walk_argument_list_extended_attribute(self, extended_attribute);
    }

    fn visit_attribute(&mut self, attribute: &Attribute) {
        walk_attribute(self, attribute);
    }

    fn visit_callback(&mut self, callback: &Callback) {
        walk_callback(self, callback);
    }

    fn visit_callback_interface(&mut self, callback_interface: &CallbackInterface) {
        walk_callback_interface(self, callback_interface);
    }

    fn visit_const(&mut self, const_: &Const) {
        walk_const(self, const_);
    }

    fn visit_const_type(&mut self, const_type: &ConstType) {
        walk_const_type(self, const_type);
    }

    fn visit_const_value(&mut self, _const_value: &ConstValue) {}

    fn visit_default_value(&mut self, default_value: &DefaultValue) {
        walk_default_value(self, default_value);
    }

    fn visit_definition(&mut self, definition: &Definition) {
        walk_definition(self, definition);
    }

    fn visit_dictionary(&mut self, dictionary: &Dictionary) {
        walk_dictionary(self, dictionary);
    }

    fn visit_dictionary_member(&mut self, dictionary_member: &DictionaryMember) {
        walk_dictionary_member(self, dictionary_member);
    }

    fn visit_enum(&mut self, enum_: &Enum) {
        walk_enum(self, enum_);
    }

    fn visit_explicit_stringifier_operation(&mut self, operation: &ExplicitStringifierOperation) {
        walk_explicit_stringifier_operation(self, operation);
    }

    fn visit_extended_attribute(&mut self, extended_attribute: &ExtendedAttribute) {
        walk_extended_attribute(self, extended_attribute);
    }

    fn visit_identifier(&mut self, _identifier: &Identifier) {}

    fn visit_identifier_extended_attribute(&mut self,
                                           extended_attribute: &IdentifierExtendedAttribute) {
        walk_identifier_extended_attribute(self, extended_attribute);
    }

    fn visit_identifier_list_extended_attribute(&mut self, ex: &IdentifierListExtendedAttribute) {
        walk_identifier_list_extended_attribute(self, ex);
    }

    fn visit_implements(&mut self, implements: &Implements) {
        walk_implements(self, implements);
    }

    fn visit_implicit_stringifier_operation(&mut self, operation: &ImplicitStringifierOperation) {
        walk_implicit_stringifier_operation(self, operation);
    }

    fn visit_interface(&mut self, interface: &Interface) {
        walk_interface(self, interface);
    }

    fn visit_interface_member(&mut self, interface_member: &InterfaceMember) {
        walk_interface_member(self, interface_member);
    }

    fn visit_iterable(&mut self, iterable: &Iterable) {
        walk_iterable(self, iterable);
    }

    fn visit_maplike(&mut self, maplike: &Maplike) {
        walk_maplike(self, maplike);
    }

    fn visit_named_argument_list_extended_attribute(&mut self,
                                                    ex: &NamedArgumentListExtendedAttribute) {
        walk_named_argument_list_extended_attribute(self, ex);
    }

    fn visit_namespace(&mut self, namespace: &Namespace) {
        walk_namespace(self, namespace);
    }

    fn visit_namespace_member(&mut self, namespace_member: &NamespaceMember) {
        walk_namespace_member(self, namespace_member);
    }

    fn visit_non_partial_dictionary(&mut self, non_partial_dictionary: &NonPartialDictionary) {
        walk_non_partial_dictionary(self, non_partial_dictionary);
    }

    fn visit_non_partial_interface(&mut self, non_partial_interface: &NonPartialInterface) {
        walk_non_partial_interface(self, non_partial_interface);
    }

    fn visit_non_partial_namespace(&mut self, non_partial_namespace: &NonPartialNamespace) {
        walk_non_partial_namespace(self, non_partial_namespace);
    }

    fn visit_operation(&mut self, operation: &Operation) {
        walk_operation(self, operation);
    }

    fn visit_other(&mut self, other: &Other) {
        walk_other(self, other);
    }

    fn visit_other_extended_attribute(&mut self, extended_attribute: &OtherExtendedAttribute) {
        walk_other_extended_attribute(self, extended_attribute);
    }

    fn visit_partial_dictionary(&mut self, partial_dictionary: &PartialDictionary) {
        walk_partial_dictionary(self, partial_dictionary);
    }

    fn visit_partial_interface(&mut self, partial_interface: &PartialInterface) {
        walk_partial_interface(self, partial_interface);
    }

    fn visit_partial_namespace(&mut self, partial_namespace: &PartialNamespace) {
        walk_partial_namespace(self, partial_namespace);
    }

    fn visit_regular_attribute(&mut self, regular_attribute: &RegularAttribute) {
        walk_regular_attribute(self, regular_attribute);
    }

    fn visit_regular_operation(&mut self, regular_operation: &RegularOperation) {
        walk_regular_operation(self, regular_operation);
    }

    fn visit_return_type(&mut self, return_type: &ReturnType) {
        walk_return_type(self, return_type);
    }

    fn visit_setlike(&mut self, setlike: &Setlike) {
        walk_setlike(self, setlike);
    }

    fn visit_special(&mut self, _special: &Special) {}

    fn visit_special_operation(&mut self, special_operation: &SpecialOperation) {
        walk_special_operation(self, special_operation);
    }

    fn visit_static_attribute(&mut self, static_attribute: &StaticAttribute) {
        walk_static_attribute(self, static_attribute);
    }

    fn visit_static_operation(&mut self, static_operation: &StaticOperation) {
        walk_static_operation(self, static_operation);
    }

    fn visit_string_type(&mut self, _string_type: &StringType) {}

    fn visit_stringifier_attribute(&mut self, stringifier_attribute: &StringifierAttribute) {
        walk_stringifier_attribute(self, stringifier_attribute);
    }

    fn visit_stringifier_operation(&mut self, stringifier_operation: &StringifierOperation) {
        walk_stringifier_operation(self, stringifier_operation);
    }

    fn visit_type(&mut self, type_: &Type) {
        walk_type(self, type_);
    }

    fn visit_type_kind(&mut self, type_kind: &TypeKind) {
        walk_type_kind(self, type_kind);
    }

    fn visit_typedef(&mut self, typedef: &Typedef) {
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

pub fn walk_argument<V: Visitor>(visitor: &mut V, argument: &Argument) {
    walk_list!(visitor,
               visit_extended_attribute,
               &argument.extended_attributes);
    visitor.visit_type(&argument.type_);
    visitor.visit_identifier(&argument.name);

    if let Some(ref default_value) = argument.default {
        visitor.visit_default_value(default_value);
    }
}

pub fn walk_argument_list_extended_attribute<V: Visitor>(visitor: &mut V,
                                                         ex: &ArgumentListExtendedAttribute) {
    visitor.visit_identifier(&ex.name);
    walk_list!(visitor, visit_argument, &ex.arguments);
}

pub fn walk_attribute<V: Visitor>(visitor: &mut V, attribute: &Attribute) {
    match *attribute {
        Attribute::Regular(ref attribute) => visitor.visit_regular_attribute(&attribute),
        Attribute::Static(ref attribute) => visitor.visit_static_attribute(&attribute),
        Attribute::Stringifier(ref attribute) => visitor.visit_stringifier_attribute(&attribute),
    }
}

pub fn walk_callback<V: Visitor>(visitor: &mut V, callback: &Callback) {
    walk_list!(visitor,
               visit_extended_attribute,
               &callback.extended_attributes);
    visitor.visit_identifier(&callback.name);
    visitor.visit_return_type(&callback.return_type);
    walk_list!(visitor, visit_argument, &callback.arguments);
}

pub fn walk_callback_interface<V: Visitor>(visitor: &mut V,
                                           callback_interface: &CallbackInterface) {
    walk_list!(visitor,
               visit_extended_attribute,
               &callback_interface.extended_attributes);
    visitor.visit_identifier(&callback_interface.name);

    if let Some(ref inherits) = callback_interface.inherits {
        visitor.visit_identifier(inherits);
    }

    walk_list!(visitor, visit_interface_member, &callback_interface.members);
}

pub fn walk_const<V: Visitor>(visitor: &mut V, const_: &Const) {
    walk_list!(visitor,
               visit_extended_attribute,
               &const_.extended_attributes);
    visitor.visit_const_type(&const_.type_);
    visitor.visit_identifier(&const_.name);
    visitor.visit_const_value(&const_.value);
}

pub fn walk_const_type<V: Visitor>(visitor: &mut V, const_type: &ConstType) {
    if let ConstType::Identifier(ref identifier) = *const_type {
        visitor.visit_identifier(identifier);
    }
}

pub fn walk_default_value<V: Visitor>(visitor: &mut V, default_value: &DefaultValue) {
    if let DefaultValue::ConstValue(ref const_value) = *default_value {
        visitor.visit_const_value(const_value);
    }
}

pub fn walk_definition<V: Visitor>(visitor: &mut V, definition: &Definition) {
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

pub fn walk_dictionary<V: Visitor>(visitor: &mut V, dictionary: &Dictionary) {
    match *dictionary {
        Dictionary::NonPartial(ref dictionary) => visitor.visit_non_partial_dictionary(dictionary),
        Dictionary::Partial(ref dictionary) => visitor.visit_partial_dictionary(dictionary),
    }
}

pub fn walk_dictionary_member<V: Visitor>(visitor: &mut V, dictionary_member: &DictionaryMember) {
    walk_list!(visitor,
               visit_extended_attribute,
               &dictionary_member.extended_attributes);
    visitor.visit_type(&dictionary_member.type_);
    visitor.visit_identifier(&dictionary_member.name);

    if let Some(ref default_value) = dictionary_member.default {
        visitor.visit_default_value(default_value);
    }
}

pub fn walk_enum<V: Visitor>(visitor: &mut V, enum_: &Enum) {
    walk_list!(visitor,
               visit_extended_attribute,
               &enum_.extended_attributes);
    visitor.visit_identifier(&enum_.name);
}

pub fn walk_explicit_stringifier_operation<V: Visitor>(visitor: &mut V,
                                                       operation: &ExplicitStringifierOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               &operation.extended_attributes);
    visitor.visit_return_type(&operation.return_type);

    if let Some(ref name) = operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &operation.arguments);
}

pub fn walk_extended_attribute<V: Visitor>(visitor: &mut V,
                                           extended_attribute: &ExtendedAttribute) {
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

pub fn walk_identifier_extended_attribute<V: Visitor>(visitor: &mut V,
                                                      ex: &IdentifierExtendedAttribute) {
    visitor.visit_identifier(&ex.lhs);
    visitor.visit_other(&ex.rhs);
}

pub fn walk_identifier_list_extended_attribute<V: Visitor>(visitor: &mut V,
                                                           ex: &IdentifierListExtendedAttribute) {
    visitor.visit_identifier(&ex.lhs);
    walk_list!(visitor, visit_identifier, &ex.rhs);
}

pub fn walk_implements<V: Visitor>(visitor: &mut V, implements: &Implements) {
    walk_list!(visitor,
               visit_extended_attribute,
               &implements.extended_attributes);
    visitor.visit_identifier(&implements.implementor);
    visitor.visit_identifier(&implements.implementee);
}

pub fn walk_implicit_stringifier_operation<V: Visitor>(visitor: &mut V,
                                                       operation: &ImplicitStringifierOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               operation.extended_attributes);
}

pub fn walk_interface<V: Visitor>(visitor: &mut V, interface: &Interface) {
    match *interface {
        Interface::Callback(ref interface) => visitor.visit_callback_interface(interface),
        Interface::NonPartial(ref interface) => visitor.visit_non_partial_interface(interface),
        Interface::Partial(ref interface) => visitor.visit_partial_interface(interface),
    }
}

pub fn walk_interface_member<V: Visitor>(visitor: &mut V, interface_member: &InterfaceMember) {
    match *interface_member {
        InterfaceMember::Attribute(ref member) => visitor.visit_attribute(member),
        InterfaceMember::Const(ref member) => visitor.visit_const(member),
        InterfaceMember::Iterable(ref member) => visitor.visit_iterable(member),
        InterfaceMember::Maplike(ref member) => visitor.visit_maplike(member),
        InterfaceMember::Operation(ref member) => visitor.visit_operation(member),
        InterfaceMember::Setlike(ref member) => visitor.visit_setlike(member),
    }
}

pub fn walk_iterable<V: Visitor>(visitor: &mut V, iterable: &Iterable) {
    walk_list!(visitor,
               visit_extended_attribute,
               &iterable.extended_attributes);

    if let Some(ref key_type) = iterable.key_type {
        visitor.visit_type(key_type);
    }

    visitor.visit_type(&iterable.value_type);
}

pub fn walk_maplike<V: Visitor>(visitor: &mut V, maplike: &Maplike) {
    walk_list!(visitor,
               visit_extended_attribute,
               &maplike.extended_attributes);
    visitor.visit_type(&maplike.key_type);
    visitor.visit_type(&maplike.value_type);
}

pub fn walk_named_argument_list_extended_attribute<V>(visitor: &mut V,
                                                      ex: &NamedArgumentListExtendedAttribute)
    where V: Visitor
{
    visitor.visit_identifier(&ex.lhs_name);
    visitor.visit_identifier(&ex.rhs_name);
    walk_list!(visitor, visit_argument, &ex.rhs_arguments);
}

pub fn walk_namespace<V: Visitor>(visitor: &mut V, namespace: &Namespace) {
    match *namespace {
        Namespace::NonPartial(ref namespace) => visitor.visit_non_partial_namespace(namespace),
        Namespace::Partial(ref namespace) => visitor.visit_partial_namespace(namespace),
    }
}

pub fn walk_namespace_member<V: Visitor>(visitor: &mut V, namespace_member: &NamespaceMember) {
    match *namespace_member {
        NamespaceMember::Attribute(ref member) => visitor.visit_attribute(member),
        NamespaceMember::Operation(ref member) => visitor.visit_operation(member),
    }
}

pub fn walk_non_partial_dictionary<V: Visitor>(visitor: &mut V,
                                               non_partial_dictionary: &NonPartialDictionary) {
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

pub fn walk_non_partial_interface<V: Visitor>(visitor: &mut V,
                                              non_partial_interface: &NonPartialInterface) {
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

pub fn walk_non_partial_namespace<V: Visitor>(visitor: &mut V,
                                              non_partial_namespace: &NonPartialNamespace) {
    walk_list!(visitor,
               visit_extended_attribute,
               &non_partial_namespace.extended_attributes);
    visitor.visit_identifier(&non_partial_namespace.name);
    walk_list!(visitor,
               visit_namespace_member,
               &non_partial_namespace.members);
}

pub fn walk_operation<V: Visitor>(visitor: &mut V, operation: &Operation) {
    match *operation {
        Operation::Regular(ref operation) => visitor.visit_regular_operation(operation),
        Operation::Special(ref operation) => visitor.visit_special_operation(operation),
        Operation::Static(ref operation) => visitor.visit_static_operation(operation),
        Operation::Stringifier(ref operation) => visitor.visit_stringifier_operation(operation),
    }
}

pub fn walk_other<V: Visitor>(visitor: &mut V, other: &Other) {
    if let Other::Identifier(ref identifier) = *other {
        visitor.visit_identifier(identifier);
    }
}

pub fn walk_other_extended_attribute<V: Visitor>(visitor: &mut V,
                                                 extended_attribute: &OtherExtendedAttribute) {
    match *extended_attribute {
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

pub fn walk_partial_dictionary<V: Visitor>(visitor: &mut V,
                                           partial_dictionary: &PartialDictionary) {
    walk_list!(visitor,
               visit_extended_attribute,
               &partial_dictionary.extended_attributes);
    visitor.visit_identifier(&partial_dictionary.name);
    walk_list!(visitor,
               visit_dictionary_member,
               &partial_dictionary.members);
}

pub fn walk_partial_interface<V: Visitor>(visitor: &mut V, partial_interface: &PartialInterface) {
    walk_list!(visitor,
               visit_extended_attribute,
               &partial_interface.extended_attributes);
    visitor.visit_identifier(&partial_interface.name);
    walk_list!(visitor, visit_interface_member, &partial_interface.members);
}

pub fn walk_partial_namespace<V: Visitor>(visitor: &mut V, partial_namespace: &PartialNamespace) {
    walk_list!(visitor,
               visit_extended_attribute,
               &partial_namespace.extended_attributes);
    visitor.visit_identifier(&partial_namespace.name);
    walk_list!(visitor, visit_namespace_member, &partial_namespace.members);
}

pub fn walk_regular_attribute<V: Visitor>(visitor: &mut V, regular_attribute: &RegularAttribute) {
    walk_list!(visitor,
               visit_extended_attribute,
               &regular_attribute.extended_attributes);
    visitor.visit_type(&regular_attribute.type_);
    visitor.visit_identifier(&regular_attribute.name);
}

pub fn walk_regular_operation<V: Visitor>(visitor: &mut V, regular_operation: &RegularOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               &regular_operation.extended_attributes);
    visitor.visit_return_type(&regular_operation.return_type);

    if let Some(ref name) = regular_operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &regular_operation.arguments);
}

pub fn walk_return_type<V: Visitor>(visitor: &mut V, return_type: &ReturnType) {
    if let ReturnType::NonVoid(ref type_) = *return_type {
        visitor.visit_type(type_);
    }
}

pub fn walk_setlike<V: Visitor>(visitor: &mut V, setlike: &Setlike) {
    walk_list!(visitor,
               visit_extended_attribute,
               &setlike.extended_attributes);
    visitor.visit_type(&setlike.type_);
}

pub fn walk_special_operation<V: Visitor>(visitor: &mut V, special_operation: &SpecialOperation) {
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

pub fn walk_static_attribute<V: Visitor>(visitor: &mut V, static_attribute: &StaticAttribute) {
    walk_list!(visitor,
               visit_extended_attribute,
               &static_attribute.extended_attributes);
    visitor.visit_type(&static_attribute.type_);
    visitor.visit_identifier(&static_attribute.name);
}

pub fn walk_static_operation<V: Visitor>(visitor: &mut V, static_operation: &StaticOperation) {
    walk_list!(visitor,
               visit_extended_attribute,
               &static_operation.extended_attributes);
    visitor.visit_return_type(&static_operation.return_type);

    if let Some(ref name) = static_operation.name {
        visitor.visit_identifier(name);
    }

    walk_list!(visitor, visit_argument, &static_operation.arguments);
}

pub fn walk_stringifier_attribute<V: Visitor>(visitor: &mut V,
                                              stringifier_attribute: &StringifierAttribute) {
    walk_list!(visitor,
               visit_extended_attribute,
               &stringifier_attribute.extended_attributes);
    visitor.visit_type(&stringifier_attribute.type_);
    visitor.visit_identifier(&stringifier_attribute.name);
}

pub fn walk_stringifier_operation<V: Visitor>(visitor: &mut V,
                                              stringifier_operation: &StringifierOperation) {
    match *stringifier_operation {
        StringifierOperation::Explicit(ref operation) => {
            visitor.visit_explicit_stringifier_operation(operation);
        }
        StringifierOperation::Implicit(ref operation) => {
            visitor.visit_implicit_stringifier_operation(operation);
        }
    }
}

pub fn walk_type<V: Visitor>(visitor: &mut V, type_: &Type) {
    walk_list!(visitor,
               visit_extended_attribute,
               &type_.extended_attributes);
    visitor.visit_type_kind(&type_.kind);
}

pub fn walk_type_kind<V: Visitor>(visitor: &mut V, type_kind: &TypeKind) {
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

pub fn walk_typedef<V: Visitor>(visitor: &mut V, typedef: &Typedef) {
    walk_list!(visitor,
               visit_extended_attribute,
               &typedef.extended_attributes);
    visitor.visit_type(&typedef.type_);
    visitor.visit_identifier(&typedef.name);
}
