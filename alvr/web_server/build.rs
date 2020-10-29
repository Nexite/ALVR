use alvr_common::data::*;

#[cfg(windows)]
fn main() {
    println!(
        "cargo:rustc-env=SETTINGS_SCHEMA={}",
        serde_json::to_string(&settings_schema(settings_cache_default())).unwrap()
    );
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/alvr.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    println!(
        "cargo:rustc-env=SETTINGS_SCHEMA={}",
        serde_json::to_string(&settings_schema(settings_cache_default())).unwrap()
    );
    let mut res = winres::WindowsResource::new();
    res.set_icon("res/alvr.ico");
    res.compile().unwrap();
}
