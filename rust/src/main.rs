mod cognito;
mod config;

use clap::Parser;

#[derive(Parser)]
#[command(name = "gimme-token", version)]
struct Cli {
    /// Profile to use
    #[arg(short = 'p', long)]
    profile: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let profile = config::load_profile(cli.profile.as_deref());

    match cognito::authenticate(&profile) {
        Ok(token) => print!("Bearer {token}"),
        Err(_) => {
            eprintln!("[ERROR] Unable to authenticate with provided profile");
            std::process::exit(0);
        }
    }
}
