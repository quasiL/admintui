use crate::app::Screen;
use crate::cron::CronTable;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
//const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
//const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct MainMenu {
    menu_list: MenuList,
}

struct MenuList {
    items: Vec<String>,
    state: ListState,
}

impl Widget for &mut MainMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).areas(area);

        self.render_list(main_area, buf);
        MainMenu::render_footer(footer_area, buf);
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
        }
    }

    pub fn handle_event(&mut self, key: event::KeyEvent) -> Option<Screen> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.select_next();
                None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.select_previous();
                None
            }
            KeyCode::Char('g') | KeyCode::Home => {
                self.select_first();
                None
            }
            KeyCode::Char('G') | KeyCode::End => {
                self.select_last();
                None
            }
            KeyCode::Char('q') => Some(Screen::Quit),
            KeyCode::Enter => self.process_select(),
            _ => None,
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

    fn process_select(&mut self) -> Option<Screen> {
        if let Some(selected) = self.menu_list.state.selected() {
            match selected {
                0 => Some(Screen::CronTable(CronTable::new())),
                _ => None,
            }
        } else {
            None
        }
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, Enter to select, q to quit.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("AdminTUI").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = self
            .menu_list
            .items
            .iter()
            .map(|todo_item| ListItem::new(Line::styled(todo_item, TEXT_FG_COLOR).centered()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE);

        StatefulWidget::render(list, area, buf, &mut self.menu_list.state);
    }
}
