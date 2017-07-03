extern crate webidl;
extern crate zip;

use std::f64;
use std::fs;
use std::io::Read;

use webidl::*;
use webidl::ast::*;
use webidl::visitor::*;

// Test to make sure that Infinity/-Infinity/NaN are correctly pretty printed since they do not
// appear in the Servo WebIDLs.
#[test]
fn pretty_print_float_literals() {
    let ast = vec![Definition::Interface(Interface::NonPartial(NonPartialInterface {
                    extended_attributes: vec![],
                    inherits: None,
                    members: vec![InterfaceMember::Const(Const {
                        extended_attributes: vec![],
                        name: "const_1".to_string(),
                        nullable: false,
                        type_: ConstType::UnrestrictedDouble,
                        value: ConstValue::FloatLiteral(f64::INFINITY)
                    }), InterfaceMember::Const(Const {
                        extended_attributes: vec![],
                        name: "const_2".to_string(),
                        nullable: false,
                        type_: ConstType::UnrestrictedDouble,
                        value: ConstValue::FloatLiteral(f64::NEG_INFINITY)
                    }), InterfaceMember::Const(Const {
                        extended_attributes: vec![],
                        name: "const_3".to_string(),
                        nullable: false,
                        type_: ConstType::UnrestrictedDouble,
                        value: ConstValue::FloatLiteral(f64::NAN)
                    })],
                    name: "Test".to_string()
              }))];
    let mut visitor = PrettyPrintVisitor::new();
    visitor.visit(&ast);
    assert_eq!(visitor.get_output(),
               "interface Test {
    const unrestricted double const_1 = Infinity;
    const unrestricted double const_2 = -Infinity;
    const unrestricted double const_3 = NaN;
};\n\n");
}

#[test]
fn pretty_print_servo_webidls() {
    let parser = Parser::new();
    let file = fs::File::open("tests/mozilla_webidls.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut webidl = String::new();
        file.read_to_string(&mut webidl).unwrap();

        let original_ast = parser.parse_string(&*webidl).unwrap();
        let mut visitor = PrettyPrintVisitor::new();
        visitor.visit(&original_ast);

        // Compare original AST with AST obtained from pretty print visitor.

        let pretty_print_ast = parser.parse_string(&*visitor.get_output()).unwrap();
        assert_eq!(pretty_print_ast, original_ast);
    }
}
