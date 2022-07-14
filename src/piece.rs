// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::{position::Position, team::Team};

/// All chess pieces with their team.
///
/// `Piece::None` is used for empty cases, where there is no real piece. It
/// avoid using an `Option` object.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Piece {
    King(Team),
    Queen(Team),
    Rook(Team),
    Bishop(Team),
    Knight(Team),
    Pawn(Team),
    None,
}

impl Piece {
    pub fn initial_positions(&self) -> Vec<Position> {
        let positions: Vec<(char, u32)> = match self {
            Piece::King(team) => match team {
                Team::Black => vec![('e', 8)],
                Team::White => vec![('e', 1)],
            },
            Piece::Queen(team) => match team {
                Team::Black => vec![('d', 8)],
                Team::White => vec![('d', 1)],
            },
            Piece::Rook(team) => match team {
                Team::Black => vec![('a', 8), ('h', 8)],
                Team::White => vec![('a', 1), ('h', 1)],
            },
            Piece::Bishop(team) => match team {
                Team::Black => vec![('c', 8), ('f', 8)],
                Team::White => vec![('c', 1), ('f', 1)],
            },
            Piece::Knight(team) => match team {
                Team::Black => vec![('b', 8), ('g', 8)],
                Team::White => vec![('b', 1), ('g', 1)],
            },
            Piece::Pawn(team) => {
                let mut v: Vec<(char, u32)> = vec![];
                for x in 'a'..'i' {
                    v.push((
                        x,
                        match team {
                            Team::Black => 7,
                            Team::White => 2,
                        },
                    ));
                }
                v
            }
            Piece::None => panic!(),
        };

        let mut positions_r: Vec<Position> = vec![];

        for position in positions {
            positions_r.push(Position::new(position.0, position.1))
        }
        positions_r
    }
}
