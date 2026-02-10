use std::{collections::HashMap, fmt::format, sync::RwLock};

use eframe::egui::{self, Color32};

use crate::tail;


lazy_static::lazy_static! {

    static ref selectedgame: RwLock<u32> = RwLock::new(0);

    static ref error: RwLock<String> = RwLock::new(format!(""));

    static ref newentry: RwLock<String> = RwLock::new(String::new());

}
    

pub fn gamedatamodal(tail: &mut tail, ctx: &egui::Context){
    egui::Modal::new("Edit Game Config :3".into()).show(ctx, |ui| {
        let mut errorz = error.write().unwrap();

        if(*errorz != ""){
            ui.group(|ui|{
                ui.label(&*errorz);
                ui.visuals_mut().text_edit_bg_color = Some(Color32::LIGHT_RED);
            });
        }

        ui.horizontal(|ui|{
            let mut newentryz = newentry.write().unwrap();

            ui.text_edit_singleline(&mut *newentryz);
            if ui.button("add").clicked() {

                if(*newentryz==""){
                    *errorz = format!("can't have empty name entry");
                    return;
                }

                let nextintery = (tail.game_data_names.len()) as u32;
                tail.game_data_names.insert(nextintery, newentryz.to_string());
                *newentryz = format!("");

                tail.game_datas.insert(nextintery, HashMap::new());
            }
        });

        if(tail.game_data_names.is_empty()) {
            ui.disable();
        };

        ui.horizontal(|ui|{
            let mut selectedgamez = selectedgame.write().unwrap();

            ui.vertical(|ui|{
                ui.group(|ui|{
                    if(tail.game_data_names.is_empty()) {return};
                    for (key, game) in &tail.game_data_names {
                        ui.selectable_value(&mut *selectedgamez, *key, game);
                    }
                    ui.set_min_width(100.0);
                });
            });
            ui.vertical(|ui|{
                if ui.button("remove").clicked(){
                    tail.game_data_names.remove_entry(&selectedgamez);
                    tail.game_datas.remove_entry(&selectedgamez);
                    tail.data_selected = 0;
                    *selectedgamez=0;
                };
                if ui.button("copy").clicked(){
                    let nextintery = (tail.game_data_names.len()) as u32;
                    tail.game_data_names.insert(nextintery, tail.game_data_names.get(&selectedgamez).unwrap().to_string());

                    tail.game_datas.insert(nextintery, tail.game_datas.get(&selectedgamez).unwrap().clone());
                };
            });
        });

        if ui.button("close").clicked() {
            tail.gamedataopened = false;
        };
    });
}