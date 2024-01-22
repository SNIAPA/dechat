use std::{collections::HashMap, process::exit, sync::{Arc, mpsc::channel}, thread};

use crossterm::event::{self, Event, KeyCode};
use tokio::sync::Mutex;

use crate::{client::Client, tui::component::Component};

use super::input::Input;

pub struct App {
    client: Arc<Mutex<Client>>,
    input: Input,
}

impl App {
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        let (tx, rx) = channel::<String>();

        let client2 = client.clone();
        tokio::spawn(async move {
            let mut client = client2.lock().await;
            loop {
                let msg = rx.recv().unwrap();
                client.send(msg.as_str()).unwrap();
            };
        });

        App {
            client,
            input: Input::new(tx),
        }
    }
}

impl Component for App {
    fn run(&mut self, terminal: &mut super::MyTerminal) -> Result<(), Box<dyn std::error::Error>> {
        if !self.input.focussed {
            let Event::Key(key) = event::read()? else { return Ok(()) };
            match key.code {
                KeyCode::Char('i') => self.input.focussed = true,
                KeyCode::Char('q') => exit(1),
                _ => {}
            }
        }
        self.input.run(terminal)?;
        Ok(())
    }
}
