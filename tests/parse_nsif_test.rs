use nimage::nsif::{parse_number, parse_string, NSIF};
use std::fs::File;

#[test]
fn parse_nsif_file() {
    let file = File::open("tests/Image.nsif").unwrap();
    let nsif = NSIF::parse(&file).unwrap();
    let file_header = nsif.file_header;
    let fhdr = file_header.fhdr;
    assert_string_eq!(fhdr.value, "NSIF");
    let number_of_image_segments = file_header.numi;
    assert_int_eq!(number_of_image_segments.value, 1);
}

#[macro_export]
macro_rules! assert_string_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let field_value = left_val.as_single().unwrap();
                assert_eq!(parse_string(&field_value).unwrap(), *right_val)
            }
        }
    };
}

#[macro_export]
macro_rules! assert_int_eq {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                let field_value = left_val.as_single().unwrap();
                assert_eq!(parse_number(&field_value).unwrap(), *right_val)
            }
        }
    };
}
