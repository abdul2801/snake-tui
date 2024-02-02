
use crossterm::event::KeyEvent;

use crate::game::{GameState, Direction};




pub fn update(state: &mut GameState, key : KeyEvent) -> Direction {
    
    match key.code {
        crossterm::event::KeyCode::Char('w') => {
            return Direction::Up;
        },
        crossterm::event::KeyCode::Char('s') => {
            return Direction::Down;
        },
        crossterm::event::KeyCode::Char('a') => {
            return Direction::Left;
        },
        crossterm::event::KeyCode::Char('d') => {
            return Direction::Right;
        },
        crossterm::event::KeyCode::Char('q') => {
            state.game_over = true;
            return state.dir.clone();
        },
        _ => {
            return state.dir.clone();
        }
}
}