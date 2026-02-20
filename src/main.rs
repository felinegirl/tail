use std::{collections::HashMap, hash::Hash, string};

use eframe::egui::{self, Vec2};

pub mod tailui;
pub mod sourcepp;
use crate::{sourcepp::vpkpp::*, tailui::{misctools::*, settings::*, topcontextmenu::*, toyboxmenu::*, gamedatamodal::*}};
pub mod fgd;
pub mod loadassets;

fn main() {
    // openvpk("./test.vpk");
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native("tail v0.0.1",
    options,
    Box::new(|cc| {

        let mut newsesh: tail = tail::default();

        newsesh.opengamedataopened = true;

        match loadsettins(&mut newsesh) {
            Ok(_) => {},
            Err(_) => {},
        };

        

        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Box::new(newsesh))
    }));
}

#[derive(Default, PartialEq)]
enum selectiontype {
    #[default] 
    Groups, Objects, Solids 
}

#[derive(Default)]
struct tail {
    fuckyou: i32,
    nosaved: bool,
    show_extra_info: bool,

    setup_menu: bool, //for users who havn't added a config
    start_menu: bool,

    //toy stuff
    selected: selectiontype,

    //if windows are opened
    settingsopened: bool,
    gamedataopened: bool,
    opengamedataopened: bool,

    //settings
    game_directory: HashMap<u32,String>,
    selected_game_data: u32,
    game_data_names: HashMap<u32,String>,
    game_datas: HashMap<u32,HashMap<u32,String>>,
    data_selected: u32,

    global_error: String
}

impl tail {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for tail {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if self.global_error != "" {
            egui::Modal::new("fuckedup".into()).show(ctx, |ui| {
                ui.heading("error:");
                ui.label(&self.global_error);
            });
            return;
        }

        if self.start_menu {
            return;
        }

        if self.opengamedataopened{
            chosegamemodal(self, ctx);
            return;
        }

        topcontextmenu(self, ctx);
        toyboxmenu(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });

        egui::panel::SidePanel::new(egui::panel::Side::Right, "miscoption").show(ctx, |ui| {
            ui.group(|ui| {
                ui.label("Select:");

                ui.radio_value(&mut self.selected, selectiontype::Groups, "Groups");
                ui.radio_value(&mut self.selected, selectiontype::Objects, "Objects");
                ui.radio_value(&mut self.selected, selectiontype::Solids, "Solids");

                ui.take_available_width();
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

                ui.take_available_width();
            });

            if self.gamedataopened{
                gamedatamodal(self, ctx);
            }

            if self.settingsopened{
                settingswindow(self, ctx);
            }

            // egui::Window::new("properties").show(ctx, |ui| {
            //     ui.label("Hello World!");
            // });
        });
    }
}
