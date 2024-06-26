use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Jake Hemmerle's Personal Website",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    // Add state variables here if needed
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Initialize state variables here if needed
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Jake Hemmerle's Personal Website");

            ui.horizontal(|ui| {
                ui.label("Blockchain and Rust Work:");
                // Add content for blockchain and Rust work here
            });

            ui.horizontal(|ui| {
                ui.label("WebAssembly Projects:");
                // Add content for WebAssembly projects here
            });

            ui.horizontal(|ui| {
                ui.label("BASE Jumping:");
                // Add content for BASE jumping here
            });

            ui.horizontal(|ui| {
                ui.label("Reading List:");
                // Add content for reading list here
            });

            ui.horizontal(|ui| {
                ui.label("Blog Posts:");
                // Add content for blog posts here
            });

            ui.horizontal(|ui| {
                ui.label("Goodreads:");
                // Add content for Goodreads integration here
            });

            ui.horizontal(|ui| {
                ui.label("Instagram:");
                // Add content for Instagram integration here
            });
        });
    }
}
