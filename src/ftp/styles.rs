use ratatui::style::{self, Color, Modifier, Style};
use style::palette::tailwind;

pub struct TableStyles {
    pub header_style: Style,
    pub selected_row_style: Style,
    pub row_style: Style,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub scrollbar_style: Style,
    pub footer_style: Style,
}

impl TableStyles {
    pub const fn new() -> Self {
        Self {
            header_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::BLUE.c950)
                .add_modifier(Modifier::BOLD),
            selected_row_style: Style::new()
                .add_modifier(Modifier::REVERSED)
                .fg(tailwind::BLUE.c600),
            row_style: Style::new().fg(tailwind::SLATE.c200),
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            scrollbar_style: Style::new()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::REVERSED),
            footer_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::SLATE.c800),
        }
    }
}
