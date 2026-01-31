use clap::Parser;
use walkdir;

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

fn main() {
    let cli = Cli::parse();
    println!("Scanning directory: {}", cli.path);
    println!("Looking for pattern: {}", cli.pattern);
   
}
