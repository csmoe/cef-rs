fn main() {
    let cef_path = std::env::var("CEF_PATH")
        .map(std::path::PathBuf::from)
        .unwrap();
    let cef_link_path = cef_path;
    build::rerun_if_changed("build.rs");

    match build::cargo_cfg_target_os().as_str() {
        "macos" => {
            build::rustc_link_search_kind("framework", cef_link_path);
            build::rustc_link_lib_kind("framework", "Chromium Embedded Framework");
        }
        "linux" => {
            build::rustc_link_lib_kind("dylib", "cef");
            build::rustc_link_search_kind("native", cef_link_path);
        }
        "windows" => {
            build::rustc_link_lib_kind("dylib", "libcef");
            build::rustc_link_search_kind("native", cef_link_path);
        }
        os => {
            panic!("unsupported {os}");
        }
    }
}
