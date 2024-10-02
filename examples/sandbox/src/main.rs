use eframe::egui;

use egui_modal_spinner::ModalSpinner;

struct MyApp {
    spinner: ModalSpinner,
}

impl MyApp {
    pub fn new() -> Self {
        Self {
            spinner: ModalSpinner::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui application");
            egui::widgets::global_theme_preference_buttons(ui);

            self.spinner.update(ctx);
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui application",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new()))),
    )
}
