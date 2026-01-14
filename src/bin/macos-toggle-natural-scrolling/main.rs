#![allow(clippy::unit_arg)]

#[cfg(not(target_os = "macos"))]
const CRATE_MACOS_ONLY: () = panic!("Crate works only on MacOS");

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::{env, fs, io, path, str};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error
{
    #[error("{0}")]
    Msg(&'static str),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Environment variable error: {0}")]
    Var(#[from] env::VarError),
}

fn main() -> Result<(), Error>
{
    if let Some(cmd) = env::args().nth(1)
    {
        let path = &std::path::Path::new("/Users")
            .join(env::var("USER").or(env::var("LOGNAME"))?)
            .join("Library/Application Support/xbar/plugins")
            .join("toggle-natural-scrolling.1m.sh");

        if cmd == "install"
        {
            fs::write(path, include_str!("toggle-natural-scrolling.sh"))?;
            make_executable(path)?;
            println!("Installed script to {path:?}\nRefreshing plugins...");
            return refresh_xbar_plugins().map_err(Into::into);
        }
        else if cmd == "uninstall"
        {
            fs::remove_file(path)?;
            println!("Uninstalled script from {path:?}\nRefreshing plugins...");
            return refresh_xbar_plugins().map_err(Into::into);
        }
        else
        {
            let pkg = env!("CARGO_BIN_NAME");
            return Ok(println!("Usage: {} [install | uninstall]", pkg));
        }
    }

    toggle_natural_scrolling()?;

    Ok(println!("Successfully toggled natural scrolling"))
}

fn make_executable(path: &path::Path) -> Result<(), io::Error>
{
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

fn refresh_xbar_plugins() -> Result<(), std::io::Error>
{
    let cmd = "xbar://app.xbarapp.com/refreshAllPlugins";
    Command::new("open").arg(cmd).status().map(drop)
}

fn toggle_natural_scrolling() -> io::Result<()>
{
    // Embed the binary at compile time
    const OBJC_BINARY: &[u8] =
        include_bytes!(env!("TOGGLE_NATURAL_SCROLLING_BIN"));

    // Extract to a temporary location
    let temp_dir = std::env::temp_dir();
    let bin_path = temp_dir.join("ToggleNaturalScrolling");

    if !bin_path.exists()
    {
        // Write the binary
        println!("Writing binary to path: {}", bin_path.display());
        let mut file = fs::File::create(&bin_path)
            .expect("Failed to create temporary binary");
        file.write_all(OBJC_BINARY).expect("Failed to write binary data");

        println!("Making executable at path: {}", bin_path.display());
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&bin_path, perms)?;
    }

    println!("Running ToggleNaturalScrolling at path: {}", bin_path.display());
    std::process::Command::new(&bin_path)
        .status()
        .expect(":: Could not run ToggleNaturalScrolling");

    Ok(())
}
