mod events;
mod formula_info;
mod formulae_list;
mod help_bar;
mod layout;
mod state;

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io;
use tui::{backend::CrosstermBackend, layout::Rect, Frame, Terminal};

use crate::homebrew::{HomebrewFormula, HomebrewGraph};
use events::Signal;
use formula_info::FormulaInfo;
use formulae_list::FormulaeList;
use help_bar::HelpBar;
use layout::Layout;
use state::State;

type Backend = CrosstermBackend<io::Stdout>;

pub struct UI<'a> {
    terminal: Terminal<Backend>,
    state: State<'a>,
}

trait Component {
    fn render_to(frame: &mut Frame<Backend>, layout: Rect, state: &mut State);
}

impl<'a> UI<'a> {
    pub fn run(&mut self) -> Result<()> {
        let terminal = &mut self.terminal;
        let state = &mut self.state;

        loop {
            if Signal::poll(state)?.should_exit() {
                break;
            }

            terminal.draw(|frame| {
                let layout = Layout::new(frame.size());

                HelpBar::render_to(frame, layout.help_bar, state);
                FormulaeList::render_to(frame, layout.formulae_list, state);
                FormulaInfo::render_to(frame, layout.info, state);
            })?;
        }

        self.teardown()
    }

    pub fn init(formulae: &'a [HomebrewFormula], graph: &'a HomebrewGraph) -> Result<UI<'a>> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        stdout.execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(UI {
            terminal,
            state: State::new(formulae, graph),
        })
    }

    fn teardown(&mut self) -> Result<()> {
        disable_raw_mode()?;
        self.terminal.backend_mut().execute(LeaveAlternateScreen)?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
