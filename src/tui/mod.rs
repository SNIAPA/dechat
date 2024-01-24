use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::{
    error::Error,
    io::{self, Stdout},
    sync::Arc, time::Duration,
};
use tokio::sync::Mutex;

use crate::{client::Client, server::Server};

use self::{app::App, component::Component, input::Input, state::State};

pub mod app;
pub mod component;
pub mod input;
pub mod state;

type MyTerminal = Terminal<CrosstermBackend<Stdout>>;

pub async fn tui(client: Arc<Mutex<Client>>, state: Arc<Mutex<State>>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(client, state).await;

    let err = run(app, &mut terminal).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    err
}

pub async fn run(mut app: App, terminal: &mut MyTerminal) -> Result<(), Box<dyn Error>> {
    loop {
        tokio::time::sleep(Duration::from_millis(100)).await;
        app.run(terminal).await?;
    }
}
