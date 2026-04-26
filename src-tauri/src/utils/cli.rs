use std::env;

#[derive(Debug, Default)]
pub struct CliArgs {
    pub start_minimized: bool,
    pub user_agent: Option<String>,
    pub discord_url: Option<String>,
    pub show_version: bool,
    pub show_help: bool,
}

pub fn parse_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();
    let mut cli = CliArgs::default();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--start-minimized" => cli.start_minimized = true,
            "--user-agent" => {
                if i + 1 < args.len() {
                    cli.user_agent = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--version" | "-v" => cli.show_version = true,
            "--help" | "-h" => cli.show_help = true,
            arg if arg.starts_with("discord://") || arg.starts_with("spotify://") => {
                cli.discord_url = Some(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    cli
}

pub fn print_help() {
    println!("Veskto - The lean, Gecko-powered Discord client");
    println!();
    println!("Usage: veskto [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --start-minimized    Launch minimized to tray");
    println!("  --user-agent <UA>    Override user agent string");
    println!("  --version, -v        Print version");
    println!("  --help, -h           Print this help message");
    println!();
    println!("Protocol URLs:");
    println!("  discord://...        Open Discord invite/custom protocol URL");
}

pub fn print_version() {
    println!("Veskto {}", env!("CARGO_PKG_VERSION"));
}
