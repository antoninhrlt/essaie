// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::string::ToString;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Team {
    Black,
    White,
}

impl ToString for Team {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

#[test]
fn print_team() {
    let team = Team::Black;
    println!("{:?}", team);
    assert_eq!(team.to_string(), String::from("black"));
}
