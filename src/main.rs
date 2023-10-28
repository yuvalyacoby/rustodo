use std::env;
use std::process;
use rustodo::server::app::get_app;
use rustodo::{InputParams, run};
use axum;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let cli = true;

    if cli {
        let input = InputParams::from_cli(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
        println!("input: {:?}", input);
    
        let _ = run(input).map_err(|e| eprintln!("An error occurred: {e}"));
    } else {
        let app = get_app();
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}