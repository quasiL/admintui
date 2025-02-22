use crate::cron::CronTable;
use crate::ftp::FtpTable;
use crate::menu::MainMenu;
use crate::mysql::Mysql;
use ratatui::style::{self, Style};
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent},
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, Paragraph, Widget},
    Terminal,
};
use std::io::Error;
use style::palette::tailwind;

pub struct App {
    screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::MainMenu(MainMenu::new()),
        }
    }
}

pub enum Screen {
    MainMenu(MainMenu),
    CronTable(CronTable),
    FtpTable(FtpTable),
    Mysql(Mysql),
    Quit,
}

pub trait ScreenTrait {
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn handle_screen(&mut self, key: event::KeyEvent, _mouse: Option<MouseEvent>)
        -> Option<Screen>;

    fn new() -> Self
    where
        Self: Sized;

    fn render_footer(&mut self, area: Rect, buf: &mut Buffer, keybinds: Vec<(&str, &str)>) {
        let mut spans: Vec<Span> = Vec::new();
        let mut lines: Vec<Line> = Vec::new();

        let mut current_width = 0;
        let max_width = area.width.saturating_sub(4);

        for (key, desc) in &keybinds {
            let key_span = Span::styled(*key, Style::new().fg(tailwind::GRAY.c400));
            let desc_span = Span::styled(*desc, Style::new().fg(tailwind::GRAY.c500));
            let spacing = Span::raw("  ");

            let pair_width = key.len() as u16 + 1 + desc.len() as u16 + 2;

            if current_width + pair_width > max_width && !spans.is_empty() {
                lines.push(Line::from(spans));
                spans = Vec::new();
                current_width = 0;
            }

            spans.push(key_span);
            spans.push(Span::raw(" "));
            spans.push(desc_span);
            spans.push(spacing);

            current_width += pair_width;
        }

        if !spans.is_empty() {
            lines.push(Line::from(spans));
        }

        let info_footer = Paragraph::new(Text::from(lines))
            .style(Style::new().bg(tailwind::SLATE.c800))
            .centered()
            .block(Block::default());

        Widget::render(info_footer, area, buf);
    }
}

impl Screen {
    fn as_trait(&mut self) -> Option<&mut dyn ScreenTrait> {
        match self {
            Screen::MainMenu(menu) => Some(menu),
            Screen::CronTable(cron) => Some(cron),
            Screen::FtpTable(ftp) => Some(ftp),
            Screen::Mysql(mysql) => Some(mysql),
            Screen::Quit => None,
        }
    }
}

impl Widget for &mut Screen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Screen::MainMenu(menu) => menu.render(area, buf),
            Screen::CronTable(cron) => cron.render(area, buf),
            Screen::FtpTable(ftp) => ftp.render(area, buf),
            Screen::Mysql(mysql) => mysql.render(area, buf),
            Screen::Quit => (),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<(), Error> {
        loop {
            terminal.draw(|frame| frame.render_widget(&mut self.screen, frame.area()))?;
            match &mut self.screen {
                Screen::Quit => break,
                _ => (),
            }
            match event::read()? {
                Event::Key(key) => {
                    self.handle_event(key.into(), None);
                }
                Event::Mouse(mouse_event) => {
                    let dummy_key_event =
                        KeyEvent::new(KeyCode::Null, event::KeyModifiers::empty());
                    self.handle_event(dummy_key_event, Some(mouse_event));
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, key: event::KeyEvent, mouse: Option<MouseEvent>) {
        if let Some(screen_trait) = self.screen.as_trait() {
            if let Some(new_screen) = screen_trait.handle_screen(key, mouse) {
                self.screen = new_screen;
            }
        }
    }
}
