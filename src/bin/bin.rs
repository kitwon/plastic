extern crate plastic;
use plastic::exec;
use std::fs::read_to_string;

fn main() {
  let buffer = read_to_string("tests/js/common.js").unwrap();
  dbg!(exec(buffer))
}
