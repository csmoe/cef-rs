fn main() {
    let cef_path = std::env::var("CEF_PATH").unwrap();

    build::rerun_if_changed("build.rs");

    let crate_path = build::cargo_manifest_dir();
    let mut builder = bindgen::builder()
        .header("wrapper.h")
        .allowlist_type("cef_.*")
        .allowlist_function("cef_.*")
        .clang_arg(format!("-I{}", cef_path))
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
