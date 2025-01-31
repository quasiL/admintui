use crate::app::Screen;
use crate::cron::CronTable;
use crate::menu::MenuStyles;
use ratatui::{
    buffer::Buffer,
    crossterm::{
        event::{self, KeyCode},
        style::Color,
    },
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

const INFO_TEXT: [&str; 3] = [
    "",
    "(Esc|q) quit | (↓↑) move up and down | (Enter) select",
    "",
];

pub struct MainMenu {
    menu_list: MenuList,
    styles: MenuStyles,
}

struct MenuList {
    items: Vec<String>,
    state: ListState,
}

impl Widget for &mut MainMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).areas(area);

        self.render_list(main_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            menu_list: MenuList {
                items: vec![
                    String::from("Cron Table"),
                    String::from("Main 2"),
                    String::from("Main 3"),
                    String::from("Main 4"),
                    String::from("Main 5"),
                ],
                state: ListState::default().with_selected(Some(0)),
            },
            styles: MenuStyles::new(),
        }
    }

    pub fn handle_screen(&mut self, key: event::KeyEvent) -> Option<Screen> {
        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
            Some(Screen::Quit)
        } else if key.code == KeyCode::Enter {
            self.process_select()
        } else {
            self.handle_keys(key);
            None
        }
    }

    fn process_select(&mut self) -> Option<Screen> {
        match self.menu_list.state.selected() {
            Some(0) => Some(Screen::CronTable(CronTable::new())),
            _ => None,
        }
    }

    fn handle_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
            }
            KeyCode::Home => {
                self.select_first();
            }
            KeyCode::End => {
                self.select_last();
            }
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.menu_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.menu_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.menu_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.menu_list.state.select_last();
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(self.styles.footer_style)
            .centered()
            .block(Block::default());

        Widget::render(info_footer, area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("AdminTUI").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(self.styles.header_style)
            .bg(self.styles.menu_background_color);

        //     .map(|todo_item| {
        //         ListItem::new(Line::styled(todo_item, self.styles.row_text_color).centered())
        //     })
        let items: Vec<Text> = self
            .menu_list
            .items
            .iter()
            .map(|item| Text::from_iter(["", item.as_str(), ""]).centered())
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(self.styles.selected_row_style);

        StatefulWidget::render(list, area, buf, &mut self.menu_list.state);
    }
}
