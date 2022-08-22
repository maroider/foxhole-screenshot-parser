use std::path::{
    Path,
    PathBuf,
};

use duct::cmd;
use pico_args::Arguments;

fn main() {
    let mut args = Arguments::from_env();
    if let Some(command) = args.subcommand().unwrap() {
        match &*command {
            "fetch" => {
                fetch();
            }
            "build-ueviewer" => {
                build_ueviewer();
            }
            "dist" => {
                dist();
            }
            _ => {
                println!("Unknown command '{command}'");
            }
        }
    } else {
        println!("No command given");
    }
}

fn fetch() {
    let ueviewer_ref = "c444911a6ad65bff5266f273dd5bdf7dd6fb506e";
    println!("Fetching UEViewer at {ueviewer_ref}");
    let dir = &ueviewer_build_dir();
    std::fs::create_dir_all(dir).unwrap();
    cmd!("git", "init").dir(dir).run().unwrap();
    let remotes = cmd!("git", "remote").dir(dir).read().unwrap();
    if remotes.is_empty() {
        cmd!(
            "git",
            "remote",
            "add",
            "origin",
            "https://github.com/gildor2/UEViewer",
        )
        .dir(dir)
        .run()
        .unwrap();
    }
    cmd!("git", "fetch", "origin", ueviewer_ref, "--depth", "1")
        .dir(dir)
        .run()
        .unwrap();
    cmd!("git", "config", "advice.detachedHead", "false")
        .dir(dir)
        .run()
        .unwrap();
    cmd!("git", "checkout", ueviewer_ref)
        .dir(dir)
        .run()
        .unwrap();
}

fn build_ueviewer() {
    println!("build");
}

fn dist() {
    println!("dist");
}

fn ueviewer_build_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("build")
        .join("UEViewer")
}
