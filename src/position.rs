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
    y: i8
}

impl Position {
    pub fn new(x: char, y: i8) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn from_i32(i: i32) -> Option<Self> {
        let x_as_i32: i32 = i - 8 * (i / 8) + 97;

        if x_as_i32 < 0 {
            return None;
        }

        Some(
            Self {
                x: char::from_u32(x_as_i32 as u32).unwrap(),
                y: i as i8 / 8 + 1
            }
        )
    }

    pub fn set_x(&mut self, x: char) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: i8) -> &mut Self {
        self.y = y;
        self
    }

    pub fn x(&self) -> char {
        self.x
    }

    pub fn y(&self) -> i8 {
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
    let i: i32 = 18; // so it's {'c', 3}
    
    println!("'a' = {}", 'a' as i32);
    println!("y : {}", i / 8 + 1);
    println!("{:?}", Position::from_i32(i));
}
