use eframe::{egui};
use egui::Context;

mod game_of_life {
    pub mod grid;
    pub mod my_app;
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        Box::new(MyApp::default()),
        options,
    );
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self {
            runner: eframe::WebRunner::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn start(&self, canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
        log::info!("Attempting to start egui application with canvas_id: {}", canvas_id);
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        log::info!("Current DOM content: {}", body.inner_html());

        let canvas = document.get_element_by_id(canvas_id);
        match canvas {
            Some(element) => {
                log::info!("Successfully found canvas with id: {}", canvas_id);
                log::info!("Canvas element attribute names: {:?}", element.get_attribute_names());
                log::info!("Canvas element outer HTML: {}", element.outer_html());

                // Ensure the canvas is fully loaded and ready for rendering
                log::info!("Checking if canvas is ready for rendering...");
                if let Some(canvas_element) = element.dyn_ref::<web_sys::HtmlCanvasElement>() {
                    canvas_element.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
                    canvas_element.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);
                    log::info!("Canvas is ready for rendering. Attempting to start WebRunner with canvas_id: {}", canvas_id);
                    let web_options = eframe::WebOptions::default();
                    match self.runner
                        .start(
                            canvas_id,
                            web_options,
                            Box::new(|cc| Box::new(MyApp::default())),
                        )
                        .await {
                        Ok(_) => {
                            log::info!("Successfully started egui application with canvas_id: {}", canvas_id);
                            Ok(())
                        },
                        Err(e) => {
                            log::error!("Failed to start egui application with canvas_id: {}. Error: {:?}", canvas_id, e);
                            log::info!("Canvas element at the time of error: {:?}", document.get_element_by_id(canvas_id));
                            log::info!("Current DOM content at the time of error: {}", body.inner_html());
                            Err(e)
                        }
                    }
                } else {
                    log::error!("Failed to cast element to HtmlCanvasElement");
                    Err(wasm_bindgen::JsValue::from_str("Failed to cast element to HtmlCanvasElement"))
                }
            },
            None => {
                log::error!("Failed to find canvas with id: {}", canvas_id);
                log::info!("Current DOM content at the time of error: {}", body.inner_html());
                Err(wasm_bindgen::JsValue::from_str(&format!("Failed to find canvas with id: {}", canvas_id)))
            }
        }
    }

    #[wasm_bindgen]
    pub fn destroy(&self) {
        self.runner.destroy();
    }

    #[wasm_bindgen]
    pub fn has_panicked(&self) -> bool {
        self.runner.has_panicked()
    }

    #[wasm_bindgen]
    pub fn panic_message(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.message())
    }

    #[wasm_bindgen]
    pub fn panic_callstack(&self) -> Option<String> {
        self.runner.panic_summary().map(|s| s.callstack())
    }
}

struct MyApp {
    dark_mode: bool,
    game_of_life: game_of_life::my_app::MyApp,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dark_mode: false,
            game_of_life: game_of_life::my_app::MyApp::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let MyApp { dark_mode, game_of_life } = self;

        // Define color schemes
        let light_theme = egui::Visuals::light();
        let dark_theme = egui::Visuals::dark();

        // Apply the selected theme
        ctx.set_visuals(if *dark_mode { dark_theme } else { light_theme });

        // Render the Game of Life in the background
        log::info!("Rendering Game of Life background...");
        game_of_life.update(ctx, _frame);
        log::info!("Game of Life background rendered.");

        // Render the main content on top of the Game of Life background
        log::info!("Rendering main content...");
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to My Personal Website");

            // Theme toggle button
            if ui.button("Toggle Theme").clicked() {
                *dark_mode = !*dark_mode;
                log::info!("Theme toggled. Dark mode: {}", dark_mode);
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
        log::info!("Main content rendered.");
        ctx.request_repaint();
    }
}
