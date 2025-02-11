use crate::cron::CronTable;
use crate::ftp::FtpTable;
use crate::menu::MainMenu;
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, MouseEvent},
    layout::Rect,
    widgets::Widget,
    Terminal,
};
use std::io::Error;

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
    Quit,
}

pub trait ScreenTrait {
    fn render(&mut self, area: Rect, buf: &mut Buffer);
    fn handle_screen(&mut self, key: event::KeyEvent, _mouse: Option<MouseEvent>)
        -> Option<Screen>;

    fn new() -> Self
    where
        Self: Sized;
}

impl Screen {
    fn as_trait(&mut self) -> Option<&mut dyn ScreenTrait> {
        match self {
            Screen::MainMenu(menu) => Some(menu),
            Screen::CronTable(cron) => Some(cron),
            Screen::FtpTable(ftp) => Some(ftp),
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
