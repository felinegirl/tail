use eframe::egui::{self, Vec2};

use crate::tailui::{topcontextmenu::*, toyboxmenu::*, misctools::*};

pub mod tailui;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("tail v0.0.1",
    options,
    Box::new(|cc| {

        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::<tail>::default())
    }));
}

#[derive(PartialEq)]
enum selectiontype { Groups, Objects, Solids }

#[derive(Default)]
struct tail {
    fuckyou: i32,
    nosaved: bool,
    show_extra_info: bool,
}

impl tail {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for tail {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        topcontextmenu(self, ctx);
        toyboxmenu(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });

        egui::panel::SidePanel::new(egui::panel::Side::Right, "miscoption").show(ctx, |ui| {
            ui.group(|ui| {
                ui.label("Select:");

                let mut selected = selectiontype::Groups;
                ui.radio_value(&mut selected, selectiontype::Groups, "Groups");
                ui.radio_value(&mut selected, selectiontype::Objects, "Objects");
                ui.radio_value(&mut selected, selectiontype::Objects, "Solids");
            });

            ui.group( |ui| {
                ui.label("Current texture:");

                ui.label("blah/blah/blah005b");
                ui.horizontal(|ui| {
                    let logo = ui.add(
                        egui::Image::new(egui::include_image!("../placeholders/thumbnail_wtf.jpeg.jpg"))
                        .fit_to_exact_size(Vec2::new(100.0,100.0))
                    );
                    ui.vertical(|ui| {
                        ui.group(|ui| {
                        ui.label("64 x 64");
                        ui.button("Browse...");
                        ui.button("Replace...");

                        ui.set_min_width(100.0);
                    });
                    });
                });
            });

            ui.group( |ui| {


                ui.label("Objects:");
                ui.label("block");
            });

            egui::Window::new("properties").show(ctx, |ui| {
                ui.label("Hello World!");
            });
        });
    }
}
