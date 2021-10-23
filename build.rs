use std::env;

fn main() {
    if env::var("PROFILE").unwrap() == "release" {
        if let Ok(_) = env::var("CARGO_CFG_WINDOWS") {
            let mut res = winres::WindowsResource::new();
            if let Ok(host) = env::var("HOST") {
                if host.contains("linux") {
                    res.set_toolkit_path("/usr/bin")
                        .set_windres_path("x86_64-w64-mingw32-windres");
                }
            }
            res.set_icon("res/copy-translator.ico").set_language(0x04);
            res.compile().unwrap();
        }
    }
}
