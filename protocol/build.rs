extern crate protobuf_codegen; // Does the business
extern crate protobuf_codegen_pure; // Helper function

use std::fs::{read_to_string, write};
use std::path::Path;

use protobuf_codegen_pure::parse_and_typecheck;
use protobuf_codegen_pure::Customize;

fn main() {
    let customizations = Customize {
        ..Default::default()
    };

    let lib_str = read_to_string("src/lib.rs").unwrap();

    // Iterate over the desired module names.
    for line in lib_str.lines() {
        if !line.starts_with("pub mod ") && !line.starts_with("mod ") {
            continue;
        }
        let len = line.len();

        let name;
        if line.starts_with("pub mod ") {
            name = &line[8..len - 1]; // Remove keywords and semi-colon
        } else {
            name = &line[4..len - 1]; // Remove keywords and semi-colon
        }

        // Build the paths to relevant files.
        let src = &format!("proto/{}.proto", name);
        let dest = &format!("src/{}.rs", name);

        // Get the contents of the existing generated file.
        let mut existing = "".to_string();
        if Path::new(dest).exists() {
            // Removing CRLF line endings if present.
            existing = read_to_string(dest).unwrap().replace("\r\n", "\n");
        }

        println!("Regenerating {} from {}", dest, src);

        // Parse the proto files as the protobuf-codegen-pure crate does.
        let p = parse_and_typecheck(&["proto"], &[src]).expect("protoc");
        // But generate them with the protobuf-codegen crate directly.
        // Then we can keep the result in-memory.
        let result = protobuf_codegen::gen(&p.file_descriptors, &p.relative_paths, &customizations);
        // Protoc result as a byte array.
        let new = &result.first().unwrap().content;
        // Convert to utf8 to compare with existing.
        let new = std::str::from_utf8(&new).unwrap();
        // Save newly generated file if changed.
        if new != existing {
            write(dest, &new).unwrap();
        }
    }
}
