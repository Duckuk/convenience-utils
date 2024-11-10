use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};

mod flags {
    use std::path::PathBuf;
    xflags::xflags! {
        cmd tar-zstd-compress {
            required output: PathBuf
            repeated inputs: PathBuf

            /// zstd compression level
            optional -c, --compression compression_level: i32
        }
    }
}

fn main() {
    let args = match flags::TarZstdCompress::from_env() {
        Ok(flags) => flags,
        Err(err) => err.exit()
    };

    let mut inputs: Vec<String> = Vec::new();
    for path in args.inputs {
        inputs.push("-C".to_string());
        inputs.push(path.canonicalize().unwrap().parent().unwrap().to_string_lossy().into_owned());
        inputs.push(path.file_name().unwrap().to_string_lossy().into_owned());
    }
    println!("{:?}", inputs);

    let output = args.output;
    let compression_level = args.compression.unwrap_or(3);

    let tar = Command::new("tar")
        .stdout(Stdio::piped())
        .arg("-cv")
        .arg("-O")
        .args(inputs)
        .spawn()
        .expect("Tar failed!");

    let tar_out = tar.stdout.expect("Failed to get tar output!");

    let mut zstd = Command::new("zstd")
        .stdin(Stdio::from(tar_out))
        .arg("-T0")
        .arg("--long")
        .arg(format!("-{compression_level}"))
        .arg("-o")
        .arg(output)
        .spawn()
        .expect("zstd failed!");

    zstd.wait().expect("Failed waiting!");

    println!("Hello, world!");
}
