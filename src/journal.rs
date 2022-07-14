// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::{
    piece::Piece,
    position::Position,
};

use std::string::ToString;

#[derive(Debug, Clone)]
pub enum JournalLog {
    /// - `Piece` : Moving piece
    /// - `Position` == "from_position"
    /// - `Position` == "to_position"
    Move(Piece, Position, Position),
    /// - `Piece` : Piece that captured the other piece
    /// - `Piece` : The captured piece
    Capture(Piece, Piece),
    /// - `Piece` : The piece attacking the enemy king
    /// - `Position` : The attacker's position
    /// - `Position` : The attacked king's position
    Check(Piece, Position, Position),
    /// - `Piece` : The piece attacking the enemy king
    /// - `Position` : The attacker's position
    /// - `Position` : The attacked king's position
    CheckMate(Piece, Position, Position),
}


impl ToString for JournalLog {
    fn to_string(&self) -> String {
        match self {
            Self::Move(moving_piece, from_position, to_position) => {
                format!(
                    "Move of {:?} from {} to {}", 
                    moving_piece, 
                    from_position, 
                    to_position
                )
            },
            Self::Capture(attacker_piece, captured_piece) => {
                format!("Capture of {:?} by {:?}", captured_piece, attacker_piece)
            },
            Self::Check(attacker_piece, attacker_position, king_position) => {
                format!("Check on {:?} by {:?} from {:?}", king_position, attacker_piece, attacker_position)
            },
            Self::CheckMate(attacker_piece, attacker_position, king_position) => {
                format!("Checkmate on {:?} by {:?} from {:?}", king_position, attacker_piece, attacker_position)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Journal {
    logs: Vec<JournalLog>,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            logs: vec![]
        }
    }

    pub fn add_log(&mut self, log: JournalLog) {
        self.logs.push(log);
    }

    pub fn logs(&self) -> &Vec<JournalLog> {
        &self.logs
    }
}

impl ToString for Journal {
    fn to_string(&self) -> String {
        format!("{}", self.logs().iter().map(|log| log.to_string() + "\n").collect::<String>())
    }
}

#[test]
fn journal_display() {
    use crate::{
        piece::Piece,
        position::Position,
        team::Team,
    };

    let mut journal = Journal::new();
    journal.add_log(JournalLog::Move(Piece::Queen(Team::Black), Position::new('a', 2), Position::new('b', 2)));
    journal.add_log(JournalLog::Capture(Piece::Pawn(Team::White), Piece::Queen(Team::Black)));

    println!("{}", journal.to_string());
}
