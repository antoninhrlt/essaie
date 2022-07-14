// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::{
    char,
    fmt,
};

/// Basically it permits to structure what is really a position on a board
///
/// - The `x` axis is letters from 'a' to 'h'
/// - The `y` axis is numbers from 1 to 8
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    x: char,
    y: u32,
}

impl Position {
    pub fn new(x: char, y: u32) -> Self {
        Self { x, y }
    }

    /// Obtains a structured position from a dumb i32 index (used to walk
    /// through the board's squares)
    pub fn from_u32(i: u32) -> Option<Self> {
        let x_as_i32: u32 = i - 8 * (i / 8) + 96;

        let x = char::from_u32(x_as_i32).unwrap();
        if !('a'..'i').contains(&x) {
            return None;
        }

        let y = i / 8 + 1;
        
        Some(Self {
            x,
            y,
        })
    }

    pub fn to_u32(&self) -> u32 {
        ((self.y - 1) * 8 + (self.x as u32 - 96))
            .try_into()
            .unwrap()
    }

    /// Destructure the position into a dumb index (used to walk through the
    /// board's squares)
    pub fn to_index(&self) -> usize {
        (self.to_u32() - 1)
            .try_into()
            .unwrap()
    }

    /// Give a vector of all the positions around this position
    pub fn positions_around(&self) -> Vec<Position> {
        let mut around = vec![];
        
        let x = self.x as u32;

        for y in self.y - 1..self.y + 2 {
            for position in vec![
                Position::new(char::from_u32(x - 1).unwrap(), y), 
                Position::new(self.x, y),
                Position::new(char::from_u32(x + 1).unwrap(), y)
            ] {
                if ('a'..'i').contains(&position.x()) 
                    && position.y() > 0 
                    && position != *self 
                {
                    around.push(position)
                }
            }
        }

        around
    }

    pub fn set_x(&mut self, x: char) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: u32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn x(&self) -> char {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.x(), self.y())
    }
}

#[test]
fn position_from_u32() {
    //   a b c d e f g h
    // 1 x x x x x x x x => 8
    // 2 x x x x x x x x => 16
    // 3 x x => 18
    //
    let i: u32 = 18; // so it's {'b', 3}

    println!("'a' = {}", 'a' as i32);
    println!("y : {}", i / 8 + 1);

    assert_eq!(Position::from_u32(i), Some(Position::new('b', 3)));
}

#[test]
fn position_to_index() {
    let position = Position::new('b', 3);

    println!(
        "{:?} = {} + {} (x was '{}' -> {} as u32)",
        position,
        position.x() as u32 - 96,
        (position.y() - 1) * 8,
        position.x(),
        position.x() as u32
    );

    assert_eq!(position.to_index(), 18 - 1);
}

#[test]
fn around() {
    let position = Position::new('b', 3);
    assert_eq!(position.positions_around(), vec![
        Position::new('a', 2),
        Position::new('b', 2),
        Position::new('c', 2),

        Position::new('a', 3),
        Position::new('c', 3),
        
        Position::new('a', 4),
        Position::new('b', 4),
        Position::new('c', 4),
    ]);

    // Corner position
    let position = Position::new('a', 1);
    assert_eq!(position.positions_around(), vec![
        Position::new('b', 1),

        Position::new('a', 2),
        Position::new('b', 2),
    ]);
}