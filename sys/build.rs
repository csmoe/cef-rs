fn main() {
    build::rerun_if_changed("build.rs");
    let (os, arch) = (build::cargo_cfg_target_os(), build::cargo_cfg_target_arch());
    let var = format!("CEF_PATH_{os}_{arch}");
    let cef_link_path = std::env::var(&var).map(std::path::PathBuf::from).unwrap();
    match os.as_str() {
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
