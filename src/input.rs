use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

/// Represents the different game actions triggered by key presses.
#[derive(Debug, Clone, Copy)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    Rotate,
    SoftDrop, // Move down faster
    HardDrop, // Instantly drop
    Quit,
    None, // No specific action, used for heartbeat or unhandled keys
}

pub fn receive_input() -> io::Result<GameAction> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let action = match key_event.code {
                    KeyCode::Left => GameAction::MoveLeft,
                    KeyCode::Right => GameAction::MoveRight,
                    KeyCode::Up | KeyCode::Char('w') => GameAction::Rotate, // 'w' for WASD support
                    KeyCode::Down | KeyCode::Char('s') => GameAction::SoftDrop, // 's' for WASD support
                    KeyCode::Char(' ') => GameAction::HardDrop, // Spacebar for hard drop
                    KeyCode::Char('q') | KeyCode::Esc => GameAction::Quit, // 'q' or Esc to quit
                    _ => GameAction::None, // Unhandled key
                };
                Ok(action)
            }
            _ => {
                Ok(GameAction::None)
            }
        }
    } else {
        Ok(GameAction::None)
    }
}
