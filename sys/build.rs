fn main() {
    build::rerun_if_changed("build.rs");
    let (os, arch) = (build::cargo_cfg_target_os(), build::cargo_cfg_target_arch());
    let var = format!("CEF_PATH_{os}_{arch}");
    let cef_link_path = std::env::var(&var).map(std::path::PathBuf::from).unwrap();
    build::rustc_link_search_kind("native", &cef_link_path);
    build::rustc_link_lib_kind(
        "static",
        if os == "windows" {
            "libcef_dll_wrapper"
        } else {
            "cef_dll_wrapper"
        },
    );
    // TODO link sandbox
    //build::rustc_link_lib_kind("static", "cef_sandbox");
    match os.as_str() {
        "macos" => {
            build::rustc_link_lib_kind("framework", "AppKit");
        }
        "linux" => {
            build::rustc_link_lib_kind("dylib", "cef");
        }
        "windows" => {
            build::rustc_link_lib_kind("dylib", "libcef");
        }
        os => {
            panic!("unsupported {os}");
        }
    }
}
