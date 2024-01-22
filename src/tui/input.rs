use crate::tui::component::Component;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, symbols::block, widgets::*};
use std::{alloc::System, error::Error, io, process::exit, task::Wake, sync::mpsc::Sender};

use super::MyTerminal;


pub struct Input {
    msg_send: Sender<String>,
    text: String,
    pub focussed: bool,
}
impl Input {
    pub fn new(send_tx: Sender<String>) -> Input {
        Input {
            msg_send: send_tx,
            text: String::new(),
            focussed: false,
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

    fn ui(&self, f: &mut Frame) {
        let title = if self.focussed { "INPUT" } else { "input" };
        let block = Block::new().borders(Borders::all()).title(title);
        let input = Paragraph::new(self.text.clone());
        f.set_cursor(self.text.len() as u16 + 1, 1);
        let mut size = f.size();
        size.height = 3;
        f.render_widget(input.block(block), size);
    }
}
impl Component for Input {
    fn run(&mut self, terminal: &mut MyTerminal) -> Result<(), Box<dyn Error>> {
        if self.focussed {
            let Event::Key(key) = event::read()? else { return Ok(()) };
            match key.code {
                KeyCode::Esc => self.focussed = false,
                KeyCode::Enter => {
                    self.msg_send.send(self.text.clone()).unwrap();
                },
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
            }
        }
        terminal.draw(|f| self.ui(f))?;
        Ok(())
    }
}
