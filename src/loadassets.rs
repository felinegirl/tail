use std::fmt::format;

use crate::{fgd::open, tail};



pub fn loadfgd(tail: &mut tail) -> Result<(), Box<dyn std::error::Error>> {
    for (index, fgdpath) in tail.game_datas.get(&tail.selected_game_data).ok_or("err getting gamedata path")? {
        match open(fgdpath) {
            Ok(_) => (),
            Err(a) => {
                tail.global_error = format!(
                    "while parsing fgd:\n '{}'\n\n{}",
                    fgdpath,
                    a.to_string()
                );
                dbg!(a);
            },
        };
    };
    Ok(())
}