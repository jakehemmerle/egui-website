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
            ui.heading("Welcome to My Personal Website");

            // Theme toggle button
            if ui.button("Toggle Theme").clicked() {
                *dark_mode = !*dark_mode;
            }

            ui.horizontal(|ui| {
                ui.label("Blockchain and Rust Work:");
                ui.label("I have been working on various blockchain projects using Rust. I have contributed to several open-source projects and have a deep understanding of blockchain technology.");
            });

            ui.horizontal(|ui| {
                ui.label("WebAssembly Projects:");
                ui.label("I have developed multiple WebAssembly projects, showcasing my expertise in this area. I have created efficient and high-performance applications using WebAssembly.");
            });

            ui.horizontal(|ui| {
                ui.label("BASE Jumping:");
                ui.label("I am an avid BASE jumper and enjoy the thrill of this extreme sport. I have completed numerous jumps and continue to pursue this passion.");
            });

            ui.horizontal(|ui| {
                ui.label("Reading List:");
                ui.label("I am an avid reader and enjoy exploring various genres. I frequently update my reading list and share my thoughts on the books I read.");
            });

            ui.horizontal(|ui| {
                ui.label("Blog Posts:");
                ui.label("I write blog posts on various topics, including technology, personal experiences, and more. Stay tuned for my latest updates.");
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
