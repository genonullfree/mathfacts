use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Subcommand
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, Parser, PartialEq)]
enum Command {
    Multiply,
    Add,
    Subtract,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Command::Multiply => println!("MULTIPLY!"),
        Command::Add => println!("ADD!"),
        Command::Subtract => println!("SUBTRACT!"),
    }
}
