use anyhow::{Context, Result};
use std::process::Command;

use super::HomebrewInfo;

pub struct HomebrewClient;

impl HomebrewClient {
    pub fn load_info() -> Result<HomebrewInfo> {
        let output = HomebrewClient::run_cmd(&["info", "--json=v2", "--installed"])?;
        HomebrewInfo::parse(&output)
    }

    pub fn remove_formulae(formulae: &[&str]) -> Result<()> {
        HomebrewClient::run_cmd(&[&["remove"], formulae].concat())?;
        Ok(())
    }

    pub fn ensure_exists() -> Result<()> {
        HomebrewClient::run_cmd(&["--version"])?;
        Ok(())
    }

    fn run_cmd(args: &[&str]) -> Result<String> {
        if cfg!(target_os = "windows") {
            println!("args: {:#?}", args);
            Ok(include_str!("../../fixtures/brew_info_all.json").to_string())
        } else {
            let output = Command::new("brew")
                .args(args)
                .output()
                .with_context(|| format!("Cannot execute command: brew {}", args.join(" ")))?;

            Ok(String::from_utf8(output.stdout)?)
        }
    }
}
