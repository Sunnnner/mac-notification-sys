extern crate cc;

const DEPLOYMENT_TARGET_VAR: &str = "MACOSX_DEPLOYMENT_TARGET";

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        let min_version = std::env::var(DEPLOYMENT_TARGET_VAR).unwrap_or_else(|_| {
            String::from(match std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
                "x86_64" => "10.8",  // NSUserNotificationCenter first showed up here.
                "aarch64" => "11.0", // Apple Silicon started here.
                arch => panic!("unknown arch: {}", arch),
            })
        });

        cc::Build::new()
            .file("objc/notify.m")
            .flag("-fmodules")
            .flag("-Wno-deprecated-declarations")
            // `cc` doesn't try to pick up on this automatically, but `clang` needs it to
            // generate a "correct" Objective-C symbol table which better matches XCode.
            // See https://github.com/h4llow3En/mac-notification-sys/issues/45.
            .flag(format!("-mmacos-version-min={}", min_version))
            .compile("notify");

        println!("cargo:rerun-if-env-changed={}", DEPLOYMENT_TARGET_VAR);
        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-changed=objc");
    }
}
