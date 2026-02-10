pub mod topcontextmenu;

pub mod toyboxmenu;

pub mod misctools;

pub mod settings;

pub mod gamedatamodal;

use rfd::FileDialog;

fn openfilebrowser(path: String, editpath: &mut String){
    *editpath = FileDialog::new()
    .add_filter("text", &["txt"])
    .set_directory(".")
    .pick_file()
    .unwrap()
    .into_os_string().into_string().unwrap();
}