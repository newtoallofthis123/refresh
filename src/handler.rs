pub fn handle_build(toolchain: &str, run: bool) {
    match toolchain {
        "Rust" => handle_cargo(run),
        "Go" => (),
        _ => (),
    }
}

fn handle_cargo(run: bool) {
    if run {
        bunt::println!("Running...");
        let output = std::process::Command::new("cargo")
            .arg("run")
            .output()
            .expect("Failed to run cargo run");
        let std_output = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);
        let status = output.status.to_string();
        if !err.is_empty() && err.contains("error") {
            crate::utils::display_output(err.to_string().as_str());
            bunt::println!("Status: {}", status);
            bunt::println!("{$red}Build Failed with errors{/$}");
            bunt::println!("Exiting...");
            std::process::exit(0);
        }
        else{
            crate::utils::display_output(std_output.to_string().as_str());
        }
    } else {
        bunt::println!("Building...");
        let _ = std::process::Command::new("cargo")
            .arg("build")
            .output()
            .expect("Failed to build");
        bunt::println!("{$green}Build Successful{/$}");
    }
}
