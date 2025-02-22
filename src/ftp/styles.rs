use ratatui::style::{self, Color, Modifier, Style};
use style::palette::tailwind;

pub struct TableStyles {
    pub header_style: Style,
    pub selected_row_style: Style,
    pub row_style: Style,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub scrollbar_style: Style,
}

impl TableStyles {
    pub const fn new() -> Self {
        Self {
            header_style: Style::new()
                .fg(tailwind::GRAY.c300)
                .bg(tailwind::SLATE.c800)
                .add_modifier(Modifier::BOLD),
            selected_row_style: Style::new().fg(tailwind::GRAY.c300).bg(tailwind::SKY.c950),
            row_style: Style::new().fg(tailwind::GRAY.c200),
            normal_row_color: tailwind::SLATE.c700,
            alt_row_color: tailwind::SLATE.c600,
            scrollbar_style: Style::new()
                .fg(tailwind::GRAY.c300)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::REVERSED),
        }
    }
}
