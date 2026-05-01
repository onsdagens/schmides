use eframe::egui;
use egui::Vec2;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 500.0]),
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
    slides_show_top: bool,
    show_slides_window: bool,
    notes: Vec<String>,
    notes_offset: i32,
    show_notes_ui: bool,
}

impl Default for SlideViewer {
    fn default() -> Self {
        SlideViewer {
            first_slide: 1,
            last_slide: 1,
            page_counter: 1,
            path: None,
            slides_show_top: true,
            show_slides_window: true,
            notes: vec![],
            // The slides are one-indexed
            notes_offset: -1,
            show_notes_ui: true,
        }
    }
}

impl eframe::App for SlideViewer {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        let mut delta: i64 = 0;
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.show_notes_ui {
                ui.heading("Notes");
                ui.label("This will in the future be the presenter notes view.");
                ui.label("Press the left/right arrows to move around the slides");
                if ui.button("Open directory").clicked() {
                    self.open_slides(ctx);
                }
                if ui.button("Open Typst file with notes").clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    self.parse_notes(path);
                }
                if ui.button("Hide UI (H)").clicked() {
                    self.show_notes_ui = false;
                }
                if ui.button("Hide Slides Window Decorations(D)").clicked() {
                    self.slides_show_top = false;
                }
                ui.horizontal(|ui| {
                    ui.label(format!("Page: {}", self.page_counter));
                    ui.label(format!("Notes offset: {}", -self.notes_offset));
                    if ui.button("+").clicked() {
                        self.notes_offset -= 1;
                    }
                    if ui.button("-").clicked() {
                        self.notes_offset += 1;
                    }
                });
            }
            if let Some(note) = self
                .notes
                .get_mut((self.page_counter as i32 + self.notes_offset) as usize)
            {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::TextEdit::multiline(note).interactive(false).show(ui);
                });
            }
            ui.ctx().input(|i| {
                if i.key_released(egui::Key::ArrowLeft) {
                    delta -= 1;
                }
                if i.key_released(egui::Key::ArrowRight) {
                    delta += 1;
                }
                if i.key_released(egui::Key::H) {
                    self.show_notes_ui = !self.show_notes_ui;
                }
                if i.key_released(egui::Key::D) {
                    self.slides_show_top = !self.slides_show_top;
                }
            });
        });
        // The content of both windows is coupled (by the page no.)
        //  so it makes sense for this to be immediate
        if self.show_slides_window {
            self.show_slides_window(ctx, &mut delta);
        }
        self.page_counter =
            ((self.page_counter as i64 + delta) as u32).clamp(self.first_slide, self.last_slide);
    }
}

impl SlideViewer {
    fn open_slides(&mut self, ctx: &eframe::egui::Context) {
        self.path = rfd::FileDialog::new().pick_folder();
        if let Some(path) = &self.path {
            // Is this fallible?
            for p in fs::read_dir(path).unwrap().flatten() {
                let name = p.file_name();
                let name_str = name.to_string_lossy();
                println!("{}", name_str);
                let name_ext: Vec<&str> = name_str.split('.').collect();
                // We expect a file name and an extension, loose periods in the name are disallowed for now
                if name_ext.len() != 2 {
                    todo!(
                        "Directory contains files with multiple periods in name, or no extension"
                    );
                }
                if name_ext[1] != "svg" {
                    // Really we could just ignore files that are not svgs
                    // but for now let's be "proper"
                    todo!("Directory containes files that are not .svg");
                }
                if let Ok(index) = name_ext[0].parse::<u32>() {
                    // This is a valid slide file, preload it
                    // Actually this doesn't seem to do much but let's keep it here
                    ctx.try_load_image(
                        &format!("file://{}", p.path().to_string_lossy()),
                        // Rasterize the SVG to 4k(??), should be good enough
                        egui::SizeHint::Width(3840),
                    )
                    .ok();
                    self.first_slide = self.first_slide.min(index);
                    self.last_slide = self.last_slide.max(index);
                } else {
                    // Same again, we could just ignore these
                    todo!("Directory contains filenames not on the form <page no.>.svg");
                }
            }
        }
    }
    fn show_slides_window(&mut self, ctx: &eframe::egui::Context, delta: &mut i64) {
        let builder = if self.slides_show_top {
            egui::ViewportBuilder::default()
                .with_title("Slides")
                .with_min_inner_size([500.0, 400.0])
                .with_resizable(true)
                .with_decorations(true)
        } else {
            egui::ViewportBuilder::default()
                .with_title("Slides")
                .with_min_inner_size([500.0, 400.0])
                .with_resizable(true)
                .with_decorations(false)
        };
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("slides"),
            builder,
            |ctx, _| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if let Some(path) = &self.path {
                        // Another attempt at prerasterizing the SVGs
                        // Note, these should be cached by egui, so this
                        // isn't as bad as it looks
                        for s in self.first_slide..=self.last_slide {
                            let i = egui::Image::new(format!(
                                "file://{}/{}.svg",
                                path.to_string_lossy(),
                                s
                            ));
                            i.load_for_size(ctx, Vec2::new(3840.0, 2160.0)).ok();
                        }
                        let i = egui::Image::new(format!(
                            "file://{}/{}.svg",
                            path.to_string_lossy(),
                            self.page_counter
                        ));
                        i.load_for_size(ctx, Vec2::new(3840.0, 2160.0)).unwrap();
                        ui.centered_and_justified(|ui| ui.add(i));
                    }
                    ctx.input(|i| {
                        if i.viewport().close_requested() {
                            self.show_slides_window = false;
                        }
                        if i.key_released(egui::Key::ArrowLeft) {
                            *delta -= 1;
                        }
                        if i.key_released(egui::Key::ArrowRight) {
                            *delta += 1;
                        }
                        if i.key_released(egui::Key::H) {
                            self.show_notes_ui = !self.show_notes_ui;
                        }
                        if i.key_released(egui::Key::D) {
                            self.slides_show_top = !self.slides_show_top;
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
