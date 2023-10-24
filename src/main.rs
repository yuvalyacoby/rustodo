use std::env;
use std::process;
use rustodo::{InputParams, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = InputParams::from_cli(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("input: {:?}", input);

    let _ = run(input).map_err(|e| eprintln!("An error occurred: {e}"));
}