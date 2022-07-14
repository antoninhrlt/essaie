// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use crate::{
    piece::Piece,
    position::Position,
    team::Team,
};

/// The chessboard filled with piece
pub struct Board {
    squares: [Piece; 64]
}

impl Board {
    pub fn new() -> Self {
        let mut chessboard = Self {
            squares: [Piece::None; 64],
        };

        chessboard.reset();
        chessboard
    }

    /// Reset the board with the initial positions for every piece.
    pub fn reset(&mut self) {
        for i in 0..8 {

        }
    }

    /// Positions start from 1, not 0 like an index. So the first position at
    /// left-top is 1:1.
    ///
    /// The first integer is `x`, the second one is `y`.
    ///
    /// Returns a `None` value when the piece was not found. This can happen 
    /// when a piece has been captured by the enemy.
    pub fn get_position(&self, piece_with_team: Piece) -> Option<Position> {
        // todo!() : Smarter finder with optimization
        
        let mut i = 1; // will be used to get the current piece's position

        for piece in self.squares {
            let position = Position::from_i32(i);

            if piece != Piece::None {
                // Checks for the same piece type with the same team
                if piece == piece_with_team  {
                    return Some(position);
                }
            }

            i += 1;
        }

        None
    }

    // todo!() : Display the board on a GUI
}

#[test]
fn board_positions() {
    let chessboard = Board::new();
    
    assert_eq!(
        chessboard.get_position(Piece::King(Team::Black)), 
        Some(Position::new('e', 8))
    );

    assert_eq!(
        chessboard.get_position(Piece::Queen(Team::White)), 
        Some(Position::new('d', 1))
    );
}
