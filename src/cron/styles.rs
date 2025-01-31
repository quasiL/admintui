use ratatui::style::{self, Color};
use style::palette::tailwind;

pub struct TableColors {
    pub header_color: Color,
    pub header_text_color: Color,
    pub row_text_color: Color,
    pub selected_row_color: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_color: Color,
}

impl TableColors {
    pub const fn new() -> Self {
        Self {
            header_color: tailwind::BLUE.c950,
            header_text_color: tailwind::SLATE.c200,
            row_text_color: tailwind::SLATE.c200,
            selected_row_color: tailwind::BLUE.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_color: tailwind::SLATE.c800,
        }
    }
}
