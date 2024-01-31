use std::{collections::VecDeque, vec};

use rand::Rng;

#[derive(PartialEq, Eq,Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x:i32,y:i32) -> Point {
        Self {
            x,
            y
        }
    }
}

pub struct GameState {
    board : (i32, i32),
    snake : VecDeque<Point>,
    food : Point,
    pub dir : Direction,
    score : i32,
    pub game_over : bool,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(dir : &Direction) -> Direction {
        match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    


    
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            board  : (16, 16),
            dir : Direction::Right,
            snake : vec![Point { x: 0, y: 0 }].into_iter().collect(),
            food : Point { x: 5, y: 0 },
            score : 0,
            game_over : false,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_head(&self) -> &Point {
        self.snake.front().unwrap()
    }

    pub fn is_valid_border(&self) -> bool {
        let head = self.get_head();

        if head.x < 0 || head.x >= self.board.0 || head.y < 0 || head.y >= self.board.1 {
            return false;
        }
        true

    }



    pub fn snake_overlap(&self, new_head : &Point) -> bool{
        for p in &self.snake {
            if p.x == new_head.x && p.y == new_head.y {
                return true;
            }

        }
        false
    }


    fn gen_apple(& mut self) {
        let mut rng = rand::thread_rng();


        self.food = loop {
            let x= rng.gen_range(0..=self.board.0);
            let y: i32= rng.gen_range(0..=self.board.1);
            let apple = Point::new(x,y);

            if !self.snake.contains(&apple) {
                break apple;
            }
            
        }
    }

    fn check_apple(& mut self) -> bool{
        if self.get_head() == &self.food {
            self.score += 1;
            self.gen_apple();

            return true;
        }
        false


    }

    pub fn update_snake(& mut self, mut dir : Direction) {
        let cur_head = self.get_head();
        let mut new_head = Point::new(cur_head.x,cur_head.y);

        if self.dir == Direction::opposite(&self.dir) {
            dir = self.dir.clone();
        }

        match dir {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y +=1,
            Direction::Left => new_head.x -=1,
            Direction::Right => new_head.x +=1,
        }

        if !self.is_valid_border() || self.snake_overlap(&new_head) {
            self.game_over = true;
            return;
        }
        self.snake.push_front(new_head);
        if !self.check_apple() {
            self.snake.pop_back();
        }
        self.dir = dir;


        

    }



        

    }


// implement tests using the update_snake function
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_update_snake() {
        let game = GameState::new();
        
        println!("{:?}", game.snake.len());
        println!("{:?}", game.food);
        assert_eq!(1, 1);
            
        }
    }

