#[derive(Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Piece {
    #[default]
    NONE = 0,
    PAWN = 1,
    BISHOP = 2,
    KNIGHT = 3,
    ROOK = 4,
    KING = 5,
    QUEEN = 6,

    WHITE = 8,
    BLACK = 16,
}

impl Piece {
    #[allow(dead_code)]
    pub fn is_white(&self) -> bool {
        *self as u8 & Piece::WHITE as u8 != 0
    }

    #[allow(dead_code)]
    pub fn is_black(&self) -> bool {
        *self as u8 & Piece::BLACK as u8 != 0
    }

    #[allow(dead_code)]
    pub fn is_piece(&self) -> bool {
        *self as u8 & 7 != 0
    }
}

pub struct Board {
    pub squares: [u8; 64],
    pub white_turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            squares: [Piece::NONE as u8; 64],
            white_turn: true,
        };

        board.squares[0] = Piece::ROOK as u8 | Piece::BLACK as u8;
        board.squares[1] = Piece::KNIGHT as u8 | Piece::BLACK as u8;
        board.squares[2] = Piece::BISHOP as u8 | Piece::BLACK as u8;
        board.squares[3] = Piece::QUEEN as u8 | Piece::BLACK as u8;
        board.squares[4] = Piece::KING as u8 | Piece::BLACK as u8;
        board.squares[5] = Piece::BISHOP as u8 | Piece::BLACK as u8;
        board.squares[6] = Piece::KNIGHT as u8 | Piece::BLACK as u8;
        board.squares[7] = Piece::ROOK as u8 | Piece::BLACK as u8;

        for i in 8..16 {
            board.squares[i] = Piece::PAWN as u8 | Piece::BLACK as u8;
        }

        for i in 48..56 {
            board.squares[i] = Piece::PAWN as u8 | Piece::WHITE as u8;
        }

        board.squares[56] = Piece::ROOK as u8 | Piece::WHITE as u8;
        board.squares[57] = Piece::KNIGHT as u8 | Piece::WHITE as u8;
        board.squares[58] = Piece::BISHOP as u8 | Piece::WHITE as u8;
        board.squares[59] = Piece::QUEEN as u8 | Piece::WHITE as u8;
        board.squares[60] = Piece::KING as u8 | Piece::WHITE as u8;
        board.squares[61] = Piece::BISHOP as u8 | Piece::WHITE as u8;
        board.squares[62] = Piece::KNIGHT as u8 | Piece::WHITE as u8;
        board.squares[63] = Piece::ROOK as u8 | Piece::WHITE as u8;

        board
    }
}
