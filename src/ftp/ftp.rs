use crate::app::{Screen, ScreenTrait};
use crate::menu::MainMenu;
use ratatui::{
    crossterm::event::{self, KeyCode, MouseEvent},
    layout::{Constraint, Layout, Rect},
    prelude::{Buffer, StatefulWidget, Widget},
    text::Text,
    widgets::{Block, Cell, Row, Table, TableState},
};
use unicode_width::UnicodeWidthStr;

pub struct FtpUser {
    pub username: String,
    pub doc_root: String,
}

impl FtpUser {
    pub fn new(username: &str, doc_root: &str) -> Self {
        Self {
            username: username.to_string(),
            doc_root: doc_root.to_string(),
        }
    }
}

pub struct FtpTable {
    state: TableState,
    items: Vec<FtpUser>,
    longest_item_lens: (u16, u16),
}

impl ScreenTrait for FtpTable {
    fn new() -> Self {
        let dummy_users = vec![
            FtpUser::new("alice", "/var/ftp/alice"),
            FtpUser::new("bob", "/var/ftp/bob"),
            FtpUser::new("charlie", "/var/ftp/charlie"),
        ];
        let longest_item_lens = constraint_len_calculator(&dummy_users);
        Self {
            state: TableState::default().with_selected(0),
            items: dummy_users,
            longest_item_lens,
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let header = ["Username", "Doc Root"]
            .iter()
            .map(|title| Cell::from(Text::from(format!("\n{}\n", title))))
            .collect::<Row>()
            .height(2);

        let rows = self.items.iter().map(|user| {
            Row::new(vec![
                Cell::from(Text::from(format!("\n{}\n", user.username))),
                Cell::from(Text::from(format!("\n{}\n", user.doc_root))),
            ])
            .height(2)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Length(self.longest_item_lens.0 + 2),
                Constraint::Min(self.longest_item_lens.1 + 1),
            ],
        )
        .header(header);

        StatefulWidget::render(table, area, buf, &mut self.state);
    }

    fn handle_screen(
        &mut self,
        key: event::KeyEvent,
        _mouse: Option<MouseEvent>,
    ) -> Option<Screen> {
        if key.code == KeyCode::Esc {
            Some(Screen::MainMenu(MainMenu::new()))
        } else {
            //self.handle_keys(key);
            None
        }
    }
}

impl FtpTable {
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i < self.items.len() - 1 => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => self.items.len() - 1,
        };
        self.state.select(Some(i));
    }
}

fn constraint_len_calculator(items: &[FtpUser]) -> (u16, u16) {
    let username_len = items
        .iter()
        .map(|user| user.username.as_str())
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let doc_root_len = items
        .iter()
        .map(|user| user.doc_root.as_str())
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (username_len as u16, doc_root_len as u16)
}
