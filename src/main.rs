use crossterm::event::{MouseButton, MouseEvent};
use event::EventHandler;
use ratatui::backend::CrosstermBackend;
use tui::Tui;

mod game;
mod event;
mod update;
mod tui;
mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut game = game::GameState::new();

    let mut backend = CrosstermBackend::new(std::io::stderr());

    let mut terminal = ratatui::Terminal::new(backend).unwrap();

    let events = EventHandler::new(500);

    let mut tui = Tui::new(terminal, events);
    let mut dir = game.dir.clone();
    tui.enter().unwrap();

    while !game.game_over {
        // implement todo draw
        tui.draw(&mut game);

        // handle events

        match tui.events.next().unwrap() {
            event::Event::Key(key) => {
                dir = update::update(&mut game, key);
            }

            event::Event::Tick => { 
                game.update_snake(dir.clone());

                
            }
      
            event::Event::Resize(w, h) => {
                // implement todo resize
            }
        }

    }
    tui.exit().unwrap();
    Ok(())

    
   
}
