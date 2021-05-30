use super::state::State;
use anyhow::Result;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum Signal {
    Exit,
    Tick,
}

impl Signal {
    pub fn poll(state: &mut State) -> Result<Signal> {
        if poll(Duration::from_millis(1000))? {
            let event = read()?;

            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        return Ok(Signal::Exit);
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        state.remove_selected_formulae()?;
                        return Ok(Signal::Exit);
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        unimplemented!();
                    }
                    KeyCode::Char(' ') | KeyCode::Char('s') | KeyCode::Char('S') => {
                        state.select_formulae_to_delete();
                    }
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        state.filter_selected();
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        state.select_prev_formula();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        state.select_next_formula();
                    }
                    _ => {}
                }
            }
        }

        Ok(Signal::Tick)
    }

    pub fn should_exit(self) -> bool {
        self == Signal::Exit
    }
}
