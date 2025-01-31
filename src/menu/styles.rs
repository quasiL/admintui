use ratatui::style::{self, Color, Modifier, Style};
use style::palette::tailwind;

pub struct MenuStyles {
    pub header_style: Style,
    pub menu_background_color: Color,
    pub selected_row_style: Style,
    pub footer_style: Style,
}

impl MenuStyles {
    pub fn new() -> MenuStyles {
        MenuStyles {
            header_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::BLUE.c800)
                .add_modifier(Modifier::BOLD),
            menu_background_color: tailwind::SLATE.c900,
            selected_row_style: Style::new()
                .bg(tailwind::SLATE.c800)
                .add_modifier(Modifier::BOLD),
            footer_style: Style::default().bg(tailwind::SLATE.c800),
        }
    }
}
