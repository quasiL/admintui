use crate::cron::utils::get_next_execution;
use crate::cron::CronJob;
use arboard::Clipboard;
use ratatui::{
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    prelude::{Buffer, Widget},
    style::{self, Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, TableState},
};
use regex::Regex;
use style::palette::tailwind;
use tui_textarea::{CursorMove, TextArea};

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
    pub is_new: bool,
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
            is_new: true,
        }
    }
}

impl Inputs {
    pub fn handle_inputs(
        &mut self,
        key: event::KeyEvent,
        show_popup: &mut bool,
        cron_jobs: &mut Vec<CronJob>,
        table_state: &mut TableState,
    ) {
        let ctrl_pressed = key.modifiers.contains(event::KeyModifiers::CONTROL);
        match key.code {
            KeyCode::Tab => {
                self.current_input = self.current_input.next();
            }
            KeyCode::Esc => {
                *show_popup = false;
                // let cron_notation_input = &mut self.cron_notation;
                // cron_notation_input.delete_line_by_head();
                // let job_input = &mut self.job;
                // job_input.delete_line_by_head();
                // let job_description_input = &mut self.job_description;
                // job_description_input.delete_line_by_head();
                self.flash_inputs();
                self.flash_values();

                self.current_input = ActiveInput::CronNotation;
            }
            KeyCode::Enter => {
                if self.is_new {
                    cron_jobs.push(self.create_new_cron());
                    table_state.select(Some(cron_jobs.len() - 1));
                } else {
                    self.update_selected_cron(&mut cron_jobs[table_state.selected().unwrap()]);
                }
                *show_popup = false;
            }
            KeyCode::Char('v') if ctrl_pressed => {
                self.handle_paste();
            }
            _ => match self.current_input {
                ActiveInput::CronNotation => {
                    let cron_input = &mut self.cron_notation;
                    let cron_value = &mut self.cron_notation_value;
                    if cron_input.input(key) {
                        validate(cron_input);
                        cron_value.clear();
                        if let Some(first_line) = cron_input.lines().get(0) {
                            cron_value.push_str(first_line);
                        }
                    }
                }
                ActiveInput::Job => {
                    let job_input = &mut self.job;
                    let job_value = &mut self.job_value;
                    if job_input.input(key) {
                        job_value.clear();
                        if let Some(first_line) = job_input.lines().get(0) {
                            job_value.push_str(first_line);
                        }
                    }
                }
                ActiveInput::JobDescription => {
                    let job_description_input = &mut self.job_description;
                    let job_description_value = &mut self.job_description_value;
                    if job_description_input.input(key) {
                        job_description_value.clear();
                        if let Some(first_line) = job_description_input.lines().get(0) {
                            job_description_value.push_str(first_line);
                        }
                    }
                }
            },
        }
    }

    fn flash_inputs(&mut self) {
        self.cron_notation.delete_line_by_head();
        self.job.delete_line_by_head();
        self.job_description.delete_line_by_head();
    }

    fn flash_values(&mut self) {
        self.cron_notation_value.clear();
        self.job_value.clear();
        self.job_description_value.clear();
    }

    pub fn init_empty(&mut self) {
        self.is_new = true;
        self.flash_inputs();
        self.flash_values();
    }

    pub fn init(&mut self, cron_jobs: &mut Vec<CronJob>, table_state: &mut TableState) {
        self.flash_inputs();
        self.flash_values();

        if !self.is_new {
            let selected_cron = &mut cron_jobs[table_state.selected().unwrap()];
            self.cron_notation.insert_str(&selected_cron.cron_notation);
            self.job.insert_str(&selected_cron.job);
            self.job_description
                .insert_str(&selected_cron.job_description);

            self.cron_notation_value = selected_cron.cron_notation.to_string();
            self.job_value = selected_cron.job.to_string();
            self.job_description_value = selected_cron.job_description.to_string();
        }
    }

    fn create_new_cron(&mut self) -> CronJob {
        let mut new_cron = CronJob::default();
        new_cron.cron_notation = format!("{}", self.cron_notation_value);
        new_cron.job = format!("{}", self.job_value);
        new_cron.next_execution = get_next_execution(&self.cron_notation_value);
        new_cron.job_description = format!("{}", self.job_description_value);
        new_cron
    }

    fn update_selected_cron(&mut self, selected_cron: &mut CronJob) {
        selected_cron.cron_notation = format!("{}", self.cron_notation_value);
        selected_cron.job = format!("{}", self.job_value);
        selected_cron.job_description = format!("{}", self.job_description_value);
    }

    fn handle_paste(&mut self) {
        let mut clipboard = Clipboard::new().unwrap();

        match self.current_input {
            ActiveInput::CronNotation => {
                self.cron_notation.move_cursor(CursorMove::End);
                self.cron_notation.insert_str(clipboard.get_text().unwrap());
            }
            ActiveInput::Job => {
                self.job.move_cursor(CursorMove::End);
                self.job.insert_str(clipboard.get_text().unwrap());
            }
            ActiveInput::JobDescription => {
                self.job_description.move_cursor(CursorMove::End);
                self.job_description
                    .insert_str(clipboard.get_text().unwrap());
            }
        }
    }

    // pub fn first_render(&mut self) {
    //     let cron_input = &mut self.cron_notation;
    //     cron_input.set_block(
    //         Block::default()
    //             .borders(Borders::ALL)
    //             .border_style(Style::default().fg(Color::LightCyan)),
    //     );
    // }

    pub fn render_inputs(&mut self, area: Rect, buf: &mut Buffer) {
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

        let selected_cron_notation = convert_to_human_readable(self.cron_notation_value.as_str());

        let cron_info = Paragraph::new(Text::from_iter([selected_cron_notation]))
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

        match self.current_input {
            ActiveInput::CronNotation => {
                cron_input.set_placeholder_text("Enter a cron notation");
                cron_input.set_cursor_line_style(Style::default());
                cron_input.render(cron_notation_area, buf);

                job_input.set_cursor_style(Style::default());
                job_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray)),
                );
                job_input.render(job_area, buf);

                description_input.set_cursor_style(Style::default());
                description_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray)),
                );
                description_input.render(description_area, buf);
            }
            ActiveInput::Job => {
                job_input.set_placeholder_text("Enter a job");
                job_input.set_cursor_line_style(Style::default());
                job_input.set_cursor_style(Style::default().bg(Color::White));
                job_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan)),
                );
                job_input.render(job_area, buf);

                cron_input.render(cron_notation_area, buf);

                description_input.set_cursor_style(Style::default());
                description_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray)),
                );
                description_input.render(description_area, buf);
            }
            ActiveInput::JobDescription => {
                description_input.set_placeholder_text("Enter a description");
                description_input.set_cursor_line_style(Style::default());
                description_input.set_cursor_style(Style::default().bg(Color::White));
                description_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan)),
                );
                description_input.render(description_area, buf);

                cron_input.render(cron_notation_area, buf);

                job_input.set_cursor_style(Style::default());
                job_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray)),
                );
                job_input.render(job_area, buf);
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

    let modified_input = format!("* {}", input);

    if Schedule::from_str(modified_input.as_str()).is_err() {
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

pub fn convert_to_human_readable(input: &str) -> String {
    let cron_regex = Regex::new(r"^(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)$").unwrap();

    if let Some(captures) = cron_regex.captures(input.trim()) {
        let minute = &captures[1];
        let hour = &captures[2];
        let day_of_month = &captures[3];
        let month = &captures[4];
        let day_of_week = &captures[5];

        let minute_str = parse_minute(minute);
        let hour_str = parse_hour(hour);
        let day_of_month_str = parse_day_of_month(day_of_month);
        let month_str = parse_month(month);
        let day_of_week_str = parse_day_of_week(day_of_week);

        return format!(
            "At {} past {} on {} {} {}",
            minute_str, hour_str, day_of_month_str, month_str, day_of_week_str
        );
    }

    "Unable to parse cron expression into human-readable format.".to_string()
}

fn parse_minute(minute: &str) -> String {
    if minute == "*" {
        "every minute".to_string()
    } else if minute.contains("/") {
        format!(
            "every {} minute(s) starting at {}",
            &minute[2..],
            &minute[0..1]
        )
    } else {
        format!("minute {}", minute)
    }
}

fn parse_hour(hour: &str) -> String {
    if hour == "*" {
        "every hour".to_string()
    } else if hour.contains("/") {
        format!("every {} hour(s) starting at {}", &hour[2..], &hour[0..1])
    } else {
        format!("hour {}", hour)
    }
}

fn parse_day_of_month(day: &str) -> String {
    if day == "*" {
        "every day".to_string()
    } else if day.contains("/") {
        format!("every {} day(s) starting at day {}", &day[2..], &day[0..1])
    } else {
        format!("day-of-month {}", day)
    }
}

fn parse_month(month: &str) -> String {
    if month == "*" {
        "every month".to_string()
    } else if month.contains("/") {
        format!(
            "every {} month(s) starting at month {}",
            &month[2..],
            &month[0..1]
        )
    } else {
        format!("month {}", month)
    }
}

fn parse_day_of_week(day: &str) -> String {
    match day {
        "*" => "every day of the week".to_string(),
        "0" | "7" => "Sunday".to_string(),
        "1" => "Monday".to_string(),
        "2" => "Tuesday".to_string(),
        "3" => "Wednesday".to_string(),
        "4" => "Thursday".to_string(),
        "5" => "Friday".to_string(),
        "6" => "Saturday".to_string(),
        _ if day.contains("-") => {
            let parts: Vec<&str> = day.split('-').collect();
            format!(
                "every day-of-week from {} through {}",
                day_name(parts[0]),
                day_name(parts[1])
            )
        }
        _ if day.contains(",") => {
            let days: Vec<String> = day.split(',').map(day_name).collect();
            format!("{}", days.join(", "))
        }
        _ => format!("day-of-week {}", day),
    }
}

fn day_name(day: &str) -> String {
    match day {
        "0" | "7" => "Sunday".to_string(),
        "1" => "Monday".to_string(),
        "2" => "Tuesday".to_string(),
        "3" => "Wednesday".to_string(),
        "4" => "Thursday".to_string(),
        "5" => "Friday".to_string(),
        "6" => "Saturday".to_string(),
        _ => day.to_string(),
    }
}
