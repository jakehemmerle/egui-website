use eframe::{egui, epi};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp::default()),
        options,
    );
}

struct MyApp {
    dark_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
        }
    }
}

impl epi::App for MyApp {
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame) {
        let MyApp { dark_mode } = self;

        // Define color schemes
        let light_theme = egui::Visuals::light();
        let dark_theme = egui::Visuals::dark();

        // Apply the selected theme
        ctx.set_visuals(if *dark_mode { dark_theme } else { light_theme });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Jake Hemmerle's Personal Website");

            // Theme toggle button
            if ui.button("Toggle Theme").clicked() {
                *dark_mode = !*dark_mode;
            }

            ui.horizontal(|ui| {
                ui.label("Blockchain and Rust Work:");
                ui.label("Jake has been working on various blockchain projects using Rust. He has contributed to several open-source projects and has a deep understanding of blockchain technology.");
            });

            ui.horizontal(|ui| {
                ui.label("WebAssembly Projects:");
                ui.label("Jake has developed multiple WebAssembly projects, showcasing his expertise in this area. He has created efficient and high-performance applications using WebAssembly.");
            });

            ui.horizontal(|ui| {
                ui.label("BASE Jumping:");
                ui.label("Jake is an avid BASE jumper and enjoys the thrill of this extreme sport. He has completed numerous jumps and continues to pursue this passion.");
            });

            ui.horizontal(|ui| {
                ui.label("Reading List:");
                ui.label("Jake is an avid reader and enjoys exploring various genres. He frequently updates his reading list and shares his thoughts on the books he reads.");
            });

            ui.horizontal(|ui| {
                ui.label("Blog Posts:");
                ui.label("Jake writes blog posts on various topics, including technology, personal experiences, and more. Stay tuned for his latest updates.");
            });

            ui.horizontal(|ui| {
                ui.label("Goodreads:");
                ui.hyperlink("https://www.goodreads.com/user/show/12345678-jake-hemmerle");
            });

            ui.horizontal(|ui| {
                ui.label("Instagram:");
                ui.hyperlink("https://www.instagram.com/sends.and.friends/");
            });
        });
    }

    fn name(&self) -> &str {
        "Jake Hemmerle's Personal Website"
    }
}
