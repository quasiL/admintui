use crate::app::Screen;
use crate::cron::{Inputs, TableColors};
use crate::menu::MainMenu;
use chrono::{DateTime, Utc};
use cron::Schedule;
use ratatui::{
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout, Margin, Rect},
    prelude::*,
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState,
    },
};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use unicode_width::UnicodeWidthStr;

const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (↑) move up | (↓) move down",
    "(Enter) select | (n) new",
];
const ITEM_HEIGHT: usize = 4;

pub struct CronJob {
    pub cron_notation: String,
    pub job: String,
    pub job_description: String,
    pub next_execution: String,
}

impl CronJob {
    const fn ref_array(&self) -> [&String; 3] {
        [
            &self.cron_notation,
            &self.next_execution,
            &self.job_description,
        ]
    }

    pub fn cron_notation(&self) -> &str {
        &self.cron_notation
    }

    fn _next_execution(&self) -> &str {
        &self.next_execution
    }

    pub fn job_description(&self) -> &str {
        &self.job_description
    }

    fn from_crontab(file_path: &str) -> Result<Vec<CronJob>, io::Error> {
        let path = Path::new(file_path);
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut cron_jobs = Vec::new();
        let mut current_description = String::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                continue;
            } else if line.starts_with('#') {
                current_description = line.trim_start_matches('#').trim().to_string();
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 6 {
                    continue;
                }

                let cron_notation = parts[..5].join(" ");
                let job = parts[5..].join(" ");

                let next_execution = format!("* {}", cron_notation);
                let modified_next_execution = get_next_execution(&next_execution)
                    .map(|dt| dt.to_string())
                    .unwrap_or_else(|| "N/A".to_string());

                cron_jobs.push(CronJob {
                    cron_notation,
                    job,
                    job_description: current_description.clone(),
                    next_execution: modified_next_execution,
                });
            }
        }

        Ok(cron_jobs)
    }
}

fn get_next_execution(cron_expr: &str) -> Option<DateTime<Utc>> {
    let schedule = Schedule::from_str(cron_expr).ok()?;
    schedule.upcoming(Utc).next()
}

pub struct CronTable {
    state: TableState,
    items: Vec<CronJob>,
    longest_item_lens: (u16, u16, u16),
    scroll_state: ScrollbarState,
    colors: TableColors,
    show_popup: bool,
    inputs: Inputs,
}

impl Widget for &mut CronTable {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(area);

        self.render_table(rects[0], buf);
        self.render_scrollbar(rects[0], buf);
        self.render_footer(rects[1], buf);

        if self.show_popup {
            self.inputs
                .render_inputs(rects[0], buf, &self.state, &self.items);
        }
    }
}

impl CronTable {
    pub fn new() -> Self {
        let cron_jobs_vec = CronJob::from_crontab("crontab").unwrap();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&cron_jobs_vec),
            scroll_state: ScrollbarState::new((cron_jobs_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(),
            items: cron_jobs_vec,
            show_popup: false,
            inputs: Inputs::default(),
        }
    }

    pub fn handle_screen(&mut self, key: event::KeyEvent) -> Option<Screen> {
        if self.show_popup == true {
            self.inputs.handle_inputs(
                key,
                &mut self.show_popup,
                &mut self.items[self.state.selected().unwrap()],
            );
            None
        } else {
            match key.code {
                KeyCode::Esc => Some(Screen::MainMenu(MainMenu::new())),
                KeyCode::Char('j') | KeyCode::Down => {
                    self.next_row();
                    None
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.previous_row();
                    None
                }
                KeyCode::Char('n') => {
                    self.show_popup = true;
                    None
                }
                KeyCode::Enter => {
                    self.show_popup = true;
                    self.inputs.first_render();
                    self.inputs
                        .setup_inputs(&self.items[self.state.selected().unwrap()]);
                    None
                }
                _ => None,
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

    fn render_table(&mut self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default()
            .fg(self.colors.header_text_color)
            .bg(self.colors.header_color);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_color);

        let header = ["Cron Notation", "Next Execution", "Description"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .bold()
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_text_color).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 4),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
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
        let scrollbar_style = Style::default().fg(Color::White).bg(Color::DarkGray);

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .style(scrollbar_style);

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
        let footer_style = Style::default();
        // .fg(self.colors.row_fg)
        // .bg(self.colors.buffer_bg);

        let border_style = Style::default().fg(self.colors.footer_border_color);

        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(footer_style)
            .centered()
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_type(BorderType::Plain)
                    .border_style(border_style),
            );

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
