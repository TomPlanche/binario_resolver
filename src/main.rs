#[path = "binario.rs"]
mod binario;

use std::time::Instant;

use binario::Binairo;
use clap::Parser;

/// CLI argument parser struct
#[derive(Parser, Clone)]
#[command(about = "CLI for solving Binairo puzzles")]
#[command(author = "Tom P. <tomplanche@icloud.com>")]
#[command(help_template = "{about}\nMade by: {author}\n\nUSAGE:\n{usage}\n\n{all-args}\n")]
struct Cli {
    ///Path to the image
    #[arg(required = true)]
    image_path: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    if args.verbose {
        println!("Image path: {}", args.image_path);
    }

    let mut binairo = Binairo::from_image(&args.image_path).unwrap();

    println!("Initial Grid:");
    binairo.print_grid();

    let now = Instant::now();
    let soved = binairo.solve();
    let elapsed = now.elapsed();

    if soved {
        binairo.print_grid();
        println!("Solved in {}ms", elapsed.as_millis());
    } else {
        println!("No solution found");
    }
}
