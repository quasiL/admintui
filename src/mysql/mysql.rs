use crate::app::{Screen, ScreenTrait};
use crate::menu::MainMenu;
use crate::mysql::ScreenStyles;
use ratatui::widgets::Padding;
use ratatui::{
    crossterm::event::{self, KeyCode, MouseEvent},
    layout::{Constraint, Layout, Margin, Rect},
    prelude::{Buffer, StatefulWidget, Widget},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph,
        Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
};

const INFO_TEXT: [&str; 3] = [
    "",
    "(Esc) Return to the main menu | (↓↑) Move up and down | (Enter) Select | (d) Delete selected user | (n) Add new user",
    "",
];

const ITEM_HEIGHT: usize = 3;

pub struct MysqlUser {
    pub username: String,
}

impl MysqlUser {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }
}

pub struct Mysql {
    state: ListState,
    items: Vec<MysqlUser>,
    show_popup: bool,
    styles: ScreenStyles,
    scroll_state: ScrollbarState,
}

impl ScreenTrait for Mysql {
    fn new() -> Self {
        let mysql_users = vec![
            MysqlUser::new("root"),
            MysqlUser::new("admin"),
            MysqlUser::new("guest"),
        ];

        let state = ListState::default().with_selected(Some(0));
        let scroll_position = if mysql_users.is_empty() {
            0
        } else {
            (mysql_users.len() - 1) * ITEM_HEIGHT
        };

        Self {
            state,
            items: mysql_users,
            show_popup: false,
            styles: ScreenStyles::new(),
            scroll_state: ScrollbarState::new(scroll_position),
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).areas(area);

        let [user_list_area, info_area] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(main_area);

        self.render_list(user_list_area, buf);
        self.render_scrollbar(user_list_area, buf);
        self.render_info(info_area, buf);
        self.render_footer(footer_area, buf);
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

impl Mysql {
    fn handle_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.next_user(),
            KeyCode::Char('k') | KeyCode::Up => self.previous_user(),
            KeyCode::Char('n') => {
                // Add a new user (dummy for now)
                self.items.push(MysqlUser::new("new_user"));
            }
            KeyCode::Char('d') => {
                if let Some(selected) = self.state.selected() {
                    self.items.remove(selected);
                    if self.items.is_empty() {
                        self.state.select(None);
                    } else if selected >= self.items.len() {
                        self.state.select(Some(self.items.len() - 1));
                    }
                }
            }
            KeyCode::Enter => {
                self.show_popup = true;
            }
            _ => {}
        }
    }

    fn next_user(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i < self.items.len() - 1 => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn previous_user(&mut self) {
        let i = match self.state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => self.items.len().saturating_sub(1),
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|user| {
                ListItem::new(vec![
                    Line::from(""),
                    Line::from(user.username.as_str()).centered(),
                    Line::from(""),
                ])
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.styles.items_border_style)
                    .border_type(BorderType::Thick)
                    .title("Users")
                    .style(self.styles.items_style),
            )
            .highlight_style(self.styles.selected_item_style);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_info(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.styles.info_border_style)
            .border_type(BorderType::Thick)
            .style(self.styles.info_style);

        Widget::render(block, area, buf);
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
}
