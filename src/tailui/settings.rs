use eframe::egui::{self, Ui, mutex::RwLock};

use crate::tail;

//this maybe stupid way of going around this, but having all of this inside the tail struct sounds bad
//do not let the pub know
lazy_static::lazy_static! {

    static ref csm: RwLock<CurrentSettingMenu> = RwLock::new(CurrentSettingMenu::GameConfigs);

}

#[derive(Default, PartialEq)]
enum CurrentSettingMenu {
    #[default] 
    Video,
    GameConfigs,
    BuildPrograms
}

fn gameconfigmenu(tail: &mut tail, ctx: &egui::Context, ui: &mut Ui) {
    ui.label("Configuration");
    


    ui.label("Game Data");


    ui.horizontal(|ui|{
        ui.hyperlink_to("help...", "https://meowingbunny.neocities.org/");
        ui.button("save");
        ui.button("no...");
    });
}

pub fn settingswindow(tail: &mut tail, ctx: &egui::Context){
    let mut csmz = csm.write();

    egui::Window::new("settings").show(ctx, |ui| {
        //top
        ui.group(|ui|{
            ui.horizontal(|ui| {
                ui.selectable_value(&mut *csmz, CurrentSettingMenu::Video, "Video");
                ui.selectable_value(&mut *csmz, CurrentSettingMenu::GameConfigs, "Game Configs");
                ui.selectable_value(&mut *csmz, CurrentSettingMenu::BuildPrograms, "Build Programs");
            });
        });

        match *csmz {
            // CurrentSettingMenu::Video => todo!(),
            CurrentSettingMenu::GameConfigs => gameconfigmenu(tail, ctx, ui),
            // CurrentSettingMenu::BuildPrograms => todo!(),
            _ => {}
        }
        
        // ui.set_height(500.0);
    });
}