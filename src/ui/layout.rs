use tui::layout::{Constraint, Direction, Layout as TUILayout, Rect};

pub struct Layout {
    pub help_bar: Rect,
    pub formulae_list: Rect,
    pub info: Rect,
}

impl Layout {
    pub fn new(size: Rect) -> Layout {
        let main_chunks = TUILayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
            .split(size);

        let content_chunks = TUILayout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(main_chunks[0]);

        Layout {
            help_bar: main_chunks[1],
            formulae_list: content_chunks[0],
            info: content_chunks[1],
        }
    }
}
