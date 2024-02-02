
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Error;

use crate::{event::EventHandler, ui};


pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;


pub struct Tui {
    pub terminal : CrosstermTerminal,
    pub events : EventHandler,
}

impl Tui {
    pub fn new(terminal : CrosstermTerminal, events : EventHandler) -> Tui {
        Self {
            terminal,
            events,
        }

    }

    pub fn enter(&mut self) -> Result<(), Error >{
        terminal::enable_raw_mode().expect("failed to enable raw mode");
        crossterm::execute!(
            std::io::stderr(),
            EnterAlternateScreen,
        ).expect("failed to enter alternate screen");

        self.terminal.hide_cursor().expect("failed to hide cursor");
        self.terminal.clear().expect("failed to clear terminal");
        Ok(())

    }
    pub fn draw(&mut self , mut state : &crate::game::GameState) {
        self.terminal.draw(|f| { ui::render(&mut state, f).unwrap();}).unwrap();
    }

    pub fn exit(&mut self) -> Result<(), Error> {
        terminal::disable_raw_mode().expect("failed to disable raw mode");
        crossterm::execute!(
            std::io::stderr(),
            LeaveAlternateScreen,
        ).expect("failed to leave alternate screen");

        self.terminal.show_cursor().expect("failed to show cursor");
        Ok(())
    }
}

