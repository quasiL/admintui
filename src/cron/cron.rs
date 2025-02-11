use crate::app::{Screen, ScreenTrait};
use crate::cron::utils::{from_crontab, save_to_crontab};
use crate::cron::{Inputs, TableStyles};
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
    "(Esc) Return to the main menu | (↓↑) Move up and down | (Enter) Select | (d) Delete selected cron | (n) Add new cron",
    "",
];
const ITEM_HEIGHT: usize = 4;

pub struct CronJob {
    pub cron_notation: String,
    pub job: String,
    pub job_description: String,
    pub next_execution: String,
}

impl Default for CronJob {
    fn default() -> Self {
        Self {
            cron_notation: String::new(),
            job: String::new(),
            job_description: String::new(),
            next_execution: String::new(),
        }
    }
}

impl CronJob {
    const fn ref_array(&self) -> [&String; 3] {
        [
            &self.cron_notation,
            &self.next_execution,
            &self.job_description,
        ]
    }

    pub fn new(cron_job: CronJob) -> Self {
        Self {
            cron_notation: cron_job.cron_notation,
            job: cron_job.job,
            job_description: cron_job.job_description,
            next_execution: cron_job.next_execution,
        }
    }
}

pub struct CronTable {
    state: TableState,
    items: Vec<CronJob>,
    longest_item_lens: (u16, u16, u16),
    scroll_state: ScrollbarState,
    styles: TableStyles,
    show_popup: bool,
    inputs: Inputs,
}

impl ScreenTrait for CronTable {
    fn new() -> Self {
        let cron_jobs_vec = from_crontab().unwrap_or_else(|err| {
            tracing::error!("Error reading crontab: {}", err);
            vec![CronJob {
                cron_notation: format!("Error: {}", err),
                job: String::new(),
                job_description: String::new(),
                next_execution: String::new(),
            }]
        });
        let scroll_position = if cron_jobs_vec.is_empty() {
            0
        } else {
            (cron_jobs_vec.len() - 1) * ITEM_HEIGHT
        };
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&cron_jobs_vec),
            scroll_state: ScrollbarState::new(scroll_position),
            styles: TableStyles::new(),
            items: cron_jobs_vec,
            show_popup: false,
            inputs: Inputs::default(),
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let vertical = &Layout::vertical([Constraint::Min(1), Constraint::Length(3)]);
        let rects = vertical.split(area);

        self.render_table(rects[0], buf);
        self.render_scrollbar(rects[0], buf);
        self.render_footer(rects[1], buf);

        if self.show_popup {
            self.inputs.render_inputs(rects[0], buf);
        }
    }

    fn handle_screen(
        &mut self,
        key: event::KeyEvent,
        _mouse: Option<MouseEvent>,
    ) -> Option<Screen> {
        if key.code == KeyCode::Esc && self.show_popup == false {
            Some(Screen::MainMenu(MainMenu::new()))
        } else {
            self.handle_keys(key);
            None
        }
    }
}

impl CronTable {
    fn handle_keys(&mut self, key: event::KeyEvent) {
        if self.show_popup == true {
            self.inputs
                .handle_inputs(key, &mut self.show_popup, &mut self.items, &mut self.state);
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
                    self.inputs.init_empty();
                }
                KeyCode::Char('d') => {
                    let index = self.state.selected().unwrap();
                    self.items.remove(index);
                    save_to_crontab(&self.items).unwrap_or_else(|err| {
                        eprint!("Error saving to crontab: {}", err);
                    });
                }
                KeyCode::Enter => {
                    if !self.items.is_empty() {
                        self.show_popup = true;
                        self.inputs.is_new = false;
                        self.inputs.init(&mut self.items, &mut self.state);
                    }
                }
                _ => {}
            }
        }
    }

    fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
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
        let header = ["Cron Notation", "Next Execution", "Description"]
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
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 8),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
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
}

fn constraint_len_calculator(items: &[CronJob]) -> (u16, u16, u16) {
    let cron_notation_len = items
        .iter()
        .map(|job| job.cron_notation.as_str())
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let next_execution_len = items
        .iter()
        .map(|job| job.next_execution.as_str())
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let job_description_len = items
        .iter()
        .map(|job| job.job_description.as_str())
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (
        cron_notation_len as u16,
        next_execution_len as u16,
        job_description_len as u16,
    )
}
