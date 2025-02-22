use ratatui::style::{self, Modifier, Style};
use style::palette::tailwind;

pub struct MenuStyles {
    pub header_style: Style,
    pub menu_background_style: Style,
    pub selected_row_style: Style,
}

impl MenuStyles {
    pub fn new() -> MenuStyles {
        MenuStyles {
            header_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::BLUE.c800)
                .add_modifier(Modifier::BOLD),
            menu_background_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::SLATE.c900),
            selected_row_style: Style::new()
                .fg(tailwind::SLATE.c100)
                .bg(tailwind::SLATE.c800)
                .add_modifier(Modifier::BOLD),
        }
    }
}
