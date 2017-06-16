mod grammar;

/// Contains all structures related to the AST for the WebIDL grammar.
pub mod ast;

#[cfg(test)]
mod test {
    use std::f64;
    use std::fmt::Debug;

    use lexer::*;
    use super::*;
    use super::grammar::*;

    fn assert_parse<'input, T, E, U>(input: &'input str, parse_fn: &U, expected: Result<T, E>)
        where T: Debug + PartialEq,
              E: Debug + PartialEq,
              U: Fn(Lexer<'input>) -> Result<T, E>
    {
        let lexer = Lexer::new(input);
        assert_eq!(parse_fn(lexer), expected);
    }

    #[test]
    fn parse_boolean_literal() {
        assert_parse("false", &parse_BooleanLiteral, Ok(false));
        assert_parse("true", &parse_BooleanLiteral, Ok(true));
    }

    #[test]
    fn parse_const() {
        assert_parse("const boolean? name = null;",
                     &parse_Const,
                     Ok(ast::Const {
                            const_type: ast::ConstType::Boolean,
                            name: "name".to_string(),
                            nullable: true,
                            value: ast::ConstValue::Null,
                        }));
        assert_parse("const unrestricted float constant = -Infinity;",
                     &parse_Const,
                     Ok(ast::Const {
                            const_type:
                                ast::ConstType::ExtendedFloatType(ast::ExtendedFloatType {
                                                                      float_type:
                                                                          ast::FloatType::Float,
                                                                      restricted: false,
                                                                  }),
                            name: "constant".to_string(),
                            nullable: false,
                            value: ast::ConstValue::FloatLiteral(f64::NEG_INFINITY),
                        }));
    }

    #[test]
    fn parse_const_type() {
        assert_parse("identifier",
                     &parse_ConstType,
                     Ok(ast::ConstType::Identifier("identifier".to_string())));
        assert_parse("boolean", &parse_ConstType, Ok(ast::ConstType::Boolean));
        assert_parse("byte", &parse_ConstType, Ok(ast::ConstType::Byte));
        assert_parse("unrestricted float",
                     &parse_ConstType,
                     Ok(ast::ConstType::ExtendedFloatType(ast::ExtendedFloatType {
                                                              float_type: ast::FloatType::Float,
                                                              restricted: false,
                                                          })));
        assert_parse("long long",
                     &parse_ConstType,
                     Ok(ast::ConstType::ExtendedIntegerType(ast::ExtendedIntegerType {
                                                                integer_type:
                                                                    ast::IntegerType::LongLong,
                                                                unsigned: false,
                                                            })));
        assert_parse("octet", &parse_ConstType, Ok(ast::ConstType::Octet));
    }

    #[test]
    fn parse_const_value() {
        assert_parse("true",
                     &parse_ConstValue,
                     Ok(ast::ConstValue::BooleanLiteral(true)));
        assert_parse("7.e4",
                     &parse_ConstValue,
                     Ok(ast::ConstValue::FloatLiteral(7.0e4)));
        assert_parse("0xFF",
                     &parse_ConstValue,
                     Ok(ast::ConstValue::IntegerLiteral(0xFF)));
        assert_parse("null", &parse_ConstValue, Ok(ast::ConstValue::Null));
    }

    #[test]
    fn parse_extended_float_type() {
        assert_parse("unrestricted double",
                     &parse_ExtendedFloatType,
                     Ok(ast::ExtendedFloatType {
                            float_type: ast::FloatType::Double,
                            restricted: false,
                        }));
        assert_parse("float",
                     &parse_ExtendedFloatType,
                     Ok(ast::ExtendedFloatType {
                            float_type: ast::FloatType::Float,
                            restricted: true,
                        }));
    }

    #[test]
    fn parse_extended_integer_type() {
        assert_parse("unsigned short",
                     &parse_ExtendedIntegerType,
                     Ok(ast::ExtendedIntegerType {
                            integer_type: ast::IntegerType::Short,
                            unsigned: true,
                        }));
        assert_parse("unsigned long long",
                     &parse_ExtendedIntegerType,
                     Ok(ast::ExtendedIntegerType {
                            integer_type: ast::IntegerType::LongLong,
                            unsigned: true,
                        }));
        assert_parse("long",
                     &parse_ExtendedIntegerType,
                     Ok(ast::ExtendedIntegerType {
                            integer_type: ast::IntegerType::Long,
                            unsigned: false,
                        }));
    }

    #[test]
    fn parse_float_literal() {
        assert_parse("5.2e2", &parse_FloatLiteral, Ok(5.2e2));
        assert_parse("-0023.5", &parse_FloatLiteral, Ok(-23.5));
        assert_parse("-Infinity", &parse_FloatLiteral, Ok(f64::NEG_INFINITY));
        assert_parse("Infinity", &parse_FloatLiteral, Ok(f64::INFINITY));

        // NaN will always return false when compared to itself. Have to use the `is_nan` function.
        let lexer = Lexer::new("NaN");
        assert!(parse_FloatLiteral(lexer).unwrap().is_nan());
    }

    #[test]
    fn parse_float_type() {
        assert_parse("double", &parse_FloatType, Ok(ast::FloatType::Double));
        assert_parse("float", &parse_FloatType, Ok(ast::FloatType::Float));
    }

    #[test]
    fn parse_integer_type() {
        assert_parse("short", &parse_IntegerType, Ok(ast::IntegerType::Short));
        assert_parse("long", &parse_IntegerType, Ok(ast::IntegerType::Long));
        assert_parse("long long",
                     &parse_IntegerType,
                     Ok(ast::IntegerType::LongLong));
    }
}
