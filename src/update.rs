
use crossterm::event::KeyEvent;

use crate::game::{GameState, Direction};




pub fn update(state: &mut GameState, key : KeyEvent) {

    match key.code {
        crossterm::event::KeyCode::Char('w') => {
            state.update_snake(Direction::Up);
        },
        crossterm::event::KeyCode::Char('s') => {
            state.update_snake(Direction::Down);
        },
        crossterm::event::KeyCode::Char('a') => {
            state.update_snake(Direction::Left);
        },
        crossterm::event::KeyCode::Char('d') => {
            state.update_snake(Direction::Right);
        },
        crossterm::event::KeyCode::Char('q') => {
            state.game_over = true;
        },
        _ => {}
}
}