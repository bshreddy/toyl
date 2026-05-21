use std::fs;

mod cli;
mod parser;

fn main() {
    let args = cli::parse_args();

    let Ok(content) = fs::read_to_string(&args.input_files[0]) else {
        return;
    };

    println!("{}", content);
}
