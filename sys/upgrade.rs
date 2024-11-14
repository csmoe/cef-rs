#!/usr/bin/env -S RUSTFLAGS=-Copt-level=3 cargo +nightly --config ../.cargo/config.toml -v -Zscript
---cargo
[dependencies]
ureq = "2"
tar = "0"
bzip2 = "0"
serde_json = "1"
cargo_metadata = "0"
sha1_smol = "1"
indicatif = "0"
---

const DOWNLOAD_TEMPLATE: &str =
    "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

const HELP: &str = r#"
Usage:
    on *nix platform: ./upgrade.rs <target>
    on windows platform: cargo +nightly -Zscript upgrade.rs <target> --release
"#;

const TARGETS: &[&str] = &[
    // macos
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    // windows
    "aarch64-pc-windows-msvc",
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    // linux
    "x86_64-unknown-linux-gnu",
    "i686-unknown-linux-gnu",
    "arm-unknown-linux-gnueabi",
    "aarch64-unknown-linux-gnu",
];

const URL: &str = "https://cef-builds.spotifycdn.com";

fn main() {
    use std::ops::Deref;
    let args = std::env::args().collect::<Vec<_>>();
    let args = args.iter().map(|s| s.deref()).collect::<Vec<_>>();

    match args.as_slice() {
        [_, "all", ..] => {
            let threads = TARGETS
                .iter()
                .map(|target| {
                    std::thread::spawn(move || {
                        let path = download_prebuilt_cef(target);
                        (target, path)
                    })
                })
                .collect::<Vec<_>>();
            for t in threads {
                if let Ok((target, path)) = t.join() {
                    bindgen(&target, &path);
                }
            }
        }
        [_, target @ _, "--download", ..] => {
            if TARGETS.contains(target) {
                download_prebuilt_cef(target);
            } else {
                eprintln!("expected targets: {TARGETS:?}");
                std::process::exit(1);
            }
        }
        [_, target @ _, ..] => {
            if TARGETS.contains(target) {
                let path = download_prebuilt_cef(target);
                bindgen(&target, &path);
            } else {
                eprintln!("expected targets: {TARGETS:?}");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("{HELP}");
            std::process::exit(1);
        }
    }
}

fn download_prebuilt_cef(target: &str) -> std::path::PathBuf {
    let cef_path = std::env::var("CEF_PATH")
        .map(std::path::PathBuf::from)
        .unwrap();
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .manifest_path("./Cargo.toml")
        .exec()
        .unwrap();
    let cef_version = metadata
        .workspace_packages()
        .iter()
        .find_map(|d| {
            if d.name == "libcef-sys" {
                d.metadata.pointer("/cef_version").unwrap().as_str()
            } else {
                None
            }
        })
        .unwrap();
    println!("cef: trying to download {target} {cef_version}");

    let url = std::env::var("CEF_URL").unwrap_or(URL.into());
    let platform = match target {
        "aarch64-apple-darwin" => "macosarm64",
        "x86_64-apple-darwin" => "macosx64",
        "i686-pc-windows-msvc" => "windows32",
        "x86_64-pc-windows-msvc" => "windows64",
        "aarch64-pc-windows-msvc" => "windowsarm64",
        "x86_64-unknown-linux-gnu" => "linux64",
        "arm-unknown-linux-gnueabi" => "linuxarm",
        "aarch64-unknown-linux-gnu" => "linuxarm64",
        v @ _ => panic!("unsupported {v:?}"),
    };
    let index_resp = ureq::get(&format!("{url}/index.json"))
        .call()
        .unwrap()
        .into_reader();
    let index: serde_json::Value = serde_json::from_reader(index_resp).unwrap();
    let versions = index
        .pointer(&format!("/{platform}/versions"))
        .unwrap()
        .as_array()
        .unwrap();
    let (file, sha) = versions
        .iter()
        .find_map(|v| {
            if v["cef_version"].as_str() == Some(&cef_version)
                && v["channel"].as_str() == Some("stable")
            {
                v["files"].as_array().unwrap().iter().find_map(|f| {
                    if f["type"].as_str() == Some("minimal") {
                        f["name"].as_str().zip(f["sha1"].as_str())
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .unwrap();
    let cef_url = format!("{url}/{file}");

    println!("cef: downloading url {cef_url}");
    let download = &cef_path.parent().unwrap();
    std::fs::create_dir_all(&download).unwrap();
    let download_file = download.join(&file);
    if !download_file.exists()
        || calculate_file_sha1(std::io::BufReader::new(
            std::fs::File::open(&download_file).unwrap(),
        )) != sha
    {
        let mut f = std::fs::File::create(&download_file).unwrap();
        let resp = ureq::get(&cef_url).call().unwrap();
        let length: u64 = resp.header("Content-Length").unwrap().parse().unwrap();
        let bar = indicatif::ProgressBar::new(!0);
        bar.set_message("Downloading");
        bar.set_style(
            indicatif::ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
                .unwrap()
                .progress_chars("##-"),
        );
        bar.set_length(length);
        std::io::copy(&mut bar.wrap_read(resp.into_reader()), &mut f).unwrap();
    }
    assert_eq!(
        calculate_file_sha1(std::io::BufReader::new(
            std::fs::File::open(&download_file).unwrap()
        )),
        sha,
        "sha1sum mismatch"
    );
    println!("cef: downloaded into {}", download_file.display());

    let decoder = bzip2::bufread::BzDecoder::new(std::io::BufReader::new(
        std::fs::File::open(download_file).unwrap(),
    ));
    tar::Archive::new(decoder).unpack(&download).unwrap();

    let from = download.join(file.strip_suffix(".tar.bz2").unwrap());
    if cef_path.exists() {
        std::fs::remove_dir_all(&cef_path).unwrap();
    }
    std::fs::rename(from, &cef_path).unwrap();
    println!("cef: extracted into {:?}", cef_path);
    cef_path
}

fn calculate_file_sha1(mut reader: std::io::BufReader<std::fs::File>) -> String {
    use std::io::Read;
    let mut sha1 = sha1_smol::Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        sha1.update(&buffer[..count]);
    }

    sha1.digest().to_string()
}

fn bindgen(target: &str, cef_path: &std::path::Path) {
    use std::io::Write;
    let mut gen = std::process::Command::new("bindgen");
    let binding = target.replace('-', "_") + ".rs";
    gen.args([
        "wrapper.h",
        "-o",
        &format!("src/bindings/{binding}"),
        "--default-enum-style=rust_non_exhaustive",
        "--allowlist-type",
        "cef_.*",
        "--allowlist-function",
        "cef_.*",
        "--bitfield-enum",
        ".*_mask_t",
        // clang args
        "--",
        &format!("-I{}", cef_path.display()),
        &format!("--target={target}"),
    ]);
    if target.contains("apple") {
        let output = std::process::Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .unwrap();
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        gen.arg(format!("--sysroot={path}"));
    }

    println!("cef: bindgen cmd={gen:?}");
    let output = gen.output().unwrap();
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}