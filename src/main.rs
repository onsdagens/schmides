use eframe::egui;
use std::{fs, path::PathBuf};

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
            Ok(Box::<SlideViewer>::default())
        }),
    )
}

struct SlideViewer {
    first_slide: u32,
    last_slide: u32,
    page_counter: u32,
    path: Option<PathBuf>,
}

impl Default for SlideViewer {
    fn default() -> Self {
        SlideViewer {
            first_slide: 1,
            last_slide: 1,
            page_counter: 1,
            path: None,
        }
    }
}

impl eframe::App for SlideViewer {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let mut delta: i64 = 0;
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Notes");
            ui.label("This will in the future be the presenter notes view.");
            ui.label("Press the left/right arrows to move around the slides");
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
                    if ui.button("Open directory").clicked() {
                        self.path = rfd::FileDialog::new().pick_folder();
                        if let Some(path) = &self.path {
                            // Is this fallible?
                            for f in fs::read_dir(path).unwrap() {
                                if let Ok(p) = f {
                                    let name = p.file_name();
                                    let name_str = name.to_string_lossy();
                                    println!("{}", name_str);
                                    let name_ext: Vec<&str> = name_str.split('.').collect();
                                    // We expect a file name and an extension, loose periods in the name are disallowed for now
                                    if name_ext.len() != 2 {
                                        todo!("Directory contains files with multiple periods in name, or no extension");
                                    }
                                    if name_ext[1] != "svg" {
                                        // Really we could just ignore files that are not svgs
                                        // but for now let's be "proper"
                                        todo!("Directory containes files that are not .svg");
                                    }
                                    if let Ok(index) = name_ext[0].parse::<u32>() {
                                        self.first_slide = self.first_slide.min(index);
                                        self.last_slide = self.last_slide.max(index);
                                    } else {
                                        // Same again, we could just ignore these
                                        todo!("Directory contains filenames not on the form <page no.>.svg");
                                    }
                                }
                            }
                        }
                    }
                    if let Some(path) = &self.path {
                        // The string gymnastics seem stupid, there has to be a better way
                        // We can also cache the image on open,
                        //  i believe egui does this automagically
                        ui.image(format!(
                            "file://{}/{}.svg",
                            path.as_path().as_os_str().to_string_lossy(), self.page_counter
                        ));
                    }

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
        self.page_counter =
            ((self.page_counter as i64 + delta) as u32).clamp(self.first_slide, self.last_slide);
    }
}
