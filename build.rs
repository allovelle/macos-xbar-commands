use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main()
{
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "macos"
    {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let objc_bin = out_dir.join("ToggleNaturalScrolling");

    // Compile ObjC tool
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
        .arg(&objc_bin)
        .args([
            "src/bin/macos-toggle-natural-scrolling/objc/Retry.m",
            "src/bin/macos-toggle-natural-scrolling/objc/ProcUtil.m",
            "src/bin/macos-toggle-natural-scrolling/objc/UiUtil.m",
            "src/bin/macos-toggle-natural-scrolling/objc/Main.m",
        ])
        .status()
        .expect("failed to invoke clang");

    if !status.success()
    {
        panic!("clang failed building ToggleNaturalScrolling");
    }

    // Tell Rust where to find the binary for embedding
    println!(
        "cargo:rustc-env=TOGGLE_NATURAL_SCROLLING_BIN={}",
        objc_bin.display()
    );

    // Rerun triggers
    println!(
        "cargo:rerun-if-changed=src/bin/macos-toggle-natural-scrolling/objc/Main.m"
    );
    println!(
        "cargo:rerun-if-changed=src/bin/macos-toggle-natural-scrolling/objc/Retry.m"
    );
    println!(
        "cargo:rerun-if-changed=src/bin/macos-toggle-natural-scrolling/objc/ProcUtil.m"
    );
    println!(
        "cargo:rerun-if-changed=src/bin/macos-toggle-natural-scrolling/objc/UiUtil.m"
    );
}
