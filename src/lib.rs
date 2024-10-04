use egui::Widget;

/// Wrapper above `egui::Spinner` to be able to customize trait implementations.
#[derive(Debug, Clone, PartialEq)]
struct Spinner {
    size: Option<f32>,
    color: Option<egui::Color32>,
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

#[derive(Debug, Clone, PartialEq)]
pub struct ModalSpinner {
    id: String,
    fill_color: egui::Color32,
    spinner: Spinner,
}

impl ModalSpinner {
    pub fn new() -> Self {
        Self {
            id: String::from("_modal_spinner"),
            fill_color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 120),
            spinner: Spinner::default(),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
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
