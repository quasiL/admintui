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

pub struct EditWindowStyles {
    pub window_style: Style,
    pub window_border_style: Style,
    pub title_style: Style,
    pub title_border_style: Style,
    pub unselected_input_border_style: Style,
    pub selected_input_border_style: Style,
    pub footer_style: Style,
    pub valid_input_style: Style,
    pub valid_cursor_style: Style,
    pub invalid_input_style: Style,
    pub invalid_cursor_style: Style,
    pub cursor_style: Style,
}

impl EditWindowStyles {
    pub const fn new() -> Self {
        Self {
            window_style: Style::new().bg(tailwind::BLUE.c950),
            window_border_style: Style::new().fg(Color::LightBlue),
            title_style: Style::new().fg(Color::LightBlue),
            title_border_style: Style::new().fg(Color::LightBlue),
            unselected_input_border_style: Style::new().fg(Color::Gray),
            selected_input_border_style: Style::new().fg(Color::LightCyan),
            footer_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::SLATE.c800),
            valid_input_style: Style::new().fg(Color::LightGreen),
            valid_cursor_style: Style::new().bg(Color::LightGreen),
            invalid_input_style: Style::new().fg(Color::LightRed),
            invalid_cursor_style: Style::new().bg(Color::LightRed),
            cursor_style: Style::new().bg(Color::White),
        }
    }
}
