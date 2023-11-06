use std::env;
use std::process;
use rustodo::server::app::get_app;
use rustodo::{InputParams, run};
use axum;
use std::net::SocketAddr;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().env().init().unwrap();
    let args: Vec<String> = env::args().collect();

    let cli = match args.get(1) {
        Some(x) if x == "server" => false,
        _ => true
    };

    if cli {
        let input = InputParams::from_cli(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
        println!("input: {:?}", input);
    
        let _ = run(input).map_err(|e| eprintln!("An error occurred: {e}"));
    } else {
        let app = get_app();
        let port = 3000;
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        println!("Running on port {}", port);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}