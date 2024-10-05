//! egui-modal-spinner
#![warn(missing_docs)] // Let's keep the public API well documented!

use std::time::SystemTime;

use egui::Widget;

/// Represents the state the spinner is currently in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpinnerState {
    /// The spinner is currently closed and not visible.
    Closed,
    /// The spinner is currently open and user input is suppressed.
    Open,
}

/// Represents a spinner instance.
#[derive(Debug, Clone)]
pub struct ModalSpinner {
    /// Represents the state of the spinner.
    state: SpinnerState,
    /// If the modal is closed but currently fading out.
    fading_out: bool,
    /// Timestamp when the spinner was opened.
    timestamp: SystemTime,

    /// The ID of the modal area. If None, a default is used.
    id: Option<egui::Id>,
    /// The fill color of the modal background.
    fill_color: Option<egui::Color32>,
    /// If the modal window should fade in when opening.
    fade_in: bool,
    /// If the modal should fade out when closing.
    fade_out: bool,
    /// Configuration of the spinner.
    spinner: Spinner,
    /// If the time elapsed since opening should be displayed under the spinner.
    show_elapsed_time: bool,
}

impl Default for ModalSpinner {
    fn default() -> Self {
        Self::new()
    }
}

/// Creation methods
impl ModalSpinner {
    /// Creates a new spinner instance.
    pub fn new() -> Self {
        Self {
            state: SpinnerState::Closed,
            fading_out: false,
            timestamp: SystemTime::now(),

            id: None,
            fill_color: None,
            fade_in: true,
            fade_out: true,
            spinner: Spinner::default(),
            show_elapsed_time: true,
        }
    }

    /// Sets the ID of the spinner.
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the fill color of the modal background.
    pub fn fill_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.fill_color = Some(color.into());
        self
    }

    /// If the modal should fade in.
    pub fn fade_in(mut self, fade_in: bool) -> Self {
        self.fade_in = fade_in;
        self
    }

    /// If the modal should fade out.
    pub fn fade_out(mut self, fade_out: bool) -> Self {
        self.fade_out = fade_out;
        self
    }

    /// Sets the size of the spinner.
    pub const fn spinner_size(mut self, size: f32) -> Self {
        self.spinner.size = Some(size);
        self
    }

    /// Sets the color of the spinner.
    pub fn spinner_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.spinner.color = Some(color.into());
        self
    }

    /// If the elapsed time should be displayed below the spinner.
    pub const fn show_elapsed_time(mut self, show_elapsed_time: bool) -> Self {
        self.show_elapsed_time = show_elapsed_time;
        self
    }
}

/// Getter and setter
impl ModalSpinner {
    /// Gets the current state of the spinner.
    pub const fn state(&self) -> &SpinnerState {
        &self.state
    }
}

/// Implementation methods
impl ModalSpinner {
    /// Opens the spinner.
    pub fn open(&mut self) {
        self.state = SpinnerState::Open;
        self.timestamp = SystemTime::now();
    }

    /// Closes the spinner.
    pub fn close(&mut self) {
        self.state = SpinnerState::Closed;
        self.fading_out = self.fade_out;
    }

    /// Main update method of the spinner that should be called every frame if you want the
    /// spinner to be visible.
    ///
    /// This has no effect if the `SpinnerState` is currently not `SpinnerState::Open`.
    pub fn update(&mut self, ctx: &egui::Context) {
        self.update_ui(ctx, |_| ());
    }

    /// Main update method of the spinner that should be called every frame if you want the
    /// spinner to be visible.
    ///
    /// This method allows additional content to be displayed under the
    /// spinner - or if activated - under the elapsed time.
    /// However, note that the additional content is not taken into account when
    /// centering the spinner. Therefore, a large amount of additional
    /// content on the Y-axis is not recommended.
    ///
    /// This has no effect if the `SpinnerState` is currently not `SpinnerState::Open`.
    pub fn update_with_content(&mut self, ctx: &egui::Context, ui: impl FnOnce(&mut egui::Ui)) {
        self.update_ui(ctx, ui);
    }
}

/// UI methods
impl ModalSpinner {
    fn update_ui(&mut self, ctx: &egui::Context, content: impl FnOnce(&mut egui::Ui)) {
        if self.state != SpinnerState::Open && !self.fading_out {
            return;
        }

        let id = self.id.unwrap_or_else(|| egui::Id::from("_modal_spinner"));
        let screen_rect = ctx.input(|i| i.screen_rect);

        let opacity = ctx.animate_bool_with_easing(
            id.with("fade_out"),
            self.state == SpinnerState::Open,
            egui::emath::easing::cubic_out,
        );

        if opacity <= 0.0 && self.fading_out {
            self.fading_out = false;
            return;
        }

        let re = egui::Area::new(id)
            .movable(false)
            .fixed_pos(screen_rect.left_top())
            .fade_in(self.fade_in)
            .show(ctx, |ui| {
                if self.fading_out {
                    ui.multiply_opacity(opacity);
                }

                let fill_color = self.fill_color.unwrap_or_else(|| {
                    if ctx.style().visuals.dark_mode {
                        egui::Color32::from_black_alpha(120)
                    } else {
                        egui::Color32::from_white_alpha(40)
                    }
                });

                ui.painter()
                    .rect_filled(screen_rect, egui::Rounding::ZERO, fill_color);

                let child_ui = egui::UiBuilder::new()
                    .max_rect(screen_rect)
                    .layout(egui::Layout::top_down(egui::Align::Center));

                ui.allocate_new_ui(child_ui, |ui| {
                    self.ui_update_spinner(ui, &screen_rect);
                    content(ui);
                });
            });

        ctx.move_to_top(re.response.layer_id);
    }

    fn ui_update_spinner(&self, ui: &mut egui::Ui, screen_rect: &egui::Rect) {
        let spinner_h = self
            .spinner
            .size
            .unwrap_or_else(|| ui.style().spacing.interact_size.y);

        let mut margin = screen_rect.height() / 2.0 - spinner_h / 2.0;

        if self.show_elapsed_time {
            let height = ui.fonts(|f| f.row_height(&egui::TextStyle::Body.resolve(ui.style())));
            margin -= ui.spacing().item_spacing.y.mul_add(2.0, height / 2.0);
        }

        ui.add_space(margin);

        self.spinner.update(ui);

        if self.show_elapsed_time {
            self.ui_update_elapsed_time(ui);
        }
    }

    fn ui_update_elapsed_time(&self, ui: &mut egui::Ui) {
        ui.add_space(ui.spacing().item_spacing.y);
        ui.label(format!(
            "Elapsed: {} s",
            self.timestamp.elapsed().unwrap_or_default().as_secs()
        ));
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
#[derive(Debug, Default, Clone, PartialEq)]
struct Spinner {
    pub size: Option<f32>,
    pub color: Option<egui::Color32>,
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
