extern crate glob;

use std::{
    env, fs,
    ops::Deref,
    path::{Path, PathBuf},
};

fn out_dir() -> PathBuf {
    Path::new(&env::var("OUT_DIR").expect("env")).to_path_buf()
}

fn cleanup() {
    let _ = fs::remove_dir_all(&out_dir());
}

fn compile() {
    let proto_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").expect("env")).join("proto");

    let files = &[
        proto_dir.join("authentication.proto"),
        proto_dir.join("keyexchange.proto"),
        proto_dir.join("mercury.proto"),
        proto_dir.join("metadata.proto"),
        proto_dir.join("playlist4changes.proto"),
        proto_dir.join("playlist4content.proto"),
        proto_dir.join("playlist4issues.proto"),
        proto_dir.join("playlist4meta.proto"),
        proto_dir.join("playlist4ops.proto"),
        proto_dir.join("pubsub.proto"),
        proto_dir.join("spirc.proto"),
    ];

    let slices = files.iter().map(Deref::deref).collect::<Vec<_>>();

    let out_dir = out_dir();
    fs::create_dir(&out_dir).expect("create_dir");

    protobuf_codegen_pure::Codegen::new()
        .out_dir(&out_dir)
        .inputs(&slices)
        .include(&proto_dir)
        .run()
        .expect("Codegen failed.");
}

fn generate_mod_rs() {
    let out_dir = out_dir();

    let mods = glob::glob(&out_dir.join("*.rs").to_string_lossy())
        .expect("glob")
        .filter_map(|p| {
            p.ok()
                .map(|p| format!("pub mod {};", p.file_stem().unwrap().to_string_lossy()))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let mod_rs = out_dir.join("mod.rs");
    fs::write(&mod_rs, format!("// @generated\n{}\n", mods)).expect("write");

    println!("cargo:rustc-env=PROTO_MOD_RS={}", mod_rs.to_string_lossy());
}

fn main() {
    cleanup();
    compile();
    generate_mod_rs();
}
