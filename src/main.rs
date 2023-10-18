use std::env;
use std::process;
use rustodocli::{InputParams, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = InputParams::from_cli(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("input: {:?}", input);

    run(input);
}