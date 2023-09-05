use std::fs;
use notify::{Error, Event, RecommendedWatcher, Watcher};

use crate::utils;

pub fn get_all_file_names() -> Vec<String> {
    let paths = fs::read_dir(".").unwrap();
    let names = paths.map(|path| path.unwrap().path().display().to_string()).collect::<Vec<String>>();
    names
}

pub fn get_file_watcher()->notify::ReadDirectoryChangesWatcher{
    let watcher: notify::ReadDirectoryChangesWatcher = RecommendedWatcher::new(move |result: Result<Event, Error>|{
        let event = result.expect("Failed to read event");

        if event.kind.is_modify(){
            if !utils::is_dir(&event.paths[0]){
                bunt::println!("Change Detected in {$yellow}{}{/$}, reloading", utils::display_file_name(&event.paths[0]));
                crate::handler::handle_build(&utils::get_lang_tool());
            }
        }
    },notify::Config::default()).unwrap();
    watcher
}