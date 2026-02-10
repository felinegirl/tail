use eframe::egui::{self, Ui, mutex::RwLock};

use crate::{tail, tailui::openfilebrowser};

//this maybe stupid way of going around this, but having all of this inside the tail struct sounds bad
//do not let the pub know
lazy_static::lazy_static! {

    static ref csm: RwLock<CurrentSettingMenu> = RwLock::new(CurrentSettingMenu::GameConfigs);

}

#[derive(Default, PartialEq, Debug)]
enum CurrentSettingMenu {
    #[default] 
    Video,
    GameConfigs,
    BuildPrograms
}

fn gameconfigmenu(tail: &mut tail, ctx: &egui::Context, ui: &mut Ui) {

    let mut config = egui::Frame::default().inner_margin(15.0).begin(ui);
        ui.label("Configuration");
        ui.horizontal(|ui|{
            let mut selected = 0;

            if(!tail.game_data_names.is_empty()){
                egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", tail.game_data_names.get(&tail.selected_game_data).unwrap()))
                    .show_ui(ui, |ui| {
                        for (key, game) in &tail.game_data_names {
                            ui.selectable_value(&mut tail.selected_game_data, *key, game);
                        }
                    }
                );
            }else{
                ui.label("there's nothing... wtf...");
            }
        
            if ui.button("Edit").clicked() {
                tail.gamedataopened = true;
            };
        });
    config.end(ui);


    if(tail.game_data_names.is_empty()) {return};


    let mut gamedata = egui::Frame::default().inner_margin(15.0).begin(ui);
        ui.label("Game Data");
        ui.group(|ui| {
            match tail.game_datas.get(&tail.selected_game_data) {
                Some(a) => {
                    for (key, dir) in a {
                        ui.selectable_value(&mut tail.data_selected, *key, dir);
                    }
                },
                None => {},
            }
            
        });
    gamedata.end(ui);

    let mut defaultentiry = egui::Frame::default().inner_margin(15.0).begin(ui);
    ui.horizontal(|ui|{
        ui.vertical(|ui|{
            ui.label("Default PointEntity");
        });
        ui.vertical(|ui|{
            ui.label("Default SolidEntity");
        });
    });
    defaultentiry.end(ui);

    let mut gamedirect = egui::Frame::default().inner_margin(15.0).begin(ui);
    ui.label("Game Ditectory (where GameInfo.txt is):");
    ui.horizontal(|ui|{
        ui.text_edit_singleline(&mut tail.game_directory);
        if ui.button("Browse").clicked() {
            openfilebrowser(".".to_string(), &mut tail.game_directory);
        }
    });
    gamedirect.end(ui);

    ui.separator();
    ui.horizontal(|ui|{
        ui.hyperlink_to("help...", "https://meowingbunny.neocities.org/");
        if ui.button("close").clicked() {
            tail.settingsopened = false;
        };
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