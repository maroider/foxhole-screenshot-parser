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
    println!("fetch");
}

fn build_ueviewer() {
    println!("build");
}

fn dist() {
    println!("dist");
}
