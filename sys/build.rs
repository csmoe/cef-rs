#[cfg(not(feature = "dox"))]
fn main() -> Result<(), String> {
    let path = std::env::var("FLATPAK")
        .map(|_| String::from("/usr/lib"))
        .or_else(|_| std::env::var("CEF_PATH"))
        .or_else(|_| {
            std::env::var("HOME").map(|mut val| {
                val.push_str("/.local/share/cef");
                val
            })
        })
        .map_err(|e| format!("Couldn't get the path of shared library: {e}"))?;
    
    match std::env::var("CARGO_CFG_TARGET_OS") {
        Ok(os) if os == "windows" => {
            println!("cargo:rustc-link-lib=libcef");
        }
        _ => {
            println!("cargo:rustc-link-lib=cef");
        }
    }

    println!("cargo:rustc-link-search={path}");
    Ok(())
}

#[cfg(feature = "dox")]
fn main() {}
