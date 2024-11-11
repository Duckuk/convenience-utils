use std::process::Command;

mod flags {
    use std::path::PathBuf;
    xflags::xflags! {
        cmd vp9-compress {
            /// Video file or image sequence to compress
            required input: PathBuf
            
            /// Path to output
            /// Defaults to `input`.new.webm
            optional -o, --output output: PathBuf

            /// CRF level
            /// Defaults to 31
            optional -c, --compression crf_level: i32
            
            /// Additional arg to pass to FFmpeg
            /// Can be specified multiple times, e.g `-a -b:a -a 96k`
            repeated -a, --ffmpeg-arg arg: String
        }
    }
}

fn main() {
    let args = match flags::Vp9Compress::from_env() {
        Ok(flags) => flags,
        Err(err) => err.exit()
    };

    let input = args.input;
    let output = match args.output {
        Some(s) => s,
        None => input.with_extension("new.webm")
    };

    println!("{:?}", args.ffmpeg_arg);

    let crf = args.compression.unwrap_or(31);
    let pass_log = std::env::temp_dir().join("vp9-compress-ffmpeg-pass.log");

    let ffmpeg_args = [
        "-i", &input.to_string_lossy(),
        "-c:v", "libvpx-vp9",
        "-c:a", "libopus",
        "-pix_fmt", "yuv420p",
        "-b:v", "0",
        "-crf", &crf.to_string(),
        "-row-mt", "1",
        "-passlogfile", &pass_log.to_string_lossy()
    ];

    let mut ffmpeg_pass_1 = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .args(args.ffmpeg_arg.iter())
        .args(["-pass", "1", "-an", "-f", "null", "/dev/null"])
        .spawn()
        .expect("ffmpeg pass 1 failed!");

    ffmpeg_pass_1.wait().expect("Failed waiting for pass 1");

    let mut ffmpeg_pass_2 = Command::new("ffmpeg")
        .args(&ffmpeg_args)
        .args(args.ffmpeg_arg.iter())
        .args(["-pass", "2", &output.to_string_lossy()])
        .spawn()
        .expect("ffmpeg pass 2 failed!");

        ffmpeg_pass_2.wait().expect("Failed waiting for pass 2");
}
