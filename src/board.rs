// This file is part of "pawn"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use std::fmt;

use crate::{piece::Piece, position::Position, team::Team};

/// The chessboard filled with piece
///
/// Instead of reverting the position each time is needed, the chessboard is
/// simply reverted. So, black pieces are at the bottom and white pieces are at
/// the top. For printing it or displaying on a GUI, remember you have to
/// graphically revert it
pub struct Board {
    squares: [Piece; 64],
    captured_pieces: Vec<Piece>,
}

impl Board {
    /// Initializes a new chessboard with the pieces are their initial pieces
    pub fn new() -> Self {
        let mut chessboard = Self {
            squares: [Piece::None; 64],
            captured_pieces: vec![],
        };

        chessboard.reset();
        chessboard
    }

    /// Fills the chessboard with `Piece::None` then resets the board with the 
    /// initial positions for every piece.
    pub fn reset(&mut self) {
        self.squares = [Piece::None; 64];

        self.reset_team(Team::White);
        self.reset_team(Team::Black);
    }

    /// Reset positions for pieces from a certain team to their initial 
    /// positions.
    fn reset_team(&mut self, team: Team) {
        let pieces = vec![
            Piece::King(team),
            Piece::Queen(team),
            Piece::Rook(team),
            Piece::Bishop(team),
            Piece::Knight(team),
            Piece::Pawn(team),
        ];

        for piece in pieces {
            for position in piece.initial_positions() {
                self.squares[position.to_index()] = piece;
            }
        }
    }

    /// Move the piece at `from_position` to the `to_position` position without
    /// checking anything : it's not following the chess rules. Before moving
    /// a piece, be sure it's a possible move.
    ///
    /// Replace the `from_position` by a `Piece::None` object.
    ///
    /// If there is a piece at `to_position`, it will be set as captured
    ///
    /// Returns an error when the `from_position` is a `Piece::None` piece
    pub fn move_piece(&mut self, from_position: Position, to_position: Position) -> Result<(), &'static str> {
        let piece = self.get_piece(&from_position);

        if piece == Piece::None {
            return Err("try to move a `None` piece");
        }
        
        // Captures the piece when not `None`
        {
            let to_piece = self.get_piece(&to_position);
            if to_piece != Piece::None {
                self.captured_pieces.push(to_piece);
            }
        }

        self.squares[to_position.to_index()] = piece;
        self.squares[from_position.to_index()] = Piece::None;

        Ok(())
    }

    /// Finds the piece at `position` and returns it
    pub fn get_piece(&self, position: &Position) -> Piece {
        self.squares[position.to_index()]
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
                if piece == piece_with_team {
                    return Some(Position::from_u32(i).unwrap());
                }
            }

            i += 1;
        }

        None
    }

    pub fn captured_pieces(&self) -> &Vec<Piece> {
        &self.captured_pieces
    }

    // todo!() : Display the board on a GUI
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (mut x, mut y) = (1, 1);

        for square in self.squares {
            write!(
                f,
                "\x1b[1;37m{:?}\x1b[0m \x1b[3m{}{}\x1b[0m | ",
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

/// All move check functions 
impl Board {
    pub fn check_piece_move(&self, from_position: Position, to_position: Position) -> Result<(), &'static str> {
        let piece: Piece = self.get_piece(&from_position);

        match piece {
            Piece::King(team) => {
                self.check_blocked(&piece, &from_position)?;
                Ok(())
            },
            Piece::Queen(team) => {
                self.check_blocked(&piece, &from_position)?;
                Ok(())
            },
            Piece::Rook(team) => {
                self.check_blocked(&piece, &from_position)?;
                Ok(())
            },
            Piece::Bishop(team) => {
                self.check_blocked(&piece, &from_position)?;
                Ok(())
            },
            Piece::Knight(team) => {
                Ok(())
            },
            Piece::Pawn(team) => {
                self.check_blocked(&piece, &from_position)?;
                Ok(())
            },
            Piece::None => {
                Err("try to move a `None` piece")
            }
        }
    }

    /// A piece is blocked when it cannot move in any direction
    fn check_blocked(&self, piece: &Piece, piece_position: &Position) -> Result<(), &'static str> {
        for position in piece_position.positions_around() {
            println!("piece {:?} at {:?}", self.get_piece(&position), position);

            if self.get_piece(&position) != Piece::None {
                return Err("the piece is blocked, all squares around are occupied");
            }
        }
        Ok(())
    }
}

#[test]
fn board_positions() {
    let chessboard = Board::new();

    println!("{}", chessboard);

    assert_eq!(
        chessboard.get_position(Piece::Rook(Team::White)),
        Some(Position::new('a', 1))
    );

    assert_eq!(
        chessboard.get_position(Piece::King(Team::Black)),
        Some(Position::new('e', 8))
    );
}

#[test]
fn move_pieces() {
    let mut chessboard = Board::new();
    println!("1:\n{}", chessboard);

    match chessboard.check_piece_move(Position::new('c', 1), Position::new('b', 2)) {
        Err(message) => panic!("{}", message),
        Ok(()) => {}
    }

    chessboard.move_piece(Position::new('b', 2), Position::new('b', 3));
    println!("2:\n{}", chessboard);
}