use std::{
    env, fs,
    ops::Deref,
    path::{Path, PathBuf},
};

fn out_dir() -> PathBuf {
    Path::new(&env::var("OUT_DIR").expect("env")).to_path_buf()
}

fn cleanup() {
    let _ = fs::remove_dir_all(out_dir());
}

fn compile() {
    let proto_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").expect("env")).join("proto");

    let files = &[
        proto_dir.join("connect.proto"),
        proto_dir.join("connectivity.proto"),
        proto_dir.join("devices.proto"),
        proto_dir.join("entity_extension_data.proto"),
        proto_dir.join("extended_metadata.proto"),
        proto_dir.join("extension_kind.proto"),
        proto_dir.join("metadata.proto"),
        proto_dir.join("player.proto"),
        proto_dir.join("playlist_annotate3.proto"),
        proto_dir.join("playlist_permission.proto"),
        proto_dir.join("playlist4_external.proto"),
        proto_dir.join("spotify/clienttoken/v0/clienttoken_http.proto"),
        proto_dir.join("storage-resolve.proto"),
        proto_dir.join("user_attributes.proto"),
        // TODO: remove these legacy protobufs when we are on the new API completely
        proto_dir.join("authentication.proto"),
        proto_dir.join("canvaz.proto"),
        proto_dir.join("canvaz-meta.proto"),
        proto_dir.join("explicit_content_pubsub.proto"),
        proto_dir.join("keyexchange.proto"),
        proto_dir.join("mercury.proto"),
        proto_dir.join("pubsub.proto"),
        proto_dir.join("spirc.proto"),
    ];

    let slices = files.iter().map(Deref::deref).collect::<Vec<_>>();

    let out_dir = out_dir();
    fs::create_dir(&out_dir).expect("create_dir");

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(&out_dir)
        .inputs(&slices)
        .include(&proto_dir)
        .run()
        .expect("Codegen failed.");
}

fn main() {
    cleanup();
    compile();
}
