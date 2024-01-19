use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, symbols::block};
use std::{error::Error, io};
use crate::tui::component::Component;

use super::MyTerminal;


pub struct Input {
    text: String,
}
impl Default for Input {
    fn default() -> Input {
        Input {
            text: String::new(),
        }
    }
}
impl Input {
    fn enter_char(&mut self, to_insert: char) {
        self.text.push(to_insert);
    }

    fn delete_char(&mut self) {
        self.text.pop();
    }

    fn move_cursor_left(&self) {}

    fn move_cursor_right(&self) {}

    fn ui(&self,f: &mut Frame) {
        let block = Block::new().borders(Borders::all()).title("INPUT");
        let input = Paragraph::new(self.text.clone());
        f.set_cursor(self.text.len() as u16 + 1 , 1);
        let mut size = f.size();
        size.height = 3;
        f.render_widget(input.block(block), size);
    }
}
impl Component for Input {
    fn run(
        &mut self,
        terminal: &mut MyTerminal,
    ) -> Result<(), Box<dyn Error>> {
        terminal.draw(|f| self.ui(f))?;

        let Event::Key(key) = event::read()? else { return Ok(()) };
        match key.code {
            //KeyCode::Enter => app.submit_message(),
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
        Ok(())
    }
}

