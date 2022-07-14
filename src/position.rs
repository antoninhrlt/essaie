// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::char;

/// Basically it permits to structure what is really a position on a board
///
/// - The `x` axis is letters from 'a' to 'h'
/// - The `y` axis is numbers from 1 to 8
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    x: char,
    y: u32
}

impl Position {
    pub fn new(x: char, y: u32) -> Self {
        Self {
            x,
            y
        }
    }

    /// Obtains a structured position from a dumb i32 index (used to walk 
    /// through the board's squares)
    pub fn from_i32(i: u32) -> Option<Self> {
        let x_as_i32: u32 = i - 8 * (i / 8) + 97;

        if x_as_i32 < 0 {
            return None;
        }

        Some(
            Self {
                x: char::from_u32(x_as_i32).unwrap(),
                y: i / 8 + 1
            }
        )
    }

    /// Destructure the position into a dumb index (used to walk through the
    /// board's squares)
    pub fn to_index(&self) -> usize {
        ((self.y * 8 - 1) + (self.x as u32 - 97 + 1)).try_into().unwrap()
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

#[test]
fn position_from_i32() {
    //   a b c d e f g h
    // 1 x x x x x x x x => 8    
    // 2 x x x x x x x x => 16
    // 3 x x => 18
    //
    let i: u32 = 18; // so it's {'c', 3}
    
    println!("'a' = {}", 'a' as i32);
    println!("y : {}", i / 8 + 1);

    assert_eq!(Position::from_i32(i), Some(Position::new('c', 3)));
}

#[test]
fn position_to_i32() {
    let position = Position::new('c', 3);
    assert_eq!(position.to_index(), 18);
}
