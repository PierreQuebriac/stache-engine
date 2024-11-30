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
            (
                Piece::PAWN as u8 | Piece::WHITE as u8,
                egui::include_image!("../assets/white_pawn.svg"),
            ),
            (
                Piece::PAWN as u8 | Piece::BLACK as u8,
                egui::include_image!("../assets/black_pawn.svg"),
            ),
            (
                Piece::BISHOP as u8 | Piece::WHITE as u8,
                egui::include_image!("../assets/white_bishop.svg"),
            ),
            (
                Piece::BISHOP as u8 | Piece::BLACK as u8,
                egui::include_image!("../assets/black_bishop.svg"),
            ),
            (
                Piece::KNIGHT as u8 | Piece::WHITE as u8,
                egui::include_image!("../assets/white_knight.svg"),
            ),
            (
                Piece::KNIGHT as u8 | Piece::BLACK as u8,
                egui::include_image!("../assets/black_knight.svg"),
            ),
            (
                Piece::ROOK as u8 | Piece::WHITE as u8,
                egui::include_image!("../assets/white_rook.svg"),
            ),
            (
                Piece::ROOK as u8 | Piece::BLACK as u8,
                egui::include_image!("../assets/black_rook.svg"),
            ),
            (
                Piece::KING as u8 | Piece::WHITE as u8,
                egui::include_image!("../assets/white_king.svg"),
            ),
            (
                Piece::KING as u8 | Piece::BLACK as u8,
                egui::include_image!("../assets/black_king.svg"),
            ),
            (
                Piece::QUEEN as u8 | Piece::WHITE as u8,
                egui::include_image!("../assets/white_queen.svg"),
            ),
            (
                Piece::QUEEN as u8 | Piece::BLACK as u8,
                egui::include_image!("../assets/black_queen.svg"),
            ),
        ];

        for (piece, img_src) in pieces {
            images.insert(piece, egui::Image::new(img_src));
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
    _turn: bool,
    drag: Option<usize>,
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            squares: [Piece::NONE as u8; 64],
            _turn: false,
            drag: None,
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

impl Board {
    pub fn draw_board(&mut self, ui: &mut egui::Ui, assets: &ChessAssets) {
        let size = ui.available_size();

        let (tile_size, offset) = if size.x > size.y {
            (size.y / 8., (size.x - size.y) / 2.)
        } else {
            (size.x / 8., (size.y - size.x) / 2.)
        };

        for x in 0..8 {
            for y in 0..8 {
                let board_pos = x + y * 8;
                let (xa, ya) = if size.x > size.y {
                    (x as f32 * tile_size + offset, y as f32 * tile_size)
                } else {
                    (x as f32 * tile_size, y as f32 * tile_size + offset)
                };

                let tile_rect = egui::Rect::from_two_pos(
                    egui::Pos2::new(xa, ya),
                    egui::Pos2::new(xa + tile_size, ya + tile_size),
                );

                ui.painter()
                    .rect_filled(tile_rect, 0.0, self.tile_color_at(x, y));

                if let Some(pointer_pos) = ui.input(|i| i.pointer.latest_pos()) {
                    if self.drag.is_some() && Some(board_pos) != self.drag {
                        if pointer_pos.x >= xa
                            && pointer_pos.x <= xa + tile_size
                            && pointer_pos.y >= ya
                            && pointer_pos.y <= ya + tile_size
                        {
                            ui.painter().rect_filled(
                                tile_rect,
                                0.0,
                                egui::Color32::from_rgba_premultiplied(0, 0, 0, 50),
                            );
                        }
                    }
                }

                // Check for drag interaction
                if ui
                    .interact(
                        tile_rect,
                        egui::Id::new((x, y)),
                        egui::Sense::click_and_drag(),
                    )
                    .drag_started()
                {
                    println!("Drag started at {:?}", board_pos);
                    self.drag = Some(board_pos);
                }

                // Render piece
                if self.squares[board_pos] != Piece::NONE as u8 {
                    // If this is the dragged piece, skip rendering it here
                    if Some(board_pos) != self.drag {
                        if let Some(piece_img) = ChessAssets::get(assets, self.squares[board_pos]) {
                            ui.put(
                                tile_rect,
                                piece_img.clone().fit_to_exact_size(egui::Vec2 {
                                    x: tile_size,
                                    y: tile_size,
                                }),
                            );
                        }
                    }
                }

                // Handle drag
                if let Some(init_drag_pos) = self.drag {
                    if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                        let drag_rect = egui::Rect::from_two_pos(
                            egui::Pos2::new(
                                pointer_pos.x - tile_size / 2.,
                                pointer_pos.y - tile_size / 2.,
                            ),
                            egui::Pos2::new(
                                pointer_pos.x + tile_size / 2.,
                                pointer_pos.y + tile_size / 2.,
                            ),
                        );

                        if let Some(piece_img) =
                            ChessAssets::get(assets, self.squares[init_drag_pos])
                        {
                            ui.put(
                                drag_rect,
                                piece_img.clone().fit_to_exact_size(egui::Vec2 {
                                    x: tile_size,
                                    y: tile_size,
                                }),
                            );
                        }
                    }
                }

                // Handle drop
                if let Some(init_drag_pos) = self.drag {
                    if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                        if ui.input(|i| i.pointer.any_released()) {
                            // Move the piece
                            let end_col =
                                ((pointer_pos.x - tile_rect.left()) / tile_size).floor() as usize;
                            let end_row =
                                ((pointer_pos.y - tile_rect.top()) / tile_size).floor() as usize;
                            let destination_board_pos = end_col + end_row * 8;
                            if end_row < 8 && end_col < 8 && init_drag_pos != destination_board_pos
                            {
                                self.squares[destination_board_pos] = self.squares[init_drag_pos];
                                self.squares[init_drag_pos] = Piece::NONE as u8;
                            }
                            self.drag = None;
                        }
                    }
                }
            }
        }
    }

    pub fn tile_color_at(&self, x: usize, y: usize) -> egui::Color32 {
        if (x + y) % 2 == 0 {
            // rgb(234, 233, 211)
            egui::Color32::from_rgb(234, 233, 211)
        } else {
            // rgb(75, 115, 152)
            egui::Color32::from_rgb(75, 115, 152)
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
