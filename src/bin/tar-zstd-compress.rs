use std::process::{Command, Stdio};

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

use convenience_utils::{check_dependencies, create_awk_with_pipe};
fn main() {
    check_dependencies(&["tar", "zstd"]);

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
        .stderr(Stdio::piped())
        .arg("-cvO")
        .args(inputs)
        .spawn()
        .expect("Tar failed!");

    let tar_err = tar.stderr.expect("Failed to get tar stderr!");
    let tar_awk = create_awk_with_pipe(tar_err, "tar: ");

    let tar_out = tar.stdout.expect("Failed to get tar stdout!");

    let compression_level_arg = format!("-{compression_level}");
    let mut zstd = Command::new("zstd")
        .stdin(Stdio::from(tar_out))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(["-T0", "--long", &compression_level_arg])
        .args(["-o", &output.to_string_lossy()])
        .spawn()
        .expect("zstd failed!");

    let zstd_out = zstd.stdout.take().expect("Failed to get zstd stdout!");
    let zstd_err = zstd.stderr.take().expect("Failed to get zstd stdout!");
    let zstd_out_awk = create_awk_with_pipe(zstd_out, "zstd: ");
    let zstd_err_awk = create_awk_with_pipe(zstd_err, "zstd: ");

    zstd.wait().expect("Failed waiting!");
}
