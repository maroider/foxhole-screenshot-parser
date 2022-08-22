use std::{
    iter,
    path::{
        Path,
        PathBuf,
    },
};

use duct::cmd;
use pico_args::Arguments;

fn main() {
    let mut args = Arguments::from_env();
    if let Some(command) = args.subcommand().unwrap() {
        match &*command {
            "run" => {
                run(&mut args);
            }
            "dist" => {
                build(true);
            }
            "build-ueviewer" => {
                build_ueviewer();
            }
            "fetch" => {
                fetch();
            }
            _ => {
                println!("Unknown command '{command}'");
            }
        }
    } else {
        println!("No command given");
    }
}

fn run(args: &mut Arguments) {
    let release = args.contains("--release");
    build(release);
}

fn build(release: bool) {
    build_ueviewer();

    println!("Building foxhole-stockpile-screenparser");
    cmd(
        "cargo",
        iter::once("build").chain(release.then_some("--release")),
    )
    .run()
    .unwrap();

    let dist_dir = &build_dir().join(if release { "dist" } else { "debug" });
    std::fs::create_dir_all(dist_dir).unwrap();
    let exe_name = &get_exe_name("foxhole-stockpile-screenparser");
    std::fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("target")
            .join("release")
            .join(exe_name),
        dist_dir.join(exe_name),
    )
    .unwrap();
    let exe_name = &get_exe_name("umodel");
    std::fs::copy(ueviewer_build_dir().join(exe_name), dist_dir.join(exe_name)).unwrap();
}

fn build_ueviewer() {
    fetch();

    println!("Building UEViewer");
    let dir = &ueviewer_build_dir();
    cmd!(dir.join("build.sh")).dir(dir).run().unwrap();
}

fn fetch() {
    let ueviewer_ref = "c444911a6ad65bff5266f273dd5bdf7dd6fb506e";
    let dir = &ueviewer_build_dir();
    std::fs::create_dir_all(dir).unwrap();

    let output = cmd!("git", "rev-parse", "HEAD")
        .dir(dir)
        .unchecked()
        .stdout_capture()
        .stderr_capture()
        .run()
        .unwrap();
    if output.status.success() && String::from_utf8(output.stdout).unwrap().trim() == ueviewer_ref {
        return;
    }

    println!("Fetching UEViewer at {ueviewer_ref}");
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

fn ueviewer_build_dir() -> PathBuf {
    build_dir().join("UEViewer")
}

fn build_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("build")
}

fn get_exe_name(name: &str) -> String {
    let suffix = if cfg!(target_os = "windows") {
        ".exe"
    } else {
        ""
    };
    format!("{name}{suffix}")
}
