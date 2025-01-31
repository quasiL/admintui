use crate::cron::CronTable;
use crate::menu::MainMenu;
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    crossterm::event::{self, Event},
    layout::Rect,
    widgets::Widget,
    Terminal,
};
use std::io::Error;

pub struct App {
    screen: Screen,
}

pub enum Screen {
    MainMenu(MainMenu),
    CronTable(CronTable),
    Quit,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::MainMenu(MainMenu::new()),
        }
    }
}

impl Widget for &mut Screen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Screen::MainMenu(menu) => menu.render(area, buf),
            Screen::CronTable(cron) => cron.render(area, buf),
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
            if let Event::Key(key) = event::read()? {
                self.handle_event(key.into());
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, key: event::KeyEvent) {
        match &mut self.screen {
            Screen::MainMenu(menu) => {
                if let Some(menu_screen) = menu.handle_screen(key) {
                    self.screen = menu_screen;
                }
            }
            Screen::CronTable(cron) => {
                if let Some(cron_screen) = cron.handle_screen(key) {
                    self.screen = cron_screen;
                }
            }
            _ => (),
        }
    }
}
