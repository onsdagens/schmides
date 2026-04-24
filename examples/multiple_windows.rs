use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Reader",
        options,
        Box::new(|_| Ok(Box::<SlideViewer>::default())),
    )
}

struct SlideViewer {
    page_counter: u32,
}

impl Default for SlideViewer {
    fn default() -> Self {
        SlideViewer { page_counter: 0 }
    }
}

impl eframe::App for SlideViewer {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let mut delta: i64 = 0;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Notes");
            ui.label("This is the main view.");
            ui.label("Press the left/right arrows to decrease/increase the counter");
            ui.label(format!("{}", self.page_counter));
            ui.ctx().input(|i| {
                if i.key_released(egui::Key::ArrowLeft) {
                    delta -= 1;
                }
                if i.key_released(egui::Key::ArrowRight) {
                    delta += 1;
                }
            });
        });
        // The content of both windows is coupled (by the page no.)
        //  so it makes sense for this to be immediate
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("slides"),
            egui::ViewportBuilder::default()
                .with_title("Slides")
                .with_inner_size([200.0, 100.0]),
            |ui, _| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.label("This is a child viewport.");
                    ui.label("You can decrease the same counter from this window as well");
                    ui.label(format!("Page: {}", self.page_counter));
                    ui.ctx().input(|i| {
                        if i.key_released(egui::Key::ArrowLeft) {
                            delta -= 1;
                        }
                        if i.key_released(egui::Key::ArrowRight) {
                            delta += 1;
                        }
                    });
                });
            },
        );
        self.page_counter = (self.page_counter as i64 + delta).clamp(0, i64::MAX) as u32;
    }
}
