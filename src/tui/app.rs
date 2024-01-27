use std::{collections::HashMap, process::exit, sync::Arc, thread, time::Duration};

use anyhow::Error;
use crossterm::event::{self, Event, KeyCode};
use log::debug;
use ratatui::{
    layout::{self, Rect},
    prelude::{Constraint, Direction, Layout},
    widgets::{self, Block, Borders, ListState, Paragraph, Clear},
};
use tokio::sync::{mpsc::channel, Mutex};

use crate::{client::Client, server::Server, tui::component::Component};

use super::{input::Input, state::State};

pub struct App {
    client: Arc<Mutex<Client>>,
    state: Arc<Mutex<State>>,
    input: Input,
    connect_popup: bool,
}

impl App {
    pub async fn new(client: Arc<Mutex<Client>>, state: Arc<Mutex<State>>) -> Self {
        let (tx, mut rx) = channel::<String>(100);

        let client2 = client.clone();
        tokio::spawn(async move {
            loop {
                let msg = rx.recv().await.unwrap();
                let mut client = client2.lock().await;
                client.send(msg.as_str()).await.unwrap();
            }
        });

        App {
            client: client.clone(),
            state,
            input: Input::new(tx),
            connect_popup: false,
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
                if self.connect_popup {
                    let block = Block::default().title("Popup").borders(Borders::ALL);
                    let area = centered_rect(60, 20, area);
                    f.render_widget(Clear, area); //this clears out the background
                    f.render_widget(block, area);
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
            if !self.input.focussed {
                let Event::Key(key) = event::read().unwrap() else { return Ok(()) };
                match key.code {
                    KeyCode::Char('i') => self.input.focussed = true,
                    KeyCode::Char('c') => self.connect_popup = true,
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
