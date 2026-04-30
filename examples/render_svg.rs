use std::path::PathBuf;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Reader",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<RenderSvg>::default())
        }),
    )
}

struct RenderSvg {
    path: Option<PathBuf>,
}

impl Default for RenderSvg {
    fn default() -> Self {
        RenderSvg { path: None }
    }
}

impl eframe::App for RenderSvg {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if ui.button("Open file").clicked() {
                self.path = rfd::FileDialog::new().pick_file()
            }
            if let Some(path) = &self.path {
                // The string gymnastics seem stupid, there has to be a better way
                // We can also cache the image on open,
                //  i believe egui does this automagically
                ui.image(format!(
                    "file://{}",
                    path.as_path().as_os_str().to_string_lossy()
                ));
            }
        });
    }
}
