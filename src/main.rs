use eframe::egui;
use egui::Ui;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

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
    slides_full_screen: bool,
    show_slides_window: bool,
    notes: Vec<String>,
    notes_offset: i32,
}

impl Default for SlideViewer {
    fn default() -> Self {
        SlideViewer {
            first_slide: 1,
            last_slide: 1,
            page_counter: 1,
            path: None,
            slides_full_screen: false,
            show_slides_window: true,
            notes: vec![],
            // The slides are one-indexed
            notes_offset: -1,
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
            if ui.button("Open Typst file with notes").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.parse_notes(path);
                }
            }
            ui.horizontal(|ui|{
                
                ui.label(format!("Page: {}", self.page_counter));
                ui.label(format!("Notes offset: {}", -self.notes_offset));
                if ui.button("+").clicked() {
                    self.notes_offset -= 1;
                }
                if ui.button("-").clicked() {
                    self.notes_offset += 1;
                }
            });
            if let Some(note) = self.notes.get_mut((self.page_counter as i32 + self.notes_offset) as usize ){
                ui.text_edit_multiline(note);
            }
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
        //let vb = if self.slides_full_screen
        if self.show_slides_window {
            self.show_slides_window(ui, &mut delta);
        }
        self.page_counter =
            ((self.page_counter as i64 + delta) as u32).clamp(self.first_slide, self.last_slide);
    }
}

impl SlideViewer {
    fn show_slides_window(&mut self, ui: &mut Ui, delta: &mut i64) {
        let builder = if self.slides_full_screen {
            egui::ViewportBuilder::default()
                .with_title("Slides")
                .with_fullscreen(true)
        } else {
            egui::ViewportBuilder::default()
                .with_title("Slides")
                .with_min_inner_size([100.0, 100.0])
                .with_resizable(true)
        };
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("slides"),
            builder,
            |ui, _| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    if let Some(path) = &self.path {
                        // The string gymnastics seem stupid, there has to be a better way
                        // We can also cache the image on open,
                        //  i believe egui does this automagically
                        ui.image(format!(
                            "file://{}/{}.svg",
                            path.as_path().as_os_str().to_string_lossy(),
                            self.page_counter
                        ));
                    }
                    ui.ctx().input(|i| {
                        if i.viewport().close_requested() {
                            self.show_slides_window = false;
                        }
                        if let Some(b) = i.viewport().fullscreen {
                            if b {
                                self.slides_full_screen = true;
                            } else {
                                self.slides_full_screen = false;
                            }
                        }
                        if let Some(b) = i.viewport().maximized {
                            if b {
                                self.slides_full_screen = true;
                            } else {
                                self.slides_full_screen = false;
                            }
                        }
                        if i.key_released(egui::Key::ArrowLeft) {
                            *delta -= 1;
                        }
                        if i.key_released(egui::Key::ArrowRight) {
                            *delta += 1;
                        }
                    });
                });
            },
        );
    }

    fn parse_notes(&mut self, path: PathBuf) {
        // this should not be fallible in normal conditions
        let mut f = File::open(path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).ok();
        while let Some(tag_idx) = contents.find("#speaker-note[") {
            // tag length is 14
            contents = contents.split_off(tag_idx + 14);
            // as we know, closing square brackets are forbidden in
            //  speaker notes, even when escaped
            if let Some(end_idx) = contents.find(']') {
                let mut note = contents.clone();
                contents = note.split_off(end_idx);
                self.notes.push(note);
            }
        }
    }
}
