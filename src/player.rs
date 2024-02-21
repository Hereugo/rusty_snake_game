use std::collections::VecDeque;
use std::mem::swap;

use crate::food::Food;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
pub struct BodyPart {
    pub direction: Direction,
    pub x: u16,
    pub y: u16,
}

impl BodyPart {
    pub fn new(x: u16, y: u16, direction: Direction) -> BodyPart {
        BodyPart { x, y, direction }
    }

    pub fn move_next(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

pub struct Player {
    pub direction: Direction,
    pub body: VecDeque<BodyPart>,
}

impl Player {
    pub fn new(width: u16, height: u16) -> Player {
        Player {
            direction: Direction::Right,
            body: vec![
                BodyPart::new(width / 2 + 1, height / 2, Direction::Right),
                BodyPart::new(width / 2 + 2, height / 2, Direction::Right),
                BodyPart::new(width / 2 + 3, height / 2, Direction::Right),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn turn(&mut self, direction: Direction) {
        match (direction, self.direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => (),
            _ => self.direction = direction,
        }
    }

    pub fn update(&mut self, food: &mut Food) {
        let tail = self.body.front().unwrap().clone();

        let mut last_direction = self.direction;
        for part in self.body.iter_mut().rev() {
            part.move_next();
            swap(&mut part.direction, &mut last_direction);
        }

        let head = self.body.back().unwrap();
        
        // Checking if player has eaten a fruit
        if head.x == food.x && head.y == food.y {
            food.is_eaten = true;
            self.body.push_front(tail);
        }
    }
}
