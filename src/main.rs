use std::path::Path;

use clap::Parser;
mod config;
mod linter;

/// Mon super programme CLI
#[derive(Parser, Debug)]
#[command(name = "dirlint", version = env!("CARGO_PKG_VERSION"), about = "Un exemple avec clap")]
struct Args {
    directory: String,
    #[arg(short, long, default_value = ".dirlint.yaml")]
    config: String,
    #[arg(long, help="Strictly validate directory (no additional files/folders allowed)")]
    strict: bool,
}

fn main() {
    let cli = Args::parse();

    println!("processing: {:?}", cli.directory);
    let config = config::load_config(&cli.config).unwrap();
    let path = Path::new(&cli.directory);
    let _ = linter::lint_directory(path, &config);
}