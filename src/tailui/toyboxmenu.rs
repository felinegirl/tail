use eframe::egui::{self, Vec2, WidgetText};

use crate::tail;

fn createtoybutton(tail: &mut tail, ctx: &egui::Context, ui: &mut egui::Ui,  img: egui::Image, title: impl Into<WidgetText>){
    ui.add(
        egui::Button::new(
            img
            .corner_radius(5)
        )
        .min_size(Vec2::new(45.0, 45.0))
        .frame(false)
    ).on_hover_text(title);
}

pub fn toyboxmenu(tail: &mut tail, ctx: &egui::Context) {
    egui::panel::SidePanel::new(egui::panel::Side::Left, "tools").show(ctx, |ui| {
            ui.label("toys :D");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/selection.png").into(), "Selection Tool");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/entity.png").into(), "Entity Tool");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/block.png").into(), "Block Tool");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/texture.png").into(), "Texture Tool");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/applytexture.png").into(), "Apply Current Texture... Tool...");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/decal.png").into(), "Apply Decal");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/overlay.png").into(), "Apply Overlay");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/clipping.png").into(), "Clipping Tool");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/vertex.png").into(), "Vertex Tool, The Fuck?");
            createtoybutton(tail, ctx, ui, egui::include_image!("../../icons/toys/polygon.png").into(), "Polygon Tool");
        });
}