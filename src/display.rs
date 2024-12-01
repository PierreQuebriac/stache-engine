use crate::chess;
use std::collections::HashMap;
pub struct ChessAssets<'a> {
    images: HashMap<chess::Piece, egui::Image<'a>>,
}

impl<'a> ChessAssets<'a> {
    pub fn new() -> Self {
        let mut images = HashMap::new();

        // Map pieces to their SVG file paths
        let pieces = [
            (
                chess::Piece::new(chess::PieceType::Pawn, Some(chess::Color::White)),
                egui::include_image!("../assets/white_pawn.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Pawn, Some(chess::Color::Black)),
                egui::include_image!("../assets/black_pawn.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Bishop, Some(chess::Color::White)),
                egui::include_image!("../assets/white_bishop.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Bishop, Some(chess::Color::Black)),
                egui::include_image!("../assets/black_bishop.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Knight, Some(chess::Color::White)),
                egui::include_image!("../assets/white_knight.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Knight, Some(chess::Color::Black)),
                egui::include_image!("../assets/black_knight.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Rook, Some(chess::Color::White)),
                egui::include_image!("../assets/white_rook.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Rook, Some(chess::Color::Black)),
                egui::include_image!("../assets/black_rook.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::King, Some(chess::Color::White)),
                egui::include_image!("../assets/white_king.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::King, Some(chess::Color::Black)),
                egui::include_image!("../assets/black_king.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Queen, Some(chess::Color::White)),
                egui::include_image!("../assets/white_queen.svg"),
            ),
            (
                chess::Piece::new(chess::PieceType::Queen, Some(chess::Color::Black)),
                egui::include_image!("../assets/black_queen.svg"),
            ),
        ];

        for (piece, img_src) in pieces {
            images.insert(piece, egui::Image::new(img_src));
        }

        Self { images }
    }

    /// Retrieve a preloaded image for a piece
    pub fn get(&self, piece: chess::Piece) -> Option<&egui::Image<'a>> {
        self.images.get(&piece)
    }
}

pub struct ChessApp<'a> {
    board: chess::Board,
    drag: Option<usize>,
    assets: ChessAssets<'a>,
}

impl ChessApp<'_> {
    pub fn draw_board(&mut self, ui: &mut egui::Ui) {
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

                // Check for drag interaction if write turn
                if ui
                    .interact(
                        tile_rect,
                        egui::Id::new((x, y)),
                        egui::Sense::click_and_drag(),
                    )
                    .drag_started()
                    && (self.board.white_turn
                        == (self.board.squares[board_pos].color() == Some(chess::Color::White)))
                {
                    self.drag = Some(board_pos);
                }

                // Render piece
                if self.board.squares[board_pos] != chess::Piece::NONE {
                    // If this is the dragged piece, skip rendering it here
                    if Some(board_pos) != self.drag {
                        if let Some(piece_img) =
                            ChessAssets::get(&self.assets, self.board.squares[board_pos])
                        {
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
                            ChessAssets::get(&self.assets, self.board.squares[init_drag_pos])
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
                                //TODO Check if the move is valid
                                self.board.squares[destination_board_pos] =
                                    self.board.squares[init_drag_pos];
                                self.board.squares[init_drag_pos] = chess::Piece::NONE;

                                self.board.white_turn = !self.board.white_turn;
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

impl<'a> Default for ChessApp<'a> {
    fn default() -> Self {
        Self {
            board: chess::Board::default(),
            assets: ChessAssets::new(),
            drag: None,
        }
    }
}

impl<'a> eframe::App for ChessApp<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_board(ui);
        });
    }
}
