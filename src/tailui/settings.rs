use std::{collections::HashMap, fs::File, io::{Read, Write}};

use eframe::egui::{self, Direction, Ui, mutex::RwLock};
use rfd::FileDialog;
use serde_json::{Value, json};

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

pub fn loadsettins(tail: &mut tail) -> Result<(), Box<dyn std::error::Error>> {

    // its probably a liitle late to be discovering "?" but no better time then the present; at some point gotta refactor the code with "?"

    let mut file = File::open("./settings.json")?;

    let mut yippee = String::new(); 

    file.read_to_string(&mut yippee)?;
    
    let setjson: Value = serde_json::from_str(&yippee)?;

    //setting last selected game
    tail.selected_game_data = match setjson["last_selected_game"].as_i64() {
        Some(a) => a as u32,
        None => 0,
    };
    

    //reading game configs

    let ba = setjson["game_configs"]
    .as_array()
    .ok_or("missing game_configs")?;

    for gc in ba {
        let nextintery = (tail.game_data_names.len()) as u32;
        let gdname = gc["name"].as_str()
        .ok_or("fucked config name")?;
        tail.game_data_names.insert(
            nextintery, 
            gdname.to_string()
        );

        tail.game_datas.insert(nextintery, {

            let mut a = HashMap::new();
            let game_data = gc["game_data"]
            .as_array()
            .ok_or(format!("fucked up game_data in #{}", nextintery))?;
        
            let mut fuck: u32 = 0; // why did I hash map and not vec?????
            for kc in game_data {
                let kc = kc.as_str()
                    .ok_or(format!("fucked gamedata dir at {}", gdname))?;
                a.insert(fuck, kc.to_string());
                fuck+=1;
            };

            a
        });

        tail.game_directory.insert(
            nextintery, 
            gc["game_directory"].as_str()
            .ok_or(format!("fucked game dir at {}", gdname))?
            .to_string()
        );

    };
    
    Ok(())
}

fn savesettins(tail: &tail) -> Result<(), std::io::Error>{
    

    let mut game_config: Vec<Value> = Vec::new();
    for (ket, gd) in (&tail.game_data_names) {

        let gds = {
            let mut gds: Vec<String> = Vec::new();
            
            for gd in tail.game_datas.get(ket).unwrap() {
                gds.push(gd.1.to_string());
            };

            gds
        };

        game_config.push(json!({
            "name": gd,
            "game_data": gds,
            "game_directory": tail.game_directory.get(ket).unwrap()
        }));
        
    }

    let export = json!({
        "version": "0.0.1",
        "last_selected_game": tail.selected_game_data,
        "game_configs": game_config
    });

    let readyexport = serde_json::to_string_pretty(&export).unwrap();

    let mut file = File::create("./settings.json").unwrap();
    file.write_all(readyexport.as_bytes());

    Ok(())
}

fn gameconfigmenu(tail: &mut tail, ctx: &egui::Context, ui: &mut Ui) {

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


    if(tail.game_data_names.is_empty()) {return};


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

            ui.take_available_width();
            
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {

            if ui.button("remove").clicked() {
                let  a = tail.game_datas.get_mut(&tail.selected_game_data).unwrap();
                a.remove(&tail.data_selected);
                tail.data_selected = 0;
            }

            if ui.button("add").clicked() {
                let newgd = match FileDialog::new()
                .add_filter("fgd", &["fgd"])
                .set_directory(".")
                .pick_file(){
                    Some(a) => a.into_os_string().into_string().unwrap(),
                    None => return,
                };

                let  a = tail.game_datas.get_mut(&tail.selected_game_data).unwrap();
                let aleng = a.len() as u32;
                a.insert(aleng, newgd);
            }

            ui.take_available_width();
        });
    
    ui.horizontal(|ui|{
        ui.vertical(|ui|{
            ui.label("Default PointEntity");
            egui::ComboBox::from_label("")
                .selected_text(format!("func_blah"));
                // .show_ui(ui, |ui| {
                //     ui.selectable_value(&mut 0, 0, "awa");
                // }
            // );
        });
        ui.vertical(|ui|{
            ui.label("Default SolidEntity");
            egui::ComboBox::from_label("")
                .selected_text(format!("logic_death"));
                // .show_ui(ui, |ui| {
                //     ui.selectable_value(&mut 0, 0, "nya");
                // }
            // );
        });
    });

    ui.label("Game Ditectory (where GameInfo.txt is):");
    ui.horizontal(|ui|{
        let gdd = tail.game_directory.get_mut(&tail.selected_game_data).unwrap();
        ui.text_edit_singleline(gdd);
        if ui.button("Browse").clicked() {
            *gdd = match FileDialog::new()
            .set_directory(".")
            .pick_folder() {
                Some(a) => a.into_os_string().into_string().unwrap(),
                None => return,
            }
        }
        ui.take_available_width();
    });

}

pub fn settingswindow(tail: &mut tail, ctx: &egui::Context){
    let mut csmz = csm.write();

    egui::Window::new("settings")
    .collapsible(false)
    .show(ctx, |ui| {
        //top
        ui.group(|ui|{
            ui.horizontal(|ui| {
                ui.selectable_value(&mut *csmz, CurrentSettingMenu::Video, "Video");
                ui.selectable_value(&mut *csmz, CurrentSettingMenu::GameConfigs, "Game Configs");
                ui.selectable_value(&mut *csmz, CurrentSettingMenu::BuildPrograms, "Build Programs");
            });
            ui.take_available_width();
        });

        match *csmz {
            // CurrentSettingMenu::Video => todo!(),
            CurrentSettingMenu::GameConfigs => gameconfigmenu(tail, ctx, ui),
            // CurrentSettingMenu::BuildPrograms => todo!(),
            _ => {}
        }

        ui.separator();
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("close").clicked() {
                tail.settingsopened = false;
            };
            if ui.button("save").clicked() {
                savesettins(tail);
            };
            ui.hyperlink_to("help...", "https://meowingbunny.neocities.org/");

            ui.take_available_width();
        });
        
        // ui.set_height(500.0);
    });
}