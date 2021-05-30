use super::{Backend, Component, State};
use tui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HelpBar;

const MENU_ITEMS: [&str; 4] = ["Select", "Filter", "Delete", "Quit"];

impl Component for HelpBar {
    fn render_to(frame: &mut Frame<Backend>, layout: Rect, _state: &mut State) {
        let spans = Spans::from(
            MENU_ITEMS
                .iter()
                .enumerate()
                .flat_map(|(idx, item)| {
                    let (first, rest) = item.split_at(1);

                    let mut menu_items = Vec::with_capacity(3);

                    if idx > 0 {
                        menu_items.push(Span::from(" "))
                    }

                    menu_items.push(Span::styled(
                        first,
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    ));
                    menu_items.push(Span::from(rest));

                    menu_items
                })
                .collect::<Vec<_>>(),
        );

        let help_bar =
            Paragraph::new(spans).block(Block::default().title("Help").borders(Borders::ALL));

        frame.render_widget(help_bar, layout)
    }
}
