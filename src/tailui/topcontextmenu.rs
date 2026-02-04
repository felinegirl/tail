use eframe::egui;

use crate::tail;

pub fn topcontextmenu(tail: &mut tail, ctx: &egui::Context) {
    egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
            let logo = ui.add(
                egui::Image::new(egui::include_image!("../../taillogo.png"))
                .max_width(200.0)
                .corner_radius(2),
            );
            logo.on_hover_text("FUCK YOU!");

            ui.button("File");
            ui.button("Edit");
            ui.button("Map");
            ui.button("View");
            ui.button("Tools");
            ui.button("Instancing");
            ui.button("Window");
            ui.button("Help");
            });
        });
}