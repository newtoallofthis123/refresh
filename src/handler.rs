pub fn handle_build(toolchain: &str){
    match toolchain {
        "Rust" => handle_cargo(),
        "Go" => (),
        _ => ()
    }
}

fn handle_cargo(){
    bunt::println!("Building...");
    let _ = std::process::Command::new("cargo")
        .arg("build")
        .output()
        .expect("Failed to build");
    bunt::println!("{$green}Build Successful{/$}");
}