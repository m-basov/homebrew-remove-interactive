mod homebrew;
mod ui;

use anyhow::Result;
use atty::Stream;
use homebrew::{FormulaeMap, HomebrewClient, HomebrewDependencyGraph};
use std::process;
use ui::UI;

fn main() -> Result<()> {
    if !atty::is(Stream::Stdout) {
        eprintln!("This is an interactive CLI tool. Non-TTY outputs are not supported.");
        process::exit(exitcode::IOERR);
    }

    if let Err(err) = HomebrewClient::ensure_exists() {
        eprintln!("brew command is not available:\n=> {}.", err);
        process::exit(exitcode::IOERR);
    }

    let info = HomebrewClient::load_info()?;
    let formulae_map = FormulaeMap::build(&info.formulae);
    let graph = HomebrewDependencyGraph::build(&formulae_map, &info.formulae);

    let mut ui = UI::init(&info.formulae, &graph)?;
    ui.run()?;

    Ok(())
}
