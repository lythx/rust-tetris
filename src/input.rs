use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
};
use std::{
    io,
    time::Duration,
};

/// Represents the different game actions triggered by key presses.
#[derive(Debug, Clone, Copy)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Rotate,
    SoftDrop,
    HardDrop,
    Quit,
    None,
}

pub fn receive_input() -> io::Result<Action> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let action = match key_event.code {
                    KeyCode::Left | KeyCode::Char('a')  => Action::MoveLeft,
                    KeyCode::Right | KeyCode::Char('d') => Action::MoveRight,
                    KeyCode::Up | KeyCode::Char('w') => Action::Rotate,
                    KeyCode::Down | KeyCode::Char('s') => Action::SoftDrop,
                    KeyCode::Char(' ') => Action::HardDrop,
                    KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                    _ => Action::None,
                };
                Ok(action)
            }
            _ => {
                Ok(Action::None)
            }
        }
    } else {
        Ok(Action::None)
    }
}
