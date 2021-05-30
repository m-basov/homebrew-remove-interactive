use super::{Backend, Component, State};
use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct FormulaInfo;

impl Component for FormulaInfo {
    fn render_to(frame: &mut Frame<Backend>, layout: Rect, state: &mut State) {
        let text = if let Some(formula) = state.get_selected_formula() {
            let title = format!("{} v{}", formula.name, formula.installed[0].version);
            let dependentants = state
                .graph
                .resolve_dependants(formula.name.as_str())
                .join(", ");
            let dependencies = formula.dependencies.join(", ");

            vec![
                // Title
                Spans::from(Span::styled(
                    title,
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::UNDERLINED),
                )),
                // Description
                Spans::from(formula.desc.as_str()),
                Spans::from(""),
                // Dependants
                Spans::from(Span::styled(
                    "Dependants",
                    Style::default().add_modifier(Modifier::UNDERLINED),
                )),
                Spans::from(if dependentants.is_empty() {
                    "No".to_string()
                } else {
                    dependentants
                }),
                Spans::from(""),
                // Dependencies
                Spans::from(Span::styled(
                    "Dependencies",
                    Style::default().add_modifier(Modifier::UNDERLINED),
                )),
                Spans::from(if dependencies.is_empty() {
                    "No".to_string()
                } else {
                    dependencies
                }),
            ]
        } else {
            vec![Spans::from("Select a formula...")]
        };

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::default().title("Info").borders(Borders::ALL))
                .wrap(Wrap { trim: true }),
            layout,
        );
    }
}
