use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main()
{
    // Only run on macOS
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "macos"
    {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bin_path = out_dir.join("ToggleNaturalScrolling");

    let status = Command::new("clang")
        .args([
            "-Wall",
            "-O2",
            "-framework",
            "Foundation",
            "-framework",
            "AppKit",
            "-framework",
            "ApplicationServices",
            "-o",
        ])
        .arg(&bin_path)
        .args([
            "objc/Retry.m",
            "objc/ProcUtil.m",
            "objc/UiUtil.m",
            "objc/Main.m",
        ])
        .status()
        .expect("failed to invoke clang");

    if !status.success()
    {
        panic!("clang failed building ToggleNaturalScrolling");
    }

    // Re-run build.rs if any ObjC file changes
    println!("cargo:rerun-if-changed=objc/Main.m");
    println!("cargo:rerun-if-changed=objc/Retry.m");
    println!("cargo:rerun-if-changed=objc/ProcUtil.m");
    println!("cargo:rerun-if-changed=objc/UiUtil.m");
    println!("cargo:rerun-if-changed=objc/Info.plist");

    // Make the binary available to Rust code (optional)
    println!(
        "cargo:rustc-env=TOGGLE_NATURAL_SCROLLING_BIN={}",
        bin_path.display()
    );
}
