use std::thread;
use std::sync::mpsc::{self, TryRecvError};

use eframe::egui;

use egui_modal_spinner::ModalSpinner;

enum ThreadState {
    Closed,
}

struct MyApp {
    spinner: ModalSpinner,
    result_recv: Option<mpsc::Receiver<ThreadState>>,
}

impl MyApp {
    pub fn new() -> Self {

        Self {
            spinner: ModalSpinner::new(),
            result_recv: None,
        }
    }

    fn exec_task(&mut self) {
        let (tx, rx) = mpsc::channel();
        self.result_recv = Some(rx);

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_secs(4));
            let _ = tx.send(ThreadState::Closed);
        });
    }

    fn update_task_thread(&mut self) {
        if let Some(rx) = &self.result_recv {
            match rx.try_recv() {
                Ok(state) => match state {
                    ThreadState::Closed => {
                        self.spinner.close();
                        self.result_recv = None;
                    },
                },
                Err(err) => if err == TryRecvError::Disconnected {
                    self.spinner.close();
                    self.result_recv = None;
                    println!("thread ended unexpectedly");
                }
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui application");
            egui::widgets::global_theme_preference_buttons(ui);

            if ui.button("Do someting resource heavy!").clicked() {
                self.exec_task();

                self.spinner.open();
            }

            self.update_task_thread();

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
