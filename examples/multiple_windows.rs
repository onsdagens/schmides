use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Reader",
        options,
        Box::new(|cc| Ok(Box::<SlideViewer>::default())),
    )
}

struct SlideViewer {
    page_counter: usize,
    channel: (Sender<egui::Key>, Receiver<egui::Key>),
}

impl Default for SlideViewer {
    fn default() -> Self {
        SlideViewer {
            page_counter: 0,
            channel: mpsc::channel(),
        }
    }
}

impl eframe::App for SlideViewer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let tx = self.channel.0.clone();
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Slide Viewer");
            ui.label("Hello World!");
            ui.label(format!("{}", self.page_counter));
            ui.ctx().input(|i| {
                if i.key_released(egui::Key::ArrowLeft) {
                    let _ = tx.send(egui::Key::ArrowLeft);
                }
                if i.key_released(egui::Key::ArrowRight) {
                    let _ = tx.send(egui::Key::ArrowRight);
                }
            });
        });
        let counter_clone = self.page_counter.clone();
        // The content of both windows is coupled (by the page no.)
        //  so it makes sense for this to be immediate
        ui.ctx().show_viewport_immediate(
            egui::ViewportId::from_hash_of("deferred_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Deferred Viewport")
                .with_inner_size([200.0, 100.0]),
            move |ui, _| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.heading("Deferred Window");
                    ui.label("Hello! :D");
                    ui.label(format!("Page: {}", counter_clone));
                    ui.ctx().input(|i| {
                        if i.key_released(egui::Key::ArrowLeft) {
                            let _ = tx.send(egui::Key::ArrowLeft);
                        }
                        if i.key_released(egui::Key::ArrowRight) {
                            let _ = tx.send(egui::Key::ArrowRight);
                        }
                    });
                });
            },
        );

        while let Ok(v) = self.channel.1.try_recv() {
            match v {
                egui::Key::ArrowLeft => {
                    self.page_counter -= 1;
                }
                egui::Key::ArrowRight => {
                    self.page_counter += 1;
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}
