use std::{collections::HashMap, process::exit, sync::Arc, thread, time::Duration};

use anyhow::Error;
use crossterm::event::{self, Event, KeyCode};
use log::debug;
use ratatui::{
    layout::{self, Rect},
    prelude::{Constraint, Direction, Layout},
    widgets::{self, Block, Borders, Clear, ListState, Paragraph},
};
use tokio::sync::{mpsc::channel, Mutex};

use crate::{client::Client, server::Server, tui::component::Component};

use super::{input::Input, state::State};

pub struct App {
    client: Arc<Mutex<Client>>,
    state: Arc<Mutex<State>>,
    chat_input: Input,
    connect_input: Input,
}

impl App {
    pub async fn new(client: Arc<Mutex<Client>>, state: Arc<Mutex<State>>) -> Self {
        let (chat_tx, mut chat_rx) = channel::<String>(100);

        let client_ref = client.clone();
        tokio::spawn(async move {
            loop {
                let msg = chat_rx.recv().await.unwrap();
                let mut client = client_ref.lock().await;
                client.send(msg.as_str()).await.unwrap();
            }
        });
        let (connect_tx, mut connect_rx) = channel::<String>(100);

        let state_ref = state.clone();
        tokio::spawn(async move {
            loop {
                let conenction_string = connect_rx.recv().await.unwrap();
                let mut state = state_ref.lock().await;
                state.url = conenction_string;
            }
        });

        let mut chat_input = Input::new(chat_tx);
        chat_input.multi = true;
        App {
            client: client.clone(),
            state,
            chat_input,
            connect_input: Input::new(connect_tx),
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

        let input: Paragraph = self.chat_input.ui();

        let state = self.state.lock().await;

        let messages = state.messages.clone();
        let list = widgets::List::new(messages);
        let title = widgets::Paragraph::new(state.url.as_str());

        terminal
            .draw(|f| {
                let area = f.size();
                let layout = layout.split(area);
                f.render_widget(title, layout[0]);
                f.render_widget(list, layout[1]);
                f.render_widget(input, layout[2]);
                if self.connect_input.focussed {
                    let block = Block::default()
                        .title("conenction string")
                        .borders(Borders::ALL);
                    let input = self.connect_input.ui().block(block);
                    let area = centered_rect(60, 7, area);
                    f.render_widget(Clear, area); //this clears out the background
                    f.render_widget(input, area);
                }
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
            if !self.chat_input.focussed && !self.connect_input.focussed {
                let Event::Key(key) = event::read().unwrap() else { return Ok(()) };
                match key.code {
                    KeyCode::Char('i') => self.chat_input.focussed = true,
                    KeyCode::Char('c') => self.connect_input.focussed = true,
                    KeyCode::Char('q') => Err(Error::msg("exit"))?,
                    _ => {}
                }
            }
        }

        self.chat_input.run(terminal).await?;
        self.connect_input.run(terminal).await?;

        self.render(terminal).await?;

        Ok(())
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ],
    )
    .split(r);

    Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ],
    )
    .split(popup_layout[1])[1]
}
