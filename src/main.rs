use notify::{RecursiveMode, Watcher};
use clap::Parser;
#[derive(Parser, Debug)]
#[command(name="HTML Reload", author="Ishan Joshi", version, about="A Simple CLI for reloading HTML pages")]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false)]
    dir: Option<String>,
}

mod files;
mod utils;
mod handler;

fn main() {
    human_panic::setup_panic!();
    utils::print_splash();
    let args = Args::parse();
    let dir = args.dir.unwrap_or(".".to_string());
    if !utils::does_dir_exist(&dir){
        bunt::println!("{$red}Directory not found{/$}");
        std::process::exit(0)
    }
    std::env::set_current_dir(&dir).unwrap();
    let lang_tool = utils::get_lang_tool();
    
    if lang_tool != "Err"{
        bunt::println!("Language Tool {$blue}{}{/$} detected", lang_tool);
        match lang_tool.clone().as_str(){
            "Rust" => {
                bunt::println!("Using {$yellow}Cargo {/$}{}...learn more in the docs", utils::get_cargo_version())
            }
            "Go" => {
                bunt::println!("Using {$yellow}Go {/$}{}...learn more in the docs", utils::get_go_version())
            }
            _ => {
                bunt::println!("{$red}Language Tool not supported{/$}");
            }
        };
    }
    else{
        bunt::println!("{$red}Language Tool not detected{/$}");
        std::process::exit(0)
    }
    if dir == "."{
        bunt::println!("Watching the {$green}current{/$} (.) directory");
    }
    else{
        bunt::println!("Watching directory {$green}{}{/$}", dir);
    }

    let mut file_watcher = files::get_file_watcher();
    let directory = std::path::Path::new(".");
    file_watcher.watch(directory, RecursiveMode::Recursive).unwrap();

    loop {}
}

