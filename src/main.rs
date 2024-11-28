use eframe::egui;

#[derive(Default)]
pub struct Board {}

impl Board {
    pub fn draw_board(&self, ui: &mut egui::Ui) {
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

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_board(ui);
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chess Board",
        options,
        Box::new(|_cc| Ok(Box::<Board>::default())),
    )
}
