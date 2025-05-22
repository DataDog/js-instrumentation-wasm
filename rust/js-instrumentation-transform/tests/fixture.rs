use std::{fs, path::PathBuf};

use js_instrumentation_transform::apply_transform;
use similar_asserts::assert_eq;

#[testing::fixture("../../tests/fixtures/**/input.*")]
fn ast_transform_test(input_path: PathBuf) {
    let extension = input_path.extension().unwrap();

    let input = fs::read_to_string(&input_path).expect("Unable to read input file");

    let expected_filename = format!("output.{}", extension.to_string_lossy());
    let expected_path = input_path.parent().unwrap().join(expected_filename);
    let expected = fs::read_to_string(expected_path).expect("Unable to read expected output file");

    let actual = apply_transform(&input_path.to_string_lossy(), &input, &Default::default())
        .expect("Should apply transform successfully");

    assert_eq!(expected, actual.code);
}
