use std::{error::Error, io::Stdout};

use ratatui::{Terminal, backend::{Backend, CrosstermBackend}};

use super::MyTerminal;


pub trait Component {
    async fn run(
        &mut self,
        terminal: &mut MyTerminal,
    ) -> Result<(), Box<dyn Error>>;
}
