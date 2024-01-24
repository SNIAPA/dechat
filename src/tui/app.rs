use std::{
    collections::HashMap,
    process::exit,
    sync::{mpsc::channel, Arc},
    thread,
    time::Duration,
};

use anyhow::Error;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout,
    prelude::{Constraint, Direction, Layout},
    widgets::{self, ListState, Paragraph},
};
use tokio::sync::Mutex;

use crate::{client::Client, server::Server, tui::component::Component};

use super::{input::Input, state::State};

pub struct App {
    client: Arc<Mutex<Client>>,
    state: Arc<Mutex<State>>,
    input: Input,
    url: String,
}

impl App {
    pub async fn new(client: Arc<Mutex<Client>>, state: Arc<Mutex<State>>) -> Self {
        let (tx, rx) = channel::<String>();

        let client2 = client.clone();
        tokio::spawn(async move {
            loop {
                let msg = rx.recv().unwrap();
                let mut client = client2.lock().await;
                client.send(msg.as_str()).await.unwrap();
            }
        });

        App {
            client: client.clone(),
            state,
            input: Input::new(tx),
            url: client.lock().await.url.clone(),
        }
    }

    async fn render(
        &mut self,
        terminal: &mut super::MyTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(3),
            ],
        );

        let input: Paragraph = self.input.ui();

        dbg!("lock");
        let messages = self.state.lock().await.messages.clone();
        dbg!("unlock");
        let list = widgets::List::new(messages);

        let title = widgets::Paragraph::new(self.url.as_str());

        terminal
            .draw(|f| {
                let layout = layout.split(f.size());
                f.render_widget(title, layout[0]);
                f.render_widget(list, layout[1]);
                f.render_widget(input, layout[2]);
            })
            .unwrap();
        Ok(())
    }
}

impl Component for App {
    async fn run(
        &mut self,
        terminal: &mut super::MyTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(1)).unwrap() {
            if !self.input.focussed {
                let Event::Key(key) = event::read().unwrap() else { return Ok(()) };
                match key.code {
                    KeyCode::Char('i') => self.input.focussed = true,
                    KeyCode::Char('q') => Err(Error::msg("exit"))?,
                    _ => {}
                }
            }
        }

        self.input.run(terminal).await?;

        self.render(terminal).await?;

        Ok(())
    }
}
