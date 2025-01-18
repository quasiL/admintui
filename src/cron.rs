use crate::app::Screen;
use crate::menu::MainMenu;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Flex, Layout, Margin, Rect},
    prelude::*,
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Cell, Clear, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap,
    },
};
use style::palette::tailwind;
use tui_textarea::TextArea;
use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
    "(Shift + →) next color | (Shift + ←) previous color",
];

const ITEM_HEIGHT: usize = 4;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

struct CronJob {
    cron_notation: String,
    next_execution: String,
    job_description: String,
}

impl CronJob {
    // Return references to the fields as an array
    const fn ref_array(&self) -> [&String; 3] {
        [
            &self.cron_notation,
            &self.next_execution,
            &self.job_description,
        ]
    }

    // Get the cron notation
    fn cron_notation(&self) -> &str {
        &self.cron_notation
    }

    // Get the next execution time
    fn next_execution(&self) -> &str {
        &self.next_execution
    }

    // Get the job description
    fn job_description(&self) -> &str {
        &self.job_description
    }
}

pub struct CronTable {
    state: TableState,
    items: Vec<CronJob>,
    longest_item_lens: (u16, u16, u16),
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
    show_popup: bool,
    is_valid: bool,
    inputs: Inputs,
    input1: Input,
    input2: Input,
    input3: Input,
    current_input: Input,
}

enum Input {
    CronNotation { input: TextArea<'static> },
    Job { input: TextArea<'static> },
    JobDescription { input: TextArea<'static> },
    None,
}

struct Inputs {
    cron_notation: TextArea<'static>,
    job: TextArea<'static>,
    job_description: TextArea<'static>,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            cron_notation: TextArea::default(),
            job: TextArea::default(),
            job_description: TextArea::default(),
        }
    }
}

impl Widget for &mut CronTable {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(area);

        self.set_colors();
        self.render_table(rects[0], buf);
        self.render_scrollbar(rects[0], buf);
        self.render_footer(rects[1], buf);

        if self.show_popup {
            self.render_popup(rects[0], buf);
        }
    }
}

impl CronTable {
    pub fn new() -> Self {
        let cron_jobs_vec = vec![
            CronJob {
                cron_notation: "0 5 * * *qqqqqqqqqqqqqqqqqqqqqqqqqqqqq".to_string(),
                next_execution: "2025-01-17 05:00".to_string(),
                job_description: "Backup Database".to_string(),
            },
            CronJob {
                cron_notation: "0 0 * * 0".to_string(),
                next_execution: "2025-01-21 00:00".to_string(),
                job_description: "Weekly System Update".to_string(),
            },
            CronJob {
                cron_notation: "30 8 * * 1-5".to_string(),
                next_execution: "2025-01-17 08:30".to_string(),
                job_description: "Send Email Reports".to_string(),
            },
            CronJob {
                cron_notation: "15 3 * * *".to_string(),
                next_execution: "2025-01-17 03:15".to_string(),
                job_description: "Cleanup Temp Files".to_string(),
            },
            CronJob {
                cron_notation: "0 12 * * *".to_string(),
                next_execution: "2025-01-17 12:00".to_string(),
                job_description: "Daily Database Sync".to_string(),
            },
            CronJob {
                cron_notation: "0 0 1 * *".to_string(),
                next_execution: "2025-01-17 00:00".to_string(),
                job_description: "Monthly Backup".to_string(),
            },
            CronJob {
                cron_notation: "0 0 1 1 *".to_string(),
                next_execution: "2025-02-01 00:00".to_string(),
                job_description: "Yearly Backup".to_string(),
            },
        ];
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&cron_jobs_vec),
            scroll_state: ScrollbarState::new((cron_jobs_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: cron_jobs_vec,
            show_popup: false,
            is_valid: false,
            inputs: Inputs::default(),
            input1: Input::CronNotation {
                input: TextArea::default(),
            },
            input2: Input::Job {
                input: TextArea::default(),
            },
            input3: Input::JobDescription {
                input: TextArea::default(),
            },
            current_input: Input::None,
        }
    }

    pub fn handle_event(&mut self, key: event::KeyEvent) -> Option<Screen> {
        let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
        if self.show_popup == true {
            let mut job = &mut self.input1;

            match &mut job {
                Input::CronNotation { input } => {
                    if input.input(key) {
                        validate(input);
                    }
                }
                _ => {}
            }

            match key {
                input => {
                    // if self.inputs.cron_notation.input(input) {
                    //     self.is_valid = validate(&mut self.inputs.cron_notation);
                    // }
                    // if self.inputs.job.input(input) {
                    //     self.is_valid = validate(&mut self.inputs.job);
                    // }
                    // if self.inputs.job_description.input(input) {
                    //     self.is_valid = validate(&mut self.inputs.job_description);
                    // }
                }
            }
            None
        } else {
            match key.code {
                KeyCode::Char('a') => Some(Screen::MainMenu(MainMenu::new())),
                KeyCode::Char('j') | KeyCode::Down => {
                    self.next_row();
                    None
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.previous_row();
                    None
                }
                KeyCode::Char('l') | KeyCode::Right if shift_pressed => {
                    self.next_color();
                    None
                }
                KeyCode::Char('h') | KeyCode::Left if shift_pressed => {
                    self.previous_color();
                    None
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    self.next_column();
                    None
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.previous_column();
                    None
                }
                KeyCode::Enter => {
                    self.show_popup = !self.show_popup;
                    None
                }
                _ => None,
            }
        }
    }

    pub fn next_row(&mut self) {
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

    pub fn previous_row(&mut self) {
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

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    fn render_table(&mut self, area: Rect, buf: &mut Buffer) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = ["Cron Notation", "Next Execution", "Description"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
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
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
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
        let footer_style = Style::default()
            .fg(self.colors.row_fg)
            .bg(self.colors.buffer_bg);

        let border_style = Style::default().fg(self.colors.footer_border_color);
        //let selected_state_info = format!("Selected index: {}", self.state.selected().unwrap());
        let selected_index = self.state.selected().unwrap_or(0); // Default to 0 if none selected
        let selected_cron = &self.items[selected_index];
        let selected_cron_description =
            format!("Selected cron: {}", selected_cron.job_description());

        let info_footer = Paragraph::new(Text::from_iter([selected_cron_description]))
            .style(footer_style)
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(border_style),
            );

        // Render the footer directly onto the buffer
        Widget::render(info_footer, area, buf);
    }

    fn render_popup(&mut self, area: Rect, buf: &mut Buffer) {
        // let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        // let [instructions, content] = vertical.areas(area);

        // let text = if self.show_popup {
        //     "Press p to close the popup"
        // } else {
        //     "Press p to show the popup"
        // };
        // let paragraph = Paragraph::new(text.slow_blink())
        //     .centered()
        //     .wrap(Wrap { trim: true });
        // paragraph.render(instructions, buf);

        // // Instead of a solid blue block, use a border with a different background or transparent block
        // let block = Block::bordered().style(Style::default().bg(Color::DarkGray));
        // block.render(content, buf);

        // let block = Block::bordered().title("Popup");
        // let area = popup_area(area, 60, 20);
        // Widget::render(Clear, area, buf);
        // block.render(area, buf);

        let area = popup_area(area, 80, 20);
        Widget::render(Clear, area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Color::LightGreen);
        block.render(area, buf);

        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);
        let [title, title_area, cron_area, description_area] = vertical.areas(area);

        let selected_index = self.state.selected().unwrap();
        let selected_cron = &self.items[selected_index];
        let selected_cron_description =
            format!("Selected cron: {}", selected_cron.job_description());

        let info_footer = Paragraph::new(Text::from_iter([selected_cron_description]))
            .style(Style::default().fg(Color::LightGreen))
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(Color::LightGreen),
            );

        Widget::render(info_footer, title, buf);

        if let Input::CronNotation { input } = &mut self.input1 {
            input.set_cursor_line_style(style::Style::default());
            input.set_placeholder_text("Enter a cron notation");
            input.render(title_area, buf);
        }

        self.inputs
            .cron_notation
            .set_cursor_line_style(style::Style::default());
        self.inputs
            .cron_notation
            .set_placeholder_text("Enter a description");
        self.inputs.cron_notation.render(cron_area, buf);

        self.inputs
            .job
            .set_cursor_line_style(style::Style::default());
        self.inputs.job.set_placeholder_text("Enter a job");
        self.inputs.job.render(description_area, buf);
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

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Max(12)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

// fn validate(textarea: &mut TextArea) -> bool {
//     if let Err(err) = textarea.lines()[0].parse::<f64>() {
//         textarea.set_style(Style::default().fg(Color::LightRed));
//         textarea.set_block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .border_style(Color::LightRed)
//                 .title(format!("ERROR: {}", err)),
//         );
//         false
//     } else {
//         textarea.set_style(Style::default().fg(Color::LightGreen));
//         textarea.set_block(
//             Block::default()
//                 .border_style(Color::LightGreen)
//                 .borders(Borders::ALL)
//                 .title("OK"),
//         );
//         true
//     }
// }

fn validate(textarea: &mut TextArea) -> bool {
    use cron::Schedule;
    use std::str::FromStr;

    let input = textarea.lines().get(0).map(|s| s.as_str()).unwrap_or("");

    if Schedule::from_str(input).is_err() {
        // If the cron expression is invalid, style the textarea with an error
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightRed))
                .title("ERROR: Invalid cron syntax"),
        );
        false
    } else {
        // If the cron expression is valid, style the textarea with success
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
