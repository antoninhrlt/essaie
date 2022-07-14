// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::team::Team;

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

// impl Piece {
//     pub fn initial_position(&self) -> Option<(i32, i32)> {
//         match self {
//             King(team) => match team {
//                 Team::Black => (,)
//                 Team::White => (,)
//             },
//             Queen(team) => match team {
//                 Team::Black => (,)
//                 Team::White => (,)
//             },
//             Rook(team) => match team {
//                 Team::Black => (,)
//                 Team::White => (,)
//             },
//             Bishop(team) => match team {
//                 Team::Black => (,)
//                 Team::White => (,)
//             },
//             Knight(team) => match team {
//                 Team::Black => (,)
//                 Team::White => (,)
//             },
//             Pawn(team) => match team {
//                 Team::Black => (,)
//                 Team::White => (,)
//             }
//         }
//     }
// }
