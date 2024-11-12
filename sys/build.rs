fn main() {
    let cef_path = if let Ok(p) = std::env::var("CEF_PATH").map(std::path::PathBuf::from) {
        p
    } else {
        download_prebuilt_cef()
    };

    build::rerun_if_changed("build.rs");

    let crate_path = build::cargo_manifest_dir();
    let mut builder = bindgen::builder()
        .header("wrapper.h")
        .allowlist_type("cef_.*")
        .allowlist_function("cef_.*")
        .clang_arg(format!("-I{}", cef_path.display()))
        .clang_arg(format!("-I{}", crate_path.display()))
        .bitfield_enum(".*_mask_t")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        });

    match build::cargo_cfg_target_os().as_str() {
        "macos" => {
            let output = std::process::Command::new("xcrun")
                .args(["--sdk", "macosx", "--show-sdk-path"])
                .output()
                .unwrap();
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            builder = builder.clang_arg(format!("--sysroot={path}"));
            build::rustc_link_search_kind(
                "framework",
                std::path::PathBuf::from(cef_path).join("Release"),
            );
            build::rustc_link_lib_kind("framework", "Chromium Embedded Framework");
        }
        "linux" => {
            build::rustc_link_lib_kind("dylib", "cef");
            build::rustc_link_search_kind("native", cef_path);
        }
        "windows" => {
            build::rustc_link_lib_kind("dylib", "libcef");
            build::rustc_link_search_kind("native", cef_path);
        }
        os @ _ => {
            panic!("unsupported {os}");
        }
    }

    let bindings = builder.generate().unwrap();
    bindings
        .write_to_file(build::out_dir().join("bindings.rs"))
        .unwrap();
}

fn download_prebuilt_cef() -> std::path::PathBuf {
    const URL: &str = "https://cef-builds.spotifycdn.com";
    let url = std::env::var("CEF_PREBUILT_DOWNLOAD_URL").unwrap_or(URL.into());
    let metadata = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .manifest_path("./Cargo.toml")
        .exec()
        .unwrap();
    let (cef_version, chromium_version) = metadata
        .workspace_packages()
        .iter()
        .find_map(|d| {
            if d.name == "libcef-sys" {
                d.metadata
                    .pointer("/cef/version")
                    .and_then(|v| v.as_str())
                    .zip(
                        d.metadata
                            .pointer("/cef/chromium_version")
                            .and_then(|v| v.as_str()),
                    )
            } else {
                None
            }
        })
        .unwrap();
    let cef_version = format!("{cef_version}+chromium-{chromium_version}");

    let platform = match (
        build::cargo_cfg_target_os().as_str(),
        build::cargo_cfg_target_arch().as_str(),
    ) {
        ("macos", "aarch64") => "macosarm64",
        ("macos", "x86_64") => "macosx64",
        ("windows", "i686") => "windows32",
        ("windows", "x86_64") => "windows64",
        ("windows", "aarch64") => "windowsarm64",
        ("linux", "x86_64") => "linux64",
        ("linux", "i686") => "linux32",
        ("linux", "arm") => "linuxarm",
        ("linux", "aarch64") => "linuxarm64",
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

    println!("cef: downloading {cef_url}");
    let download = &build::out_dir().join(&file);
    if !download.exists() {
        let mut f = std::fs::File::create(download).unwrap();
        let resp = ureq::get(&cef_url).call().unwrap();
        std::io::copy(&mut resp.into_reader(), &mut f).unwrap();
    }
    assert_eq!(
        calculate_file_sha1(std::io::BufReader::new(
            std::fs::File::open(download).unwrap()
        )),
        sha,
        "sha1sum mismatch"
    );
    println!("cef: downloaded into {}", download.display());

    let output = build::out_dir().join("cef");
    let decoder = bzip2::bufread::BzDecoder::new(std::io::BufReader::new(
        std::fs::File::open(download).unwrap(),
    ));
    tar::Archive::new(decoder).unpack(&output).unwrap();
    println!("cef: extracted into {:?}", output);

    output.join(file.strip_suffix(".tar.bz2").unwrap())
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
