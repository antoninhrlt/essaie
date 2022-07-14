// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::fmt;

use crate::{
    piece::Piece,
    position::Position,
    team::Team,
};

/// The chessboard filled with piece
///
/// Instead of reverting the position each time is needed, the chessboard is 
/// simply reverted. So, black pieces are at the bottom and white pieces are at
/// the top. For printing it or displaying on a GUI, remember you have to 
/// graphically revert it
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
        let pawn_position = Position::new('a', 2);
        self.squares[pawn_position.to_index()] = Piece::Pawn(Team::Black);
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
            if piece != Piece::None {
                // Checks for the same piece type with the same team
                if piece == piece_with_team  {
                    return Some(Position::from_i32(i).unwrap());
                }
            }

            i += 1;
        }

        None
    }

    // todo!() : Display the board on a GUI
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (mut x, mut y) = (1, 1);
        write!(f, "{}", y)?;

        for square in self.squares {
            write!(f, " | {:?}\t", square)?;

            if x == 8 && y != 8 {
                y += 1;
                write!(f, "\n{}", y)?;
                x = 0;
            }
            x += 1;
        }

        Ok(())
    }
}

#[test]
fn board_positions() {
    let chessboard = Board::new();

    println!("{}", chessboard);
    
    // assert_eq!(
    //     chessboard.get_position(Piece::King(Team::Black)), 
    //     Some(Position::new('e', 8))
    // );

    // assert_eq!(
    //     chessboard.get_position(Piece::Queen(Team::White)), 
    //     Some(Position::new('d', 1))
    // );
}
