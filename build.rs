// only build for windows
#[cfg(target_os = "windows")]
fn main() {
    // only build the resource for release builds
    // as calling rc.exe might be slow
    if std::env::var("PROFILE").unwrap() == "release" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("res//icon.ico");
        if let Err(e) = res.compile() {
            eprint!("{}", e);
            std::process::exit(1);
        }
    }
}

// nothing to do for other operating systems
#[cfg(not(target_os = "windows"))]
fn main() {}
