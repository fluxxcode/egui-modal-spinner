//! egui-modal-spinner
#![warn(missing_docs)] // Let's keep the public API well documented!

use egui::Widget;

/// Represents the state the spinner is currently in.
#[derive(Debug, Clone, PartialEq)]
pub enum SpinnerState {
    /// The spinner is currently closed and not visible.
    Closed,
    /// The spinner is currently open and user input is suppressed.
    Open,
}

/// Represents a spinner instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ModalSpinner {
    state: SpinnerState,
    id: String,

    fill_color: egui::Color32,
    spinner: Spinner,
}

/// Creation methods
impl ModalSpinner {
    /// Creates a new spinner instance.
    pub fn new() -> Self {
        Self {
            state: SpinnerState::Closed,
            id: String::from("_modal_spinner"),

            fill_color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 120),
            spinner: Spinner::default(),
        }
    }

    /// Sets the fill color of the modal background.
    pub fn fill_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.fill_color = color.into();
        self
    }

    /// Sets the size of the spinner.
    pub fn spinner_size(mut self, size: f32) -> Self {
        self.spinner.size = Some(size);
        self
    }

    /// Sets the color of the spinner.
    pub fn spinner_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.spinner.color = Some(color.into());
        self
    }
}

/// Getter and setter
impl ModalSpinner {
    /// Gets the current state of the spinner.
    pub fn state(&self) -> &SpinnerState {
        &self.state
    }
}

/// Implementation methods
impl ModalSpinner {
    /// Opens the spinner.
    pub fn open(&mut self) {
       self.state = SpinnerState::Open;
    }

    /// Closes the spinner.
    pub fn close(&mut self) {
        self.state = SpinnerState::Closed;
    }

    /// Main update method of the spinner that should be called every frame if you want the
    /// spinner to be visible.
    ///
    /// This has no effect if the `SpinnerState` is currently not `SpinnerState::Open`.
    pub fn update(&mut self, ctx: &egui::Context) {
        if self.state != SpinnerState::Open {
            return;
        }

        let window_fill = ctx.style().visuals.window_fill;
        let screen_rect = ctx.input(|i| i.screen_rect);

        ctx.style_mut(|s| s.visuals.window_fill = self.fill_color);

        let re = egui::Window::new(&self.id)
            .interactable(false)
            .title_bar(false)
            .fixed_size(screen_rect.size())
            .fixed_pos(screen_rect.left_top())
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    self.spinner.update(ui);
                });
            });

        if let Some(re) = re {
            ctx.move_to_top(re.response.layer_id);
        }

        ctx.style_mut(|s| s.visuals.window_fill = window_fill);
    }
}

/// This tests if the spinner is send and sync.
#[cfg(test)]
const fn test_prop<T: Send + Sync>() {}

#[test]
const fn test() {
    test_prop::<ModalSpinner>();
}

/// Wrapper above `egui::Spinner` to be able to customize trait implementations.
#[derive(Debug, Clone, PartialEq)]
struct Spinner {
    pub size: Option<f32>,
    pub color: Option<egui::Color32>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: None,
            color: None,
        }
    }
}

impl Spinner {
    fn update(&self, ui: &mut egui::Ui) -> egui::Response {
        let mut spinner = egui::Spinner::new();

        if let Some(size) = self.size {
            spinner = spinner.size(size);
        }

        if let Some(color) = self.color {
            spinner = spinner.color(color);
        }

        spinner.ui(ui)
    }
}
