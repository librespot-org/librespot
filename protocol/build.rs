use std::env;
use std::process::{Command, Stdio};
use std::path::{Path,PathBuf};

#[derive(Debug)]
enum ProtobufError {
    IoError(::std::io::Error),
    Other
}

impl std::convert::From<::std::io::Error> for ProtobufError {
    fn from(e: ::std::io::Error) -> ProtobufError {
        ProtobufError::IoError(e)
    }
}

fn compile(prefix : &Path, files : &[&Path]) -> Result<(),ProtobufError>{
    let mut c = Command::new("protoc");
    c.arg("--rust_out").arg(env::var("OUT_DIR").unwrap())
        .arg("--proto_path").arg(prefix.to_str().unwrap());

    for f in files.iter() {
        c.arg(f.to_str().unwrap());
    }

    //c.stdout(Stdio::inherit());
    c.stderr(Stdio::inherit());

    let mut p = try!(c.spawn());
    let r = try!(p.wait());
    return match r.success() {
        true => Ok(()),
        false => Err(ProtobufError::Other),
    };
}

fn main() {
    let root = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto = root.join("proto");

    compile(&proto, &[
            &proto.join("keyexchange.proto"),
            &proto.join("authentication.proto"),
            &proto.join("mercury.proto"),
            &proto.join("metadata.proto"),
            &proto.join("playlist4changes.proto"),
            &proto.join("playlist4content.proto"),
            &proto.join("playlist4issues.proto"),
            &proto.join("playlist4meta.proto"),
            &proto.join("playlist4ops.proto"),
            &proto.join("playlist4service.proto"),
    ]).unwrap();
}

