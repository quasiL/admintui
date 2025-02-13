use crate::app::{Screen, ScreenTrait};
use crate::ftp::TableStyles;
use crate::menu::MainMenu;
use ratatui::{
    crossterm::event::{self, KeyCode, MouseEvent},
    layout::{Constraint, Layout, Margin, Rect},
    prelude::{Buffer, StatefulWidget, Widget},
    text::Text,
    widgets::{
        Block, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
};
use unicode_width::UnicodeWidthStr;

const INFO_TEXT: [&str; 3] = [
    "",
    "(Esc) Return to the main menu | (↓↑) Move up and down | (Enter) Select | (d) Delete selected user | (n) Add new user",
    "",
];
const ITEM_HEIGHT: usize = 4;

pub struct FtpUser {
    pub username: String,
    pub doc_root: String,
}

impl FtpUser {
    const fn ref_array(&self) -> [&String; 2] {
        [&self.username, &self.doc_root]
    }

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
    scroll_state: ScrollbarState,
    styles: TableStyles,
    show_popup: bool,
}

impl ScreenTrait for FtpTable {
    fn new() -> Self {
        let ftp_users = vec![
            FtpUser::new("alice", "/var/ftp/alice"),
            FtpUser::new("bob", "/var/ftp/bob"),
            FtpUser::new("charlie", "/var/ftp/charlie"),
        ];
        let longest_item_lens = constraint_len_calculator(&ftp_users);
        let scroll_position = if ftp_users.is_empty() {
            0
        } else {
            (ftp_users.len() - 1) * ITEM_HEIGHT
        };
        Self {
            state: TableState::default().with_selected(0),
            items: ftp_users,
            longest_item_lens,
            scroll_state: ScrollbarState::new(scroll_position),
            styles: TableStyles::new(),
            show_popup: false,
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]);
        let rects = vertical.split(area);

        self.render_table(rects[0], buf);
        self.render_scrollbar(rects[0], buf);
        self.render_footer(rects[1], buf);

        if self.show_popup {
            //self.render_popup(rects[0], buf);
        }
    }

    fn handle_screen(
        &mut self,
        key: event::KeyEvent,
        _mouse: Option<MouseEvent>,
    ) -> Option<Screen> {
        if key.code == KeyCode::Esc {
            Some(Screen::MainMenu(MainMenu::new()))
        } else {
            self.handle_keys(key);
            None
        }
    }
}

impl FtpTable {
    fn handle_keys(&mut self, key: event::KeyEvent) {
        if self.show_popup == true {
            // self.inputs
            //     .handle_inputs(key, &mut self.show_popup, &mut self.items, &mut self.state);
        } else {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.next_row();
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.previous_row();
                }
                KeyCode::Char('g') | KeyCode::Home => {
                    self.first_row();
                }
                KeyCode::Char('G') | KeyCode::End => {
                    self.last_row();
                }
                KeyCode::Char('n') => {
                    self.show_popup = true;
                    // self.inputs.init_empty();
                }
                KeyCode::Esc => {
                    self.show_popup = false;
                }
                KeyCode::Char('d') => {
                    let index = self.state.selected().unwrap();
                    self.items.remove(index);
                    // save_to_crontab(&self.items).unwrap_or_else(|err| {
                    //     eprint!("Error saving to crontab: {}", err);
                    // });
                }
                // KeyCode::Enter => {
                //     if !self.items.is_empty() {
                //         self.show_popup = true;
                //         self.inputs.is_new = false;
                //         self.inputs.init(&mut self.items, &mut self.state);
                //     }
                // }
                _ => {}
            }
        }
    }

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

    fn first_row(&mut self) {
        self.state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0);
    }

    fn last_row(&mut self) {
        if !self.items.is_empty() {
            let last_index = self.items.len() - 1;
            self.state.select(Some(last_index));
            self.scroll_state = self.scroll_state.position(last_index * ITEM_HEIGHT);
        }
    }

    fn render_table(&mut self, area: Rect, buf: &mut Buffer) {
        let header = ["User", "Document Root"]
            .into_iter()
            .map(|title| Cell::from(Text::from(format!("\n{}\n", title)))) // Adds top and bottom padding
            .collect::<Row>()
            .style(self.styles.header_style)
            .height(3);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.styles.normal_row_color,
                _ => self.styles.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(self.styles.row_style.bg(color))
                .height(ITEM_HEIGHT.try_into().unwrap())
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 8),
                Constraint::Min(self.longest_item_lens.1 + 1),
            ],
        )
        .header(header)
        .row_highlight_style(self.styles.selected_row_style)
        .style(
            self.styles
                .row_style
                .bg(if (self.items.len() + 1) % 2 == 0 {
                    self.styles.alt_row_color
                } else {
                    self.styles.normal_row_color
                }),
        )
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .highlight_spacing(HighlightSpacing::Always);
        StatefulWidget::render(t, area, buf, &mut self.state);
    }

    fn render_scrollbar(&mut self, area: Rect, buf: &mut Buffer) {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .style(self.styles.scrollbar_style);

        StatefulWidget::render(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            buf,
            &mut self.scroll_state,
        );
    }

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(self.styles.footer_style)
            .centered()
            .block(Block::default());

        Widget::render(info_footer, area, buf);
    }

    fn render_popup(area: Rect, buf: &mut Buffer) {}
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
