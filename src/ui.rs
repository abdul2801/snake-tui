use std::io::Error;

use ratatui::{layout::{Constraint, Layout}, widgets::{Block, Borders}, Frame};

use crate::{game::GameState};





pub fn render(state: &GameState, f : &mut Frame) -> Result<(), Error> {
   
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let block = Block::default().title("Block").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
 

    Ok(())
}