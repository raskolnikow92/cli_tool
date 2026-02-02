use crate::scan::scan_walkdir;

mod cli;
mod scan;

fn main() {
    let arguments = cli::get_cli_commands();   
    let entries = scan_walkdir(arguments.0);
}
