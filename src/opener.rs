use crate::{config::Config, store::EntryType};
use std::process::Command;

pub fn open(
    target: &str,
    entry_type: &EntryType,
    config: &Config,
    override_cmd: Option<&str>,
) -> std::io::Result<()> {
    let cmd = override_cmd.unwrap_or_else(|| match entry_type {
        EntryType::Url => &config.opener.url,
        EntryType::File => &config.opener.file,
        EntryType::App => &config.opener.app,
        EntryType::Path => "default",
    });

    if cmd == "default" {
        open_default(target)
    } else {
        Command::new(cmd).arg(target).spawn()?.wait()?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn open_default(target: &str) -> std::io::Result<()> {
    Command::new("cmd")
        .args(["/C", "start", "", target])
        .spawn()?
        .wait()?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn open_default(target: &str) -> std::io::Result<()> {
    Command::new("open").arg(target).spawn()?.wait()?;
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn open_default(target: &str) -> std::io::Result<()> {
    if is_wsl() {
        if let Ok(mut child) = Command::new("wslview").arg(target).spawn() {
            child.wait()?;
            return Ok(());
        }
        Command::new("cmd.exe")
            .args(["/c", "start", "", target])
            .spawn()?
            .wait()?;
    } else {
        Command::new("xdg-open").arg(target).spawn()?.wait()?;
    }
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn is_wsl() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|v| v.to_ascii_lowercase().contains("microsoft"))
        .unwrap_or(false)
}
