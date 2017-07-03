extern crate webidl;
extern crate zip;

use std::fs;
use std::io::Read;

use webidl::*;

#[test]
fn parse_servo_webidls() {
    let parser = Parser::new();
    let file = fs::File::open("tests/mozilla_webidls.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut webidl = String::new();
        file.read_to_string(&mut webidl).unwrap();

        assert!(parser.parse_string(&*webidl).is_ok());
    }
}
