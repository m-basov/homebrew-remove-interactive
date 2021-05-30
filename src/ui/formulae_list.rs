use super::{Backend, Component, Frame, State};
use tui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub struct FormulaeList;

impl Component for FormulaeList {
    fn render_to(frame: &mut Frame<Backend>, layout: Rect, state: &mut State) {
        let packages_list = List::new(
            state
                .formulae
                .iter()
                .map(|formula| {
                    let symbol = if state.is_formula_selected(&formula.name) {
                        "✓"
                    } else {
                        "-"
                    };
                    ListItem::new(format!("{} {}", symbol, formula.name))
                })
                .collect::<Vec<_>>(),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::Rgb(255, 255, 255)),
        )
        .block(
            Block::default()
                .title(format!("Formulae({}) [↑↓]", state.formulae.len()))
                .borders(Borders::ALL),
        );

        frame.render_stateful_widget(packages_list, layout, &mut state.selected_formula);
    }
}
