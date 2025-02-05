use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod config;
mod cron;
mod menu;
use app::App;

/// The main entry point of the application.
/// Initializes the terminal, runs the application, and restores the terminal to its original state.
fn main() {
    let mut terminal = init(CrosstermBackend::new(io::stdout())).unwrap();

    match App::default().run(&mut terminal) {
        Ok(_) => {}
        Err(_err) => {
            panic!("Something went wrong");
        }
    }

    restore(&mut terminal).unwrap();
}

/// Initializes the terminal with the specified backend.
///
/// # Arguments
/// - `backend`: The backend implementing the `Backend` trait to be used for the terminal.
///
/// # Returns
/// - `Ok(Terminal<B>)`: A `Terminal` instance configured with the given backend.
/// - `Err(Box<dyn Error>)`: If initialization fails.
///
/// # Errors
/// This function can fail if entering alternate screen mode, enabling raw mode,
/// or initializing the terminal backend fails.
fn init<B: Backend>(backend: B) -> Result<Terminal<B>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    Ok(terminal)
}

/// Restores the terminal to its original state.
///
/// # Arguments
/// - `terminal`: A mutable reference to the terminal instance to be restored.
///
/// # Returns
/// - `Ok(())`: If the terminal is successfully restored.
/// - `Err(Box<dyn Error>)`: If any restoration step fails.
///
/// # Errors
/// This function can fail if disabling raw mode, leaving alternate screen mode or showing the cursor fails.
fn restore<B>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>>
where
    B: Backend + io::Write,
{
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
