use std::collections::HashMap;

use eframe::egui;

#[derive(Default, Clone, PartialEq)]
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
    pub fn to_svg(value: u8) -> Option<&'static str> {
        match value {
            v if v & Piece::WHITE as u8 != 0 => match v & 0b111 {
                x if x == Piece::PAWN as u8 => Some("white_pawn.svg"),
                x if x == Piece::BISHOP as u8 => Some("white_bishop.svg"),
                x if x == Piece::KNIGHT as u8 => Some("white_knight.svg"),
                x if x == Piece::ROOK as u8 => Some("white_rook.svg"),
                x if x == Piece::KING as u8 => Some("white_king.svg"),
                x if x == Piece::QUEEN as u8 => Some("white_queen.svg"),
                _ => None,
            },
            v if v & Piece::BLACK as u8 != 0 => match v & 0b111 {
                x if x == Piece::PAWN as u8 => Some("black_pawn.svg"),
                x if x == Piece::BISHOP as u8 => Some("black_bishop.svg"),
                x if x == Piece::KNIGHT as u8 => Some("black_knight.svg"),
                x if x == Piece::ROOK as u8 => Some("black_rook.svg"),
                x if x == Piece::KING as u8 => Some("black_king.svg"),
                x if x == Piece::QUEEN as u8 => Some("black_queen.svg"),
                _ => None,
            },
            _ => None,
        }
    }
}

pub struct ChessAssets<'a> {
    images: HashMap<u8, egui::Image<'a>>,
}

impl<'a> ChessAssets<'a> {
    pub fn new() -> Self {
        let mut images = HashMap::new();

        // Map pieces to their SVG file paths
        let pieces = [
            (Piece::PAWN as u8 | Piece::WHITE as u8, "white_pawn.svg"),
            (Piece::PAWN as u8 | Piece::BLACK as u8, "black_pawn.svg"),
            (Piece::BISHOP as u8 | Piece::WHITE as u8, "white_bishop.svg"),
            (Piece::BISHOP as u8 | Piece::BLACK as u8, "black_bishop.svg"),
            (Piece::KNIGHT as u8 | Piece::WHITE as u8, "white_knight.svg"),
            (Piece::KNIGHT as u8 | Piece::BLACK as u8, "black_knight.svg"),
            (Piece::ROOK as u8 | Piece::WHITE as u8, "white_rook.svg"),
            (Piece::ROOK as u8 | Piece::BLACK as u8, "black_rook.svg"),
            (Piece::KING as u8 | Piece::WHITE as u8, "white_king.svg"),
            (Piece::KING as u8 | Piece::BLACK as u8, "black_king.svg"),
            (Piece::QUEEN as u8 | Piece::WHITE as u8, "white_queen.svg"),
            (Piece::QUEEN as u8 | Piece::BLACK as u8, "black_queen.svg"),
        ];

        for (piece, path) in pieces {
            images.insert(
                piece,
                //egui::Image::new(format!("bytes://../assets/{}", path)),
                egui::Image::new("file://../assets/black_bishop.svg"),
            );
        }

        Self { images }
    }

    /// Retrieve a preloaded image for a piece
    pub fn get(&self, piece: u8) -> Option<&egui::Image<'a>> {
        self.images.get(&piece)
    }
}

pub struct Board {
    squares: [u8; 64],
    turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            squares: [Piece::NONE as u8; 64],
            turn: false,
        };

        board.squares[0] = Piece::PAWN as u8 | Piece::WHITE as u8;

        board
    }
}

impl Board {
    pub fn draw_board(&self, ui: &mut egui::Ui, assets: &ChessAssets) {
        let size = ui.available_size();

        let (tile_size, offset) = if size.x > size.y {
            (size.y / 8., (size.x - size.y) / 2.)
        } else {
            (size.x / 8., (size.y - size.x) / 2.)
        };

        for x in 0..8 {
            for y in 0..8 {
                let (xa, ya) = if size.x > size.y {
                    (x as f32 * tile_size + offset, y as f32 * tile_size)
                } else {
                    (x as f32 * tile_size, y as f32 * tile_size + offset)
                };

                let rect = egui::Rect::from_two_pos(
                    egui::Pos2::new(xa, ya),
                    egui::Pos2::new(xa + tile_size, ya + tile_size),
                );

                ui.painter()
                    .rect_filled(rect, 0.0, self.tile_color_at(x, y));

                let board_pos = x * 8 + y;
                if let Some(piece_img) = ChessAssets::get(assets, self.squares[board_pos]) {
                    ui.put(
                        rect,
                        piece_img.clone().fit_to_exact_size(egui::Vec2 {
                            x: tile_size,
                            y: tile_size,
                        }),
                    );
                }
            }
        }
    }

    pub fn tile_color_at(&self, x: usize, y: usize) -> egui::Color32 {
        if (x + y) % 2 == 0 {
            egui::Color32::from_rgb(236, 212, 179)
        } else {
            egui::Color32::from_rgb(53, 45, 45)
        }
    }
}

struct ChessApp<'a> {
    board: Board,
    assets: ChessAssets<'a>,
}

impl<'a> Default for ChessApp<'a> {
    fn default() -> Self {
        Self {
            board: Board::default(),
            assets: ChessAssets::new(),
        }
    }
}

impl<'a> eframe::App for ChessApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.board.draw_board(ui, &self.assets);
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chess Board",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<ChessApp>::default())
        }),
    )
}
