use crate::app::{Screen, ScreenTrait};
use crate::cron::CronTable;
use crate::ftp::FtpTable;
use crate::menu::MenuStyles;
use crate::mysql::Mysql;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, MouseEvent, MouseEventKind},
    layout::{Constraint, Layout, Rect},
    symbols,
    text::{Line, Text},
    widgets::{Block, Borders, List, ListState, Paragraph, StatefulWidget, Widget},
};

const INFO_TEXT: [&str; 3] = [
    "",
    "(Esc|q) Quit | (↓↑) Move up and down | (Enter) Select",
    "",
];

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
                label: "🕜 Cron Table",
                action: || Screen::CronTable(CronTable::new()),
            },
            MenuItem {
                label: "📒 FTP",
                action: || Screen::FtpTable(FtpTable::new()),
            },
            MenuItem {
                label: "🔒 MySQL",
                action: || Screen::Mysql(Mysql::new()),
            },
            MenuItem {
                label: "🌎 Webserver",
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
            Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).areas(area);

        self.render_list(main_area, buf);
        self.render_footer(footer_area, buf);
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
                let menu_start_row = 2;
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
        let block = Block::new()
            .title(Line::raw("AdminTUI").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(self.styles.header_style)
            .style(self.styles.menu_background_style);

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

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(self.styles.footer_style)
            .centered()
            .block(Block::default());

        Widget::render(info_footer, area, buf);
    }
}
