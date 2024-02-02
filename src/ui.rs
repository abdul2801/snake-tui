use std::{clone, io::Error};

use ratatui::{layout::Constraint, style::{Color, Style, Stylize}, widgets::{Block, Borders, Cell, Paragraph, Row, Table}, Frame};

use crate::{game::GameState};
use std::borrow::Cow;





pub fn render(state: &GameState, f : &mut Frame) -> Result<(), Error> {
   
        // create a grid of width and height from state

        let snake = "🟥";
        let food = "🍎";

        let mut grid  = vec![vec![
            Cell::new("🟨").style(Style::default().fg(Color::Yellow)); state.board.0 as usize]; state.board.1 as usize];

        

        // add snake to grid
        for p in &state.snake {
            grid[p.y as usize][p.x as usize] = Cell::new(snake).style(Style::default().fg(Color::Red));
        }

        // add food to grid
        grid[state.food.y as usize][state.food.x as usize] = Cell::new(food).style(Style::default().fg(Color::Green));

        // combine grid into a a Row container


        let rows = grid.iter().map(|row| Row::new(row.iter().cloned()));



        


        let table = Table::new(rows, [Constraint::Length(2); 20 as usize]).block(Block::default().borders(Borders::ALL).title(state.dir.to_string()));

        // render the table to the center of the screen

        f.render_widget(table, f.size());





        

       

       
            
        

        
    Ok(())
}