use std::io::{self, stdin, stdout, Write};
use std::num::ParseIntError;
use std::time::{Duration, Instant};

use clap::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
enum MFError {
    #[error("IoError: {0}")]
    Io(#[from] io::Error),
    #[error("Error converting string to number")]
    ParseIntError(#[from] ParseIntError),
}

#[derive(Debug, Parser)]
struct Args {
    /// Subcommand
    #[clap(subcommand)]
    cmd: Command,

    /// Largest number to possibly generate
    #[clap(short, long, default_value = "12")]
    max: usize,

    /// Number of questions to ask
    #[clap(short, long, default_value = "10")]
    number: usize,
}

#[derive(Debug, Default)]
struct Answers {
    correct: usize,
    total: usize,
    times: Vec<Duration>,
}

#[derive(Debug, Parser, PartialEq)]
enum Command {
    Multiply,
    Add,
    Subtract,
}

fn main() -> Result<(), MFError> {
    let args = Args::parse();

    match args.cmd {
        Command::Multiply => multiply(&args),
        Command::Add => Ok(()),
        Command::Subtract => Ok(()),
    }
}

fn get_ans() -> Result<usize, MFError> {
    let mut resp = String::new();
    stdin().read_line(&mut resp).unwrap();

    // Strip the newline off
    resp.pop();

    let a: usize = resp.parse()?;

    Ok(a)
}

fn multiply(args: &Args) -> Result<(), MFError> {
    let mut ans = Answers::default();
    let mut count = 1;
    loop {
        let a: usize = rand::random::<usize>() % args.max;
        let b: usize = rand::random::<usize>() % args.max;

        print!("{})\n{} * {} = ", count, a, b);
        stdout().flush().unwrap();

        let now = Instant::now();
        let guess = get_ans()?;
        let try_time = now.elapsed();

        if guess == a * b {
            println!("great job!");
            ans.correct += 1;
        } else {
            println!("sorry, wrong answer!");
        }
        ans.total += 1;
        ans.times.push(try_time);

        count += 1;

        if count > args.number {
            break;
        }
    }

    println!("{:?}", ans);
    let mut avg: Duration = Duration::default();
    for i in ans.times.iter() {
        avg += *i;
    }

    avg /= ans.times.len() as u32;

    println!("Average time: {:?}", avg);

    Ok(())
}
