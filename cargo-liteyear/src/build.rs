use std::process::{Command, Output};

fn fail(cmd: &mut Command) -> bool {
    println!("running command: {cmd:?}");

    match cmd.output() {
        Ok(Output { status, stdout, .. }) if status.success() => {
            let version = String::from_utf8(stdout).unwrap();
            println!("got {version}");

            false
        }

        err => {
            eprintln!("failed to run command");
            match err {
                Ok(output) => eprintln!("{output:#?}"),
                Err(err) => eprintln!("{err:?}"),
            };

            true
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/build.rs");

    if fail(Command::new("rustc").arg("+nightly").arg("--version")) {
        println!("cargo:warning=Nightly not installed.");
        eprintln!("Run `rustup component add rustfmt` to install it.");
    }

    if fail(Command::new("rustfmt").arg("--version")) {
        println!("cargo:warning=Failed to find `rustfmt` in PATH.");
        eprintln!("Run `rustup component add rustfmt` to install it.");
    }

    if fail(Command::new("cargo-expand").arg("--version")) {
        println!("cargo:warning=Failed to find `cargo-expand` in PATH.");
        eprintln!("Run `cargo install cargo-expand` to install it.");
    }
}
