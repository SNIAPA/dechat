use crate::tui::component::Component;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use ratatui::{prelude::*, symbols::block, widgets::*};
use std::{alloc::System, error::Error, io, process::exit, task::Wake, time::Duration};
use tokio::sync::mpsc::Sender;

use super::MyTerminal;

pub struct Input {
    msg_send: Sender<String>,
    text: String,
    pub focussed: bool,
    pub multi: bool,
}
impl Input {
    pub fn new(send_tx: Sender<String>) -> Input {
        Input {
            msg_send: send_tx,
            text: String::new(),
            focussed: false,
            multi: false,
        }
    }
    fn enter_char(&mut self, to_insert: char) {
        self.text.push(to_insert);
    }

    fn delete_char(&mut self) {
        self.text.pop();
    }

    fn move_cursor_left(&self) {}

    fn move_cursor_right(&self) {}

    pub fn ui(&self) -> ratatui::widgets::Paragraph {
        let title = if self.focussed { "INPUT" } else { "input" };
        let block = Block::new().borders(Borders::all()).title(title);
        let input = Paragraph::new(self.text.clone());
        return input.block(block);
    }
}
impl Component for Input {
    async fn run(&mut self, terminal: &mut MyTerminal) -> Result<(), Box<dyn Error>> {
        terminal.set_cursor(self.text.len() as u16 + 1, 1)?;

        if event::poll(Duration::from_millis(1)).unwrap() {
            if self.focussed {
                match event::read().unwrap() {
                    Event::Paste(text) => self.text += text.as_str(),
                    Event::Key(key) => match key.code {
                        KeyCode::Esc => self.focussed = false,
                        KeyCode::Enter => {
                            self.msg_send.send(self.text.clone()).await.unwrap();
                            self.text.clear();
                            if !self.multi {
                                self.focussed = false
                            }
                        }
                        KeyCode::Char(to_insert) => {
                            self.enter_char(to_insert);
                        }
                        KeyCode::Backspace => {
                            self.delete_char();
                        }
                        KeyCode::Left => {
                            self.move_cursor_left();
                        }
                        KeyCode::Right => {
                            self.move_cursor_right();
                        }
                        _ => {}
                    },
                    _ => return Ok(()),
                }
            }
        }
        Ok(())
    }
}
