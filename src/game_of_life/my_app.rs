use crate::game_of_life::grid::Grid;
use eframe::{
    egui::{self},
    epaint::{Stroke},
};
use std::time::{Duration, Instant};

pub struct MyApp {
    grid: Grid,
    running: bool,
    last_update: Instant,
    update_interval: Duration,
    tile_size: f32,
    temporary_size: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            grid: Grid::new(64, 64),
            running: false,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(500),
            tile_size: 16.0,
            temporary_size: 16.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if self.running && now - self.last_update >= self.update_interval {
            self.grid.update_game_state();
            self.last_update = now;
        }
        let cellcolor = egui::Color32::from_rgb(0, 100, 200);

        let strokecolor = egui::Color32::from_rgb(100, 100, 100);
        let stroke = Stroke {
            width: 0.1,
            color: strokecolor,
        };
        let size = egui::Vec2::new(self.tile_size, self.tile_size);
        egui::CentralPanel::default().show(ctx, |ui| {

            for (x, row) in self.grid.cells.iter_mut().enumerate() {
                for (y, cell) in row.iter_mut().enumerate() {
                    let top_left =
                        egui::Pos2::new(x as f32 * self.tile_size, y as f32 * self.tile_size);
                    let rect = egui::Rect::from_min_size(top_left, size);
                    if *cell {
                        ui.painter().rect_filled(rect, 0.0, cellcolor);
                    } else {
                        ui.painter().rect_stroke(rect, 0.0, stroke);
                    }
                    if !self.running
                        && ui
                            .interact(rect, egui::Id::new((x, y)), egui::Sense::click())
                            .clicked()
                    {
                        *cell = !*cell;
                    }
                }
            }

            if ui.button("Start").clicked() {
                self.running = !self.running;
            }

            ui.add(egui::Slider::new(&mut self.temporary_size, 0.0..=100.0).text("Size of tile"));
            ui.add(egui::Slider::new(&mut self.grid.width, 0..=100).text("Grid width"));
            ui.add(egui::Slider::new(&mut self.grid.height, 0..=100).text("Grid height"));
            if ui.button("Apply settings").clicked() && !self.running {
                self.tile_size = self.temporary_size;
                self.grid = Grid::new(self.grid.width, self.grid.height);
                self.grid.update_game_state();
            }
        });
        ctx.request_repaint();
    }
}
