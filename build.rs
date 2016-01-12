extern crate vergen;

fn main() {
    vergen::vergen(vergen::SHORT_SHA).unwrap();
}

