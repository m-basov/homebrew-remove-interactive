mod homebrew;
mod ui;

use anyhow::Result;
use homebrew::{FormulaeMap, HomebrewClient, HomebrewDependencyGraph};
use ui::UI;

fn main() -> Result<()> {
    let info = HomebrewClient::load_info()?;
    let formulae_map = FormulaeMap::build(&info.formulae);
    let graph = HomebrewDependencyGraph::build(&formulae_map, &info.formulae);

    let mut ui = UI::init(&info.formulae, &graph)?;
    ui.run()?;

    Ok(())
}
