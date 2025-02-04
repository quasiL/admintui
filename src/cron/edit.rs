use crate::cron::utils::{convert_to_human_readable, get_next_execution};
use crate::cron::CronJob;
use arboard::Clipboard;
use ratatui::style::Modifier;
use ratatui::{
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    prelude::{Buffer, Widget},
    style::{self, Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, TableState},
};
use style::palette::tailwind;
use tui_textarea::{CursorMove, TextArea};

const INFO_TEXT: [&str; 3] = [
    "",
    "(Esc) Quit without saving | (Tab) Move to the next field | (Enter) Save and quit",
    "",
];

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
                self.flash_inputs();
                self.flash_values();
                self.current_input = ActiveInput::CronNotation;
            }
            KeyCode::Enter => {
                if validate(&mut self.cron_notation) {
                    if self.is_new {
                        cron_jobs.push(self.create_new_cron());
                        table_state.select(Some(cron_jobs.len() - 1));
                    } else {
                        self.update_selected_cron(&mut cron_jobs[table_state.selected().unwrap()]);
                    }
                    *show_popup = false;
                }
            }
            KeyCode::Char('v') if ctrl_pressed => {
                self.handle_paste();
            }
            _ => match self.current_input {
                ActiveInput::CronNotation => {
                    let cron_input = &mut self.cron_notation;
                    let cron_value = &mut self.cron_notation_value;
                    if cron_input.input(key) {
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
        self.current_input = ActiveInput::CronNotation;
        self.initial_render();
    }

    pub fn init(&mut self, cron_jobs: &mut Vec<CronJob>, table_state: &mut TableState) {
        self.flash_inputs();
        self.flash_values();
        self.current_input = ActiveInput::CronNotation;
        self.initial_render();

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

    fn initial_render(&mut self) {
        let cron_input = &mut self.cron_notation;
        let job_input = &mut self.job;
        let description_input = &mut self.job_description;

        cron_input.set_placeholder_text("Enter a cron notation");
        cron_input.set_cursor_line_style(Style::default());
        cron_input.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightCyan)),
        );

        job_input.set_placeholder_text("Enter a job");
        job_input.set_cursor_line_style(Style::default());

        description_input.set_placeholder_text("Enter a description");
        description_input.set_cursor_line_style(Style::default());
    }

    pub fn render_inputs(&mut self, area: Rect, buf: &mut Buffer) {
        let area = popup_area(area, 70);
        Widget::render(Clear, area, buf);

        let layout = Layout::vertical([Constraint::Length(17), Constraint::Length(3)])
            .flex(Flex::SpaceBetween);

        let [main_area, footer_area] = layout.areas(area);

        let main_block = Block::default()
            .style(Style::default().bg(tailwind::BLUE.c950))
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Color::LightBlue);

        Widget::render(main_block, main_area, buf);

        let main = Layout::vertical([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(2)
        .flex(Flex::Start);
        let [title_area, cron_notation_area, job_area, description_area] = main.areas(main_area);

        let footer = Layout::vertical([Constraint::Length(3)]);
        let [info_area] = footer.areas(footer_area);

        let selected_cron_notation = convert_to_human_readable(self.cron_notation_value.as_str());

        let wrapped_text: Vec<Line> = selected_cron_notation
            .chars()
            .collect::<Vec<_>>()
            .chunks(100)
            .map(|chunk| {
                Line::from(Span::styled(
                    chunk.iter().collect::<String>(),
                    Style::default().fg(Color::LightBlue),
                ))
            })
            .collect();

        let title = Paragraph::new(Text::from(wrapped_text))
            .style(Style::default().fg(Color::LightBlue))
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Style::default().fg(Color::LightBlue)),
            );
        Widget::render(title, title_area, buf);

        let info = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(Style::default().bg(tailwind::SLATE.c800))
            .centered()
            .block(Block::default());
        Widget::render(info, info_area, buf);

        let cron_input: &mut TextArea<'_> = &mut self.cron_notation;
        let job_input = &mut self.job;
        let description_input = &mut self.job_description;

        match self.current_input {
            ActiveInput::CronNotation => {
                if validate(cron_input) {
                    cron_input.set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::LightGreen))
                            .title("Cron notation* (OK)"),
                    );
                    cron_input.set_cursor_style(Style::default().bg(Color::LightGreen));
                } else {
                    cron_input.set_block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(Color::LightRed))
                            .title("Cron notation* (Invalid cron syntax)"),
                    );
                    cron_input.set_cursor_style(Style::default().bg(Color::LightRed));
                }
                cron_input.render(cron_notation_area, buf);

                job_input.set_cursor_style(Style::default());
                job_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .title("Job"),
                );
                job_input.render(job_area, buf);

                description_input.set_cursor_style(Style::default());
                description_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .title("Description"),
                );
                description_input.render(description_area, buf);
            }
            ActiveInput::Job => {
                job_input.set_cursor_style(Style::default().bg(Color::White));
                job_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan))
                        .title("Job"),
                );
                job_input.render(job_area, buf);

                cron_input.set_cursor_style(Style::default());
                cron_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .title("Cron notation*"),
                );
                cron_input.set_style(Style::default());
                cron_input.render(cron_notation_area, buf);

                description_input.set_cursor_style(Style::default());
                description_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .title("Description"),
                );
                description_input.render(description_area, buf);
            }
            ActiveInput::JobDescription => {
                description_input.set_cursor_style(Style::default().bg(Color::White));
                description_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::LightCyan))
                        .title("Description"),
                );
                description_input.render(description_area, buf);

                cron_input.set_cursor_style(Style::default());
                cron_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .title("Cron notation*"),
                );
                cron_input.set_style(Style::default());
                cron_input.render(cron_notation_area, buf);

                job_input.set_cursor_style(Style::default());
                job_input.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Gray))
                        .title("Job"),
                );
                job_input.render(job_area, buf);
            }
        }
    }
}

fn popup_area(area: Rect, percent_x: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Max(20)]).flex(Flex::Center);
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
        false
    } else {
        textarea.set_style(Style::default().fg(Color::LightGreen));
        true
    }
}
