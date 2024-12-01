#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Bishop = 2,
    Knight = 3,
    Rook = 4,
    King = 5,
    Queen = 6,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White = 8,  // 0b1000
    Black = 16, // 0b10000
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Piece(u8);

impl Piece {
    pub const NONE: Self = Self(0); // Empty square

    pub fn new(piece_type: PieceType, color: Option<Color>) -> Self {
        assert!(
            (color.is_some() && piece_type != PieceType::None)
                || (color.is_none() && piece_type == PieceType::None)
        );
        match color {
            Some(color) => Self(piece_type as u8 | color as u8),
            None => Self::NONE,
        }
    }

    #[allow(dead_code)]
    pub fn piece_type(&self) -> PieceType {
        match self.0 & 0b0111 {
            0 => PieceType::None,
            1 => PieceType::Pawn,
            2 => PieceType::Bishop,
            3 => PieceType::Knight,
            4 => PieceType::Rook,
            5 => PieceType::King,
            6 => PieceType::Queen,
            _ => unreachable!(),
        }
    }

    pub fn color(&self) -> Option<Color> {
        match self.0 & 0b11000 {
            0b1000 => Some(Color::White),
            0b10000 => Some(Color::Black),
            0 => None,
            _ => unreachable!(),
        }
    }
}

pub struct Board {
    pub squares: [Piece; 64],
    pub white_turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            squares: [Piece::NONE; 64],
            white_turn: true,
        };

        board.squares[0] = Piece::new(PieceType::Rook, Some(Color::Black));
        board.squares[1] = Piece::new(PieceType::Knight, Some(Color::Black));
        board.squares[2] = Piece::new(PieceType::Bishop, Some(Color::Black));
        board.squares[3] = Piece::new(PieceType::Queen, Some(Color::Black));
        board.squares[4] = Piece::new(PieceType::King, Some(Color::Black));
        board.squares[5] = Piece::new(PieceType::Bishop, Some(Color::Black));
        board.squares[6] = Piece::new(PieceType::Knight, Some(Color::Black));
        board.squares[7] = Piece::new(PieceType::Rook, Some(Color::Black));

        for i in 8..16 {
            board.squares[i] = Piece::new(PieceType::Pawn, Some(Color::Black));
        }

        for i in 48..56 {
            board.squares[i] = Piece::new(PieceType::Pawn, Some(Color::White));
        }

        board.squares[56] = Piece::new(PieceType::Rook, Some(Color::White));
        board.squares[57] = Piece::new(PieceType::Knight, Some(Color::White));
        board.squares[58] = Piece::new(PieceType::Bishop, Some(Color::White));
        board.squares[59] = Piece::new(PieceType::Queen, Some(Color::White));
        board.squares[60] = Piece::new(PieceType::King, Some(Color::White));
        board.squares[61] = Piece::new(PieceType::Bishop, Some(Color::White));
        board.squares[62] = Piece::new(PieceType::Knight, Some(Color::White));
        board.squares[63] = Piece::new(PieceType::Rook, Some(Color::White));

        board
    }
}
