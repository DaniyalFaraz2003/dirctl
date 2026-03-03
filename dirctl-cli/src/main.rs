use clap::Parser;

/// dirctl - Deterministic file organization engine
#[derive(Parser, Debug)]
#[command(name = "dirctl")]
#[command(about = "Organize files deterministically using rules", long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("Debug mode is on");
    }

    println!("dirctl - Deterministic file organization engine");
    println!("CLI skeleton - more commands will be added in Phase 9");
}
