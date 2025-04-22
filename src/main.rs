struct ShadowGhostingArtifactExampleApp {
    do_paint: bool,
}

impl eframe::App for ShadowGhostingArtifactExampleApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Fully transparent background
        egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.do_paint = !self.do_paint;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                if self.do_paint {
                    ui.centered_and_justified(|ui| {
                        ui.label("Press space to toggle painting");
                    });
                }
            });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(300.0, 200.0))
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "Example: Ghosting Artifacts",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(ShadowGhostingArtifactExampleApp {
                do_paint: true,
            }))
        }),
    )
}
