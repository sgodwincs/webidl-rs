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

        // If the file being parsed is `DOMException.webidl`, we will skip it. I am not sure why,
        // but this file defines an interface named `DOMException`. This seems to go against what
        // the grammar suggests, that `DOMException` is a keyword (terminal) and cannot be used as
        // the name of an interface (since it is not an identifier). It would be fine if it was
        // `_DOMException` since leading underscores are removed, but that is not the case here.

        if file.name() == "DOMException.webidl" {
            continue;
        }

        let mut webidl = String::new();
        file.read_to_string(&mut webidl).unwrap();

        assert!(parser.parse_string(&*webidl).is_ok());
    }
}
