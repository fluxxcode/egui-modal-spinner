#[derive(Debug, Clone, PartialEq)]
pub struct ModalSpinner {
    fill_color: egui::Color32,
}

impl ModalSpinner {
    pub fn new() -> Self {
        Self {
            fill_color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 120),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let window_fill = ctx.style().visuals.window_fill;
        let screen_rect = ctx.input(|i| i.screen_rect);

        ctx.style_mut(|s| s.visuals.window_fill = self.fill_color);

        let re = egui::Window::new("TODO: Add a proper ID!")
            .interactable(false)
            .title_bar(false)
            .fixed_size(screen_rect.size())
            .fixed_pos(screen_rect.left_top())
            .show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.spinner();
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
