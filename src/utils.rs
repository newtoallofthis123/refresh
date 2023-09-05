use super::files;

pub fn get_lang_tool() -> String {
    let file_names = files::get_all_file_names();
    let mut lang_tool = String::new();

    for file in file_names {
        match file.as_str() {
            ".\\Cargo.toml" => {
                lang_tool = "Rust".to_string();
                break;
            },
            ".\\go.mod" => {
                lang_tool = "Go".to_string();
                break;
            },
            _ => {
                lang_tool = "Err".to_string();
            }
        }   
    }
    lang_tool
}

pub fn display_file_name(filepath: &std::path::Path) -> String {
    let last_parent = filepath.components().rev().nth(1).unwrap().as_os_str().to_str().unwrap();
    let file_name = filepath.file_name().unwrap();
    let file = format!("{}/{}", last_parent, file_name.to_str().unwrap());
    file.to_string()
}

pub fn is_dir(dir: &std::path::PathBuf) -> bool {
    let path = std::path::Path::new(dir);
    let is_dir = path.is_dir();
    is_dir
}

pub fn get_cargo_version() -> String {
    let output = std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .expect("Cargo ToolChain not installed");
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.split(" ").collect::<Vec<&str>>();
    let output = output[1].to_string();
    output
}

pub fn get_go_version() -> String {
    let output = std::process::Command::new("go")
        .arg("version")
        .output()
        .expect("GO ToolChain not installed");
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.split(" ").collect::<Vec<&str>>();
    let output = output[2].to_string();
    output
}

pub fn does_dir_exist(path: &str) -> bool {
    let dir = std::path::Path::new(path);
    let dir = dir.is_dir();
    dir
}

pub fn get_ignore_files()->Vec<String>{
    let mut ignore_files = Vec::new();
    if std::path::Path::new(".gitignore").exists(){
        let file = std::fs::read_to_string(".gitignore").unwrap();
        let file = file.split("\n").collect::<Vec<&str>>();
        for line in file{
            if !line.is_empty(){
                ignore_files.push(line.to_string());
            }
        }
    }
    ignore_files
}

pub fn open_webpage(url: &str) {
    let _ = webbrowser::open(url);
}

pub fn print_splash() {
    bunt::println!("{$cyan}    ____       ____               __  {/$}");
    bunt::println!("{$yellow}   / __ \\___  / __/_______  _____/ /_ {/$}");
    bunt::println!("{$red}  / /_/ / _ \\/ /_/ ___/ _ \\/ ___/ __ \\{/$}");
    bunt::println!("{$yellow} / _, _/  __/ __/ /  /  __(__  ) / / /{/$}");
    bunt::println!("{$cyan}/_/ |_|\\___/_/ /_/   \\___/____/_/ /_/ {/$}");
    println!("                                      ");
}

pub fn display_output(output: &str){
    bunt::println!("Output:");
    bunt::println!("---------");
    println!("{}", output.trim());
    bunt::println!("---------");
}