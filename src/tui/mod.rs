use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::{
    error::Error,
    io::{self, Stdout},
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::client::Client;

use self::{app::App, component::Component, input::Input};

pub mod app;
pub mod component;
pub mod input;

type MyTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn tui(client: Arc<Mutex<Client>>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(client);

    run(app, &mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn run(mut app: App, terminal: &mut MyTerminal) -> Result<(), Box<dyn Error>> {
    loop {
        app.run(terminal)?;
    }
}
