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
        let blacks = vec![
            Piece::King(Team::Black),
            Piece::Queen(Team::Black),
            Piece::Rook(Team::Black),
            Piece::Bishop(Team::Black),
            Piece::Knight(Team::Black),
            Piece::Pawn(Team::Black),
        ];

        let whites = vec![
            Piece::King(Team::White),
            Piece::Queen(Team::White),
            Piece::Rook(Team::White),
            Piece::Bishop(Team::White),
            Piece::Knight(Team::White),
            Piece::Pawn(Team::White),
        ];

        for i in blacks.iter().zip(whites.iter()) {
            let (black_piece, white_piece) = i;
            for j in black_piece.initial_positions().iter().zip(white_piece.initial_positions().iter()) {
                let (black_initial_position, white_initial_position) = j;
                self.squares[black_initial_position.to_index()] = *black_piece;
                self.squares[white_initial_position.to_index()] = *white_piece;
            }
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
            if piece != Piece::None {
                // Checks for the same piece type with the same team
                if piece == piece_with_team  {
                    return Some(Position::from_i32(i));
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

        for square in self.squares {
            write!(f, "\x1b[1;37m{:?}\x1b[0m \x1b[3m{}{}\x1b[0m | ", 
                square, 
                char::from_u32(x + 96).unwrap(), 
                y
            )?;

            if x == 8 && y != 8 {
                y += 1;
                write!(f, "\n")?;
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

    assert_eq!(
        chessboard.get_position(Piece::King(Team::Black)), 
        Some(Position::new('e', 8))
    );

    assert_eq!(
        chessboard.get_position(Piece::Queen(Team::White)), 
        Some(Position::new('d', 1))
    );
}
