use crate::cron::CronJob;
use ratatui::{
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    prelude::*,
    style::{self, Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, TableState},
};
use style::palette::tailwind;
use tui_textarea::TextArea;

pub enum ActiveInput {
    CronNotation,
    Job,
    JobDescription,
}

impl ActiveInput {
    pub fn next(&self) -> Self {
        match self {
            ActiveInput::CronNotation => ActiveInput::Job,
            ActiveInput::Job => ActiveInput::JobDescription,
            ActiveInput::JobDescription => ActiveInput::CronNotation,
        }
    }
}

pub struct Inputs {
    pub cron_notation: TextArea<'static>,
    pub job: TextArea<'static>,
    pub job_description: TextArea<'static>,
    pub current_input: ActiveInput,
    pub cron_notation_value: String,
    pub job_value: String,
    pub job_description_value: String,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            cron_notation: TextArea::default(),
            job: TextArea::default(),
            job_description: TextArea::default(),
            current_input: ActiveInput::CronNotation,
            cron_notation_value: String::new(),
            job_value: String::new(),
            job_description_value: String::new(),
        }
    }
}

impl Inputs {
    pub fn handle_inputs(&mut self, key: event::KeyEvent, show_popup: &mut bool) {
        match key.code {
            KeyCode::Tab => {
                self.current_input = self.current_input.next();
            }
            KeyCode::Esc => {
                *show_popup = false;
                let cron_notation_input = &mut self.cron_notation;
                cron_notation_input.delete_line_by_head();
                let job_input = &mut self.job;
                job_input.delete_line_by_head();
                let job_description_input = &mut self.job_description;
                job_description_input.delete_line_by_head();
            }
            _ => match self.current_input {
                ActiveInput::CronNotation => {
                    let cron_input = &mut self.cron_notation;
                    let cron_value = &mut self.cron_notation_value;
                    if cron_input.input(key) {
                        validate(cron_input);
                        if let Some(first_line) = cron_input.lines().get(0) {
                            cron_value.clear();
                            cron_value.push_str(first_line);
                        } else {
                            cron_value.clear();
                        }
                    }
                }
                ActiveInput::Job => {
                    let job_input = &mut self.job;
                    let job_value = &mut self.job_value;
                    if job_input.input(key) {
                        if let Some(first_line) = job_input.lines().get(0) {
                            job_value.clear();
                            job_value.push_str(first_line);
                        } else {
                            job_value.clear();
                        }
                    }
                }
                ActiveInput::JobDescription => {
                    let job_description_input = &mut self.job_description;
                    let job_description_value = &mut self.job_description_value;
                    if job_description_input.input(key) {
                        if let Some(first_line) = job_description_input.lines().get(0) {
                            job_description_value.clear();
                            job_description_value.push_str(first_line);
                        } else {
                            job_description_value.clear();
                        }
                    }
                }
            },
        }
    }

    pub fn setup_inputs(&mut self, selected_cron: &CronJob) {
        let cron_notation_value = &selected_cron.cron_notation;
        let cron_notation_input = &mut self.cron_notation;
        cron_notation_input.delete_line_by_head();
        cron_notation_input.insert_str(cron_notation_value);

        let job_value = &selected_cron.job;
        let job_input = &mut self.job;
        job_input.delete_line_by_head();
        job_input.insert_str(job_value);

        let job_description_value = &selected_cron.job_description;
        let job_description_input = &mut self.job_description;
        job_description_input.delete_line_by_head();
        job_description_input.insert_str(job_description_value);
    }

    pub fn render_inputs(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        state: &TableState,
        items: &[CronJob],
    ) {
        let area = popup_area(area, 80);
        Widget::render(Clear, area, buf);

        let block = Block::default()
            .bg(tailwind::SLATE.c700)
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Color::LightBlue);
        block.render(area, buf);

        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(2)
        .flex(Flex::Start);
        let [title, cron_notation_area, job_area, description_area] = vertical.areas(area);

        let selected_index = state.selected().unwrap();
        let selected_cron = &items[selected_index];
        let selected_cron_description =
            format!("Selected cron: {}", selected_cron.job_description());

        let cron_info = Paragraph::new(Text::from_iter([selected_cron_description]))
            .style(Style::default().fg(Color::LightBlue))
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Color::LightBlue),
            );

        Widget::render(cron_info, title, buf);

        let cron_input = &mut self.cron_notation;
        let job_input = &mut self.job;
        let description_input = &mut self.job_description;

        cron_input.render(cron_notation_area, buf);

        job_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightCyan)),
        );
        //job_input.set_cursor_style(Style::default());
        job_input.render(job_area, buf);

        description_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightCyan)),
        );
        //description_input.set_cursor_style(Style::default());
        description_input.render(description_area, buf);

        match self.current_input {
            ActiveInput::CronNotation => {
                cron_input.set_cursor_line_style(Style::default());
                cron_input.set_placeholder_text("Enter a cron notation");
                cron_input.render(cron_notation_area, buf);

                cron_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan)),
                );
                cron_input.set_cursor_style(Style::default());
                cron_input.render(cron_notation_area, buf);
            }
            ActiveInput::Job => {
                job_input.set_cursor_line_style(Style::default());
                job_input.set_placeholder_text("Enter a job");
                job_input.render(job_area, buf);

                cron_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan)),
                );
                cron_input.set_cursor_style(Style::default());
                cron_input.render(cron_notation_area, buf);
            }
            ActiveInput::JobDescription => {
                description_input.set_cursor_line_style(Style::default());
                description_input.set_placeholder_text("Enter a description");
                description_input.render(description_area, buf);

                cron_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan)),
                );
                cron_input.set_cursor_style(Style::default());
                cron_input.render(cron_notation_area, buf);
            }
        }
    }
}

fn popup_area(area: Rect, percent_x: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Max(16)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn validate(textarea: &mut TextArea) -> bool {
    use cron::Schedule;
    use std::str::FromStr;

    let input = textarea
        .lines()
        .get(0)
        .map(|s| s.as_str())
        .unwrap_or("")
        .trim();

    if Schedule::from_str(input).is_err() {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightRed))
                .title("ERROR: Invalid cron syntax"),
        );
        false
    } else {
        textarea.set_style(Style::default().fg(Color::LightGreen));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightGreen))
                .title("OK"),
        );
        true
    }
}
