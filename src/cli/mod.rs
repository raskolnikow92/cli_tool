


use clap::Parser;


#[derive(Parser)]
#[command(name = "rust_cli_tool")]
#[command(about = "High-Performance File Analyzer in Rust", long_about = None)]
struct Cli {
    /// Directory to scan
    #[arg(short, long)]
    path: String,

    /// Pattern to search
    #[arg(short = 't', long)]
    pattern: String,
}

pub fn get_cli_commands () -> (String, String) {
    let cli = Cli::parse();
    println!("Scanning directory: {}", cli.path);
    println!("Looking for pattern: {}", cli.pattern);
    (cli.path, cli.pattern)

}