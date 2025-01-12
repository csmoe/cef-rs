#!/usr/bin/env -S RUSTFLAGS=-Copt-level=3 cargo +nightly --config ../.cargo/config.toml -v -Zscript
---cargo
[dependencies]
ureq = { version = "2", features = [ "json" ] }
tar = "0"
toml = { version = "0.8", features = [ "parse" ] }
bzip2 = "0"
serde_json = "1"
sha1_smol = "1"
indicatif = "0"
---

use std::env;
use std::ops::Deref;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

const DOWNLOAD_TEMPLATE: &str = "{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
const HELP: &str = r#"
Usage:
    on *nix platform: ./upgrade.rs <target> [ --download | --bindgen ]
    on windows platform: cargo +nightly --config ../.cargo/.config.toml -Zscript upgrade.rs <target> [ --download | --bindgen ]
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
    let args = env::args().collect::<Vec<_>>();
    let args = args.iter().map(|s| s.deref()).collect::<Vec<_>>();

    match &args[1..] {
        [target, "--download"] if TARGETS.contains(target) => {
            let (os, arch) = target_to_os_arch(target);
            let cef_path = env::var(&format!("CEF_PATH_{os}_{arch}"))
                .map(PathBuf::from)
                .expect("CEF_PATH environment variable not set");
            let archive_dir = download_prebuilt_cef(target, &cef_path);
            build_cef_dll_wrapper(&cef_path, &archive_dir, os);
        }
        [target, "--bindgen"] if TARGETS.contains(target) => {
            let (os, arch) = target_to_os_arch(target);
            let cef_path = env::var(&format!("CEF_PATH_{os}_{arch}"))
                .map(PathBuf::from)
                .expect("CEF_PATH environment variable not set");
            bindgen(target, &cef_path);
        }
        _ => {
            eprintln!("{HELP}");
            exit(1);
        }
    }
}

fn download_prebuilt_cef(target: &str, cef_path: &Path) -> PathBuf {
    let metadata: toml::Table =
        toml::from_str(&fs::read_to_string("./Cargo.toml").unwrap()).unwrap();
    let cef_version = metadata["package"]["metadata"]["cef_version"]
        .as_str()
        .unwrap();
    println!("cef: trying to download {target} {cef_version}");

    let url = env::var("CEF_URL").unwrap_or_else(|_| URL.to_string());
    let platform = target_to_cef_target(target);
    let index: serde_json::Value = ureq::get(&format!("{url}/index.json"))
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    let (file, sha) = index[platform]["versions"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|v| {
            if v["cef_version"].as_str() == Some(cef_version)
                && v["channel"].as_str() == Some("stable")
            {
                v["files"].as_array().unwrap().iter().find_map(|f| {
                    if f["type"].as_str() == Some("minimal") {
                        Some((f["name"].as_str().unwrap(), f["sha1"].as_str().unwrap()))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .expect("Matching CEF version not found");

    let cef_url = format!("{url}/{file}");
    println!("cef: downloading url {cef_url}");

    let download = cef_path.parent().unwrap();
    fs::create_dir_all(download).unwrap();
    let download_file = download.join(file);

    if !download_file.exists() || calculate_file_sha1(&download_file) != sha {
        download_file_with_progress(&cef_url, &download_file);
    }

    assert_eq!(calculate_file_sha1(&download_file), sha, "sha1sum mismatch");
    println!("cef: downloaded into {}", download_file.display());

    extract_and_organize(download, file, &download_file, target, cef_path)
}

fn download_file_with_progress(url: &str, path: &Path) {
    let mut file = File::create(path).unwrap();
    let resp = ureq::get(url).call().unwrap();
    let length: u64 = resp.header("Content-Length").unwrap().parse().unwrap();

    let bar = indicatif::ProgressBar::new(length);
    bar.set_style(
        indicatif::ProgressStyle::with_template(DOWNLOAD_TEMPLATE)
            .unwrap()
            .progress_chars("##-"),
    );
    bar.set_message("Downloading");

    std::io::copy(&mut bar.wrap_read(resp.into_reader()), &mut file).unwrap();
}

fn extract_and_organize(
    download_path: &Path,
    file_name: &str,
    download_file: &Path,
    target: &str,
    cef_path: &Path,
) -> PathBuf {
    let decoder =
        bzip2::bufread::BzDecoder::new(std::io::BufReader::new(File::open(download_file).unwrap()));
    tar::Archive::new(decoder).unpack(download_path).unwrap();

    let extracted_dir = download_path.join(file_name.strip_suffix(".tar.bz2").unwrap());
    let (os, arch) = target_to_os_arch(target);
    let archive_dir = download_path.join(format!("archive_{os}_{arch}"));

    if archive_dir.exists() {
        fs::remove_dir_all(&archive_dir).unwrap();
    }
    fs::rename(extracted_dir, &archive_dir).unwrap();

    if cef_path.exists() {
        fs::remove_dir_all(cef_path).unwrap();
    }
    fs::rename(archive_dir.join("Release"), cef_path).unwrap();

    if target.contains("windows") {
        copy_directory(&archive_dir.join("Resources"), cef_path);
    }
    copy_directory(&archive_dir.join("include"), &cef_path.join("include"));

    println!("cef: extracted into {:?}", cef_path);
    archive_dir
}

fn calculate_file_sha1(path: &Path) -> String {
    use std::io::Read;
    let mut file = std::io::BufReader::new(File::open(path).unwrap());
    let mut sha1 = sha1_smol::Sha1::new();
    let mut buffer = [0; 8192];

    loop {
        let count = file.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        sha1.update(&buffer[..count]);
    }

    sha1.digest().to_string()
}

fn bindgen(target: &str, cef_path: &Path) {
    let binding = format!("src/bindings/{}.rs", target.replace('-', "_"));
    let mut cmd = Command::new("bindgen");
    cmd.args([
        "wrapper.h",
        "-o",
        &binding,
        "--default-enum-style=rust_non_exhaustive",
        "--allowlist-type",
        "cef_.*",
        "--allowlist-function",
        "cef_.*",
        "--bitfield-enum",
        ".*_mask_t",
        "--with-derive-custom-struct",
        r#".*=crate::FfiRc"#,
        "--",
        &format!("-I{}", cef_path.display()),
        &format!("--target={target}"),
    ]);

    if target.contains("apple") {
        let sdk_path = Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .unwrap()
            .stdout;
        cmd.arg(format!(
            "--sysroot={}",
            String::from_utf8_lossy(&sdk_path).trim()
        ));
    }

    println!("cef: bindgen cmd={:?}", cmd);
    let output = cmd.output().unwrap();
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}

fn build_cef_dll_wrapper(cef_path: &Path, archive_dir: &Path, os: &str) {
    let lib_name = format!(
        "libcef_dll_wrapper.{}",
        if os == "windows" { "lib" } else { "a" }
    );
    if cef_path.join(&lib_name).exists() {
        println!("cef: {lib_name} already exists, skip building");
        return;
    }

    let build_dir = archive_dir.join("build");
    fs::create_dir_all(&build_dir).unwrap();

    let cmake_output = Command::new("cmake")
        .current_dir(&build_dir)
        .args([
            "-G",
            "Ninja",
            "-DCMAKE_OBJECT_PATH_MAX=500",
            "-DCMAKE_BUILD_TYPE=RelWithDebInfo",
            "..",
        ])
        .output()
        .unwrap();

    print_command_output(&cmake_output);

    let ninja_output = Command::new("ninja")
        .current_dir(&build_dir)
        .arg("libcef_dll_wrapper")
        .output()
        .unwrap();

    print_command_output(&ninja_output);

    fs::copy(
        build_dir.join("libcef_dll_wrapper").join(&lib_name),
        cef_path.join(&lib_name),
    )
    .unwrap();
}

fn print_command_output(output: &std::process::Output) {
    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    assert!(output.status.success());
}

fn copy_directory(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry.file_type().unwrap().is_dir() {
            copy_directory(&src_path, &dst_path);
        } else {
            fs::copy(&src_path, &dst_path).unwrap();
        }
    }
}

fn target_to_cef_target(target: &str) -> &str {
    match target {
        "aarch64-apple-darwin" => "macosarm64",
        "x86_64-apple-darwin" => "macosx64",
        "i686-pc-windows-msvc" => "windows32",
        "x86_64-pc-windows-msvc" => "windows64",
        "aarch64-pc-windows-msvc" => "windowsarm64",
        "x86_64-unknown-linux-gnu" => "linux64",
        "arm-unknown-linux-gnueabi" => "linuxarm",
        "aarch64-unknown-linux-gnu" => "linuxarm64",
        v => panic!("unsupported {v:?}"),
    }
}

fn target_to_os_arch(target: &str) -> (&str, &str) {
    match target {
        "aarch64-apple-darwin" => ("macos", "aarch64"),
        "x86_64-apple-darwin" => ("macos", "x86_64"),
        "i686-pc-windows-msvc" => ("windows", "x86"),
        "x86_64-pc-windows-msvc" => ("windows", "x86_64"),
        "aarch64-pc-windows-msvc" => ("windows", "aarch64"),
        "x86_64-unknown-linux-gnu" => ("linux", "x86_64"),
        "arm-unknown-linux-gnueabi" => ("linux", "arm"),
        "aarch64-unknown-linux-gnu" => ("linux", "aarch64"),
        v => panic!("unsupported {v:?}"),
    }
}
