#![allow(clippy::unit_arg)]

#[cfg(not(target_os = "macos"))]
const CRATE_MACOS_ONLY: () = panic!("Crate works only on MacOS");

use clap::{Parser, Subcommand};
use std::os::unix::fs::PermissionsExt;
use std::process::{self, Command};
use std::{env, fs, io, path, str};
use thiserror::Error;

#[derive(Parser)]
#[command(name = "macos-toggle-dark-mode")]
#[command(about = "Tiny CLI for setting light/auto/dark mode on MacOS")]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands
{
    Install,
    Uninstall,
    LightMode,
    DarkMode,
}

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
    let cli = Cli::parse();
    let path = &std::path::Path::new("/Users")
        .join(env::var("USER").or(env::var("LOGNAME"))?)
        .join("Library/Application Support/xbar/plugins")
        .join("toggle-dark-mode.1m.sh");

    let dark_mode_enable = match cli.command
    {
        Commands::Install =>
        {
            fs::write(path, include_str!("toggle-dark-mode.sh"))?;
            make_executable(path)?;
            println!("Installed script to {path:?}\nRefreshing plugins...");
            return refresh_xbar_plugins().map_err(Into::into);
        }
        Commands::Uninstall =>
        {
            fs::remove_file(path)?;
            println!("Uninstalled script\nRefreshing plugins...");
            return refresh_xbar_plugins().map_err(Into::into);
        }
        Commands::LightMode => "false",
        Commands::DarkMode => "true",
    };

    let command = &[
        "osascript",
        "-e",
        &format!(
            "tell application \"System Events\" to tell appearance preferences to set dark mode to {}",
            dark_mode_enable
        ),
    ];
    run_command(command)?;

    Ok(println!("Successfully altered theme"))
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

fn run_command(cli: &[&str]) -> Result<process::Output, io::Error>
{
    Command::new(cli[0]).args(&cli[1 ..]).output()
}
