# Auto Refresh your build

A Simple and easy to use auto builder tool for your projects.

## What is refrsh?

refrsh is a very _simple_ cli written in rust that does one thing and one thing only: **Auto Build your project when any file changes**.

Atleast by default it does that. I am planning to add more features to it in the future.
For now, you can use refrsh on your:

- Rust projects
- Go projects

Other than that, you can free to contribute and add support for your own language or toolchain.

## How to use refrsh?

For now, using refrsh is very simple. Just install it using cargo:

```bash
cargo install refrsh
```

And then run it in your project directory:

```bash
refrsh <dir>
```

Where `<dir>` is the directory you want to watch for changes. If you don't specify a directory, it will watch the current directory.

## How does refrsh work?

refrsh uses the [notify](https://crates.io/crates/notify) crate to watch for changes in the specified directory.

refrsh automatically picks up on which tool chain and language you are using and runs the appropriate command to build your project.

For example, if you are using rust, refrsh will run `cargo build` on your project. If you are using go, refrsh will run `go build` on your project.

However, if just run `refrsh` in a directory, if refrsh doesn't find an appropriate command to run, it will just exit.

## How to contribute?

If you want to add support for your own language or toolchain, you can do so by modifying the `src/handler.rs` file.

For example, the rust handler looks like this:

```rust
fn handle_cargo(){
 bunt::println!("Building...");
 let _ = std::process::Command::new("cargo")
  .arg("build")
  .output()
  .expect("Failed to build");
 bunt::println!("{$green}Build Successful{/$}");
}
```

You can add your own handler in the same way. Just add a new function and call it in the `handle` function.
If you want to add support for a new language, you can do so by adding a new function in the same way.

Well, there is more to it and I will add more documentation in the future. For now, you can just look at the source code and figure it out.

## License

refrsh is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
