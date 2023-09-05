use std::fs;
use notify::{Error, Event, RecommendedWatcher, Watcher};

use crate::utils;

pub fn get_all_file_names() -> Vec<String> {
    let paths = fs::read_dir(".").unwrap();
    let names = paths.map(|path| path.unwrap().path().display().to_string()).collect::<Vec<String>>();
    names
}

pub fn get_file_watcher(ext: String, run: bool)->notify::ReadDirectoryChangesWatcher{
    let watcher: notify::ReadDirectoryChangesWatcher = RecommendedWatcher::new(move |result: Result<Event, Error>|{
        let event = result.expect("Failed to read event");
        let ignore_files = utils::get_ignore_files();

        if event.kind.is_modify(){
            if !ext.is_empty(){
                if !event.paths[0].to_str().unwrap().ends_with(&ext){
                    return;
                }
            }
            for file in ignore_files.clone(){
                if event.paths[0].to_str().unwrap().contains(&file){
                    return;
                }
            }
            if !utils::is_dir(&event.paths[0]){
                bunt::println!("{$blue}[!]{/$} Change Detected in {$yellow}{}{/$}, reloading", utils::display_file_name(&event.paths[0]));
                crate::handler::handle_build(&utils::get_lang_tool(), run);
            }
        }
    },notify::Config::default()).unwrap();
    watcher
}