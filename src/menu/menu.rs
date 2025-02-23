use crate::app::{Screen, ScreenTrait};
use crate::cron::CronTable;
use crate::ftp::FtpTable;
use crate::menu::MenuStyles;
use crate::mysql::Mysql;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, MouseEvent, MouseEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListState, StatefulWidget, Widget},
};
use tui_big_text::{BigText, PixelSize};

pub struct MainMenu {
    menu_list: MenuList,
    styles: MenuStyles,
    menu_items: Vec<MenuItem>,
}

struct MenuList {
    items: Vec<String>,
    state: ListState,
}

struct MenuItem {
    label: &'static str,
    action: fn() -> Screen,
}

impl ScreenTrait for MainMenu {
    fn new() -> Self {
        let menu_items = vec![
            MenuItem {
                label: "🕗 Cron Jobs",
                action: || Screen::CronTable(CronTable::new()),
            },
            MenuItem {
                label: "👤 FTP",
                action: || Screen::FtpTable(FtpTable::new()),
            },
            MenuItem {
                label: "🐬 MySQL",
                action: || Screen::Mysql(Mysql::new()),
            },
            MenuItem {
                label: "🌐 Webserver",
                action: || Screen::Quit,
            },
            MenuItem {
                label: "🔧 Settings",
                action: || Screen::Quit,
            },
        ];
        Self {
            menu_list: MenuList {
                items: menu_items
                    .iter()
                    .map(|item| item.label.to_string())
                    .collect(),
                state: ListState::default().with_selected(Some(0)),
            },
            styles: MenuStyles::new(),
            menu_items,
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [main_area, footer_area] =
            Layout::vertical([Constraint::Min(1), Constraint::Length(2)]).areas(area);

        let [title_area, menu_area] =
            Layout::vertical([Constraint::Length(5), Constraint::Min(1)]).areas(main_area);

        self.render_list(menu_area, buf);
        self.render_title(title_area, buf);
        self.render_footer(
            footer_area,
            buf,
            vec![
                ("<Esc/q>", "Quit"),
                ("<Enter>", "Select"),
                ("<↓↑>", "Move up and down"),
            ],
        );
    }

    fn handle_screen(&mut self, key: event::KeyEvent, mouse: Option<MouseEvent>) -> Option<Screen> {
        if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
            Some(Screen::Quit)
        } else if key.code == KeyCode::Enter {
            self.process_select()
        } else if let Some(mouse_event) = mouse {
            self.handle_mouse(mouse_event)
        } else {
            self.handle_keys(key);
            None
        }
    }
}

impl MainMenu {
    fn handle_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.menu_list.state.select_next();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.menu_list.state.select_previous();
            }
            KeyCode::Home => {
                self.menu_list.state.select_first();
            }
            KeyCode::End => {
                self.menu_list.state.select_last();
            }
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse_event: MouseEvent) -> Option<Screen> {
        match mouse_event.kind {
            MouseEventKind::Down(_) => {
                let menu_start_row = 6;
                let menu_height = self.menu_list.items.len();
                let item_vertical_span: usize = 3;

                if mouse_event.row >= menu_start_row
                    && mouse_event.row
                        < menu_start_row + menu_height as u16 * item_vertical_span as u16
                {
                    let selected_index =
                        (mouse_event.row as usize - menu_start_row as usize) / item_vertical_span;

                    self.menu_list.state.select(Some(selected_index));
                    return self.process_select();
                }
            }
            _ => {}
        }
        None
    }

    fn process_select(&mut self) -> Option<Screen> {
        self.menu_list
            .state
            .selected()
            .and_then(|index| self.menu_items.get(index))
            .map(|item| (item.action)())
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().style(self.styles.menu_background_style);

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

    fn render_title(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            // .borders(Borders::ALL)
            // .border_type(BorderType::Thick)
            .style(self.styles.header_border_style);

        let inner_area = block.inner(area);

        let [_top_padding, text_area, _bottom_padding] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(inner_area);

        let big_text = BigText::builder()
            .pixel_size(PixelSize::Sextant)
            .alignment(Alignment::Center)
            .style(self.styles.header_style)
            .lines(vec!["TUIxel".into()])
            .build();

        Widget::render(block, area, buf);
        Widget::render(big_text, text_area, buf);
    }
}
