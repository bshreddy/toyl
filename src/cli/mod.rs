use std::env;

#[derive(Default)]
pub struct CLIArgs {
    pub input_files: Vec<String>,
}

pub fn parse_args() -> CLIArgs {
    let mut args = CLIArgs::default();

    for arg in env::args().skip(1) {
        args.input_files.push(arg);
    }

    return args;
}
