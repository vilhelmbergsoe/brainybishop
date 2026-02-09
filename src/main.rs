use brainybishop::board::Color;
use brainybishop::error::Result;
use brainybishop::uci::{run_interactive_mode, UciEngine};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "uci" => {
                let mut engine = UciEngine::new();
                engine.run()?;
            }
            "white" | "w" => {
                run_interactive_mode(Color::White)?;
            }
            "black" | "b" => {
                run_interactive_mode(Color::Black)?;
            }
            "--help" | "-h" => {
                print_help();
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                print_help();
                std::process::exit(1);
            }
        }
    } else {
        run_interactive_mode(Color::White)?;
    }

    Ok(())
}

fn print_help() {
    println!("brainybishop - {}", VERSION);
    println!();
    println!("Usage:");
    println!("  brainybishop [white|black]  - Play as white or black (default: white)");
    println!("  brainybishop uci            - UCI protocol mode");
}
