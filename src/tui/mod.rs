use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::{error::Error, io::{self, Stdout}};

use self::{input::Input, component::Component};

pub mod input;
pub mod app;
pub mod component;

type MyTerminal = Terminal<CrosstermBackend<Stdout>>;

pub fn tui(hostname: &str) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut input = Input::default();
    //let components = vec![input];

    run(input,&mut terminal)?;

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


pub fn run (mut input: Input, terminal: &mut MyTerminal) -> Result<(), Box<dyn Error>>{
    loop {
        input.run(terminal)?;
    }
}
