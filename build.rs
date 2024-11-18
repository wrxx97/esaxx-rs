use std::env;

#[cfg(feature = "cpp")]
#[cfg(not(target_os = "macos"))]
fn main() {
    let static_crt = env::var("ESAXX_STATIC_CRT")
        .map(|v| v == "1")
        .unwrap_or(false);
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .static_crt(static_crt)
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(feature = "cpp")]
#[cfg(target_os = "macos")]
fn main() {
    let static_crt = env::var("ESAXX_STATIC_CRT")
        .map(|v| v == "1")
        .unwrap_or(false);
    cc::Build::new()
        .cpp(static_crt)
        .flag("-std=c++11")
        .flag("-stdlib=libc++")
        .static_crt(true)
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
