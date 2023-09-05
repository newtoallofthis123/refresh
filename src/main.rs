use notify::{RecursiveMode, Watcher};
use clap::Parser;
#[derive(Parser, Debug)]
#[command(name="Refrsh", author="Ishan Joshi", version, about="A Simple CLI for hot reloading your applications")]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false, help="Specify the directory to watch, defaults to the current directory")]
    dir: Option<String>,

    #[arg(short, long, help="Specify the extension of the files to watch")]
    extension: Option<String>,

    #[arg(short, long, help="Run and build the project", default_value="false")]
    run: bool,

    #[arg(long, help="Open the docs")]
    docs: bool,
}

mod files;
mod utils;
mod handler;

fn main() {
    human_panic::setup_panic!();
    utils::print_splash();
    let args = Args::parse();

    if args.docs{
        bunt::println!("{$cyan}Opening docs...{/$}");
        utils::open_webpage("https://noobscience.rocks/refrsh");
        std::process::exit(0)
    }

    let dir = args.dir.unwrap_or(".".to_string());
    if !utils::does_dir_exist(&dir){
        bunt::println!("{$red}Directory not found{/$}");
        std::process::exit(0)
    }
    std::env::set_current_dir(&dir).unwrap();
    let lang_tool = utils::get_lang_tool();
    let extension = args.extension.unwrap_or(String::new());
    
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

    let mut file_watcher = files::get_file_watcher(extension.clone(), args.run);
    let directory = std::path::Path::new(".");
    file_watcher.watch(directory, RecursiveMode::Recursive).unwrap();

    loop {}
}

