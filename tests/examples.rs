#![cfg(test)]
extern crate test_generator;

use aws_iam::io;
use std::fs;
use std::path::PathBuf;
use test_generator::test_resources;

#[test_resources("tests/data/good/*.json")]
fn verify_good_examples(resource: &str) {
    println!("verify_good_examples reading file {}", resource);
    let file_name = PathBuf::from(resource);
    let result = io::read_from_file(&file_name);
    println!("{:#?}", result);
    assert!(result.is_ok());
}

#[test_resources("tests/data/bad/*.json")]
fn verify_bad_examples(resource: &str) {
    println!("verify_bad_examples reading file {}", resource);
    let file_name = PathBuf::from(resource);
    let result = io::read_from_file(&file_name);
    println!("{:#?}", result);
    assert!(result.is_err());

    let expected_error = read_expected_error(&file_name.clone().with_extension("txt"));
    assert_eq!(format!("{:?}", result.err().unwrap()), expected_error);
}

fn read_expected_error(file_name: &PathBuf) -> String {
    match fs::read_to_string(file_name) {
        Ok(s) => s,
        Err(e) => panic!(
            "Could not read expected error from file {:?}, error: {:?}",
            &file_name, e
        ),
    }
}
