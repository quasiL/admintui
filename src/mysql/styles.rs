use ratatui::style::{self, Color, Modifier, Style};
use style::palette::tailwind;

pub struct ScreenStyles {
    pub items_style: Style,
    pub items_border_style: Style,
    pub info_style: Style,
    pub info_border_style: Style,
    pub selected_item_style: Style,
    pub scrollbar_style: Style,
    pub footer_style: Style,
}

impl ScreenStyles {
    pub const fn new() -> Self {
        Self {
            items_style: Style::new()
                .fg(tailwind::GRAY.c300)
                .bg(tailwind::SLATE.c800),
            items_border_style: Style::new()
                .fg(tailwind::GRAY.c400)
                .bg(tailwind::SLATE.c800),
            info_style: Style::new()
                .fg(tailwind::GRAY.c300)
                .bg(tailwind::SLATE.c800),
            info_border_style: Style::new()
                .fg(tailwind::GRAY.c400)
                .bg(tailwind::SLATE.c800),
            selected_item_style: Style::new()
                .fg(tailwind::SLATE.c900)
                .bg(tailwind::GRAY.c400),
            scrollbar_style: Style::new()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::REVERSED),
            footer_style: Style::new()
                .fg(tailwind::GRAY.c300)
                .bg(tailwind::SLATE.c700),
        }
    }
}
