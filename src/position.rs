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

    pub fn from_i32(i: i32) -> Self {
        Self {
            x: char::from_u32((i - (i / 8)).try_into().unwrap()).unwrap(),
            y: (i / 8) as i8
        }
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
    let i: i32 = 17; // so it's (7, b)
    println!("{:?}", Position::from_i32(i));
}
