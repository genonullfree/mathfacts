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

fn get_ans(question: String) -> Result<usize, MFError> {
    loop {
        print!("{}", question);
        stdout().flush().unwrap();

        let mut resp = String::new();
        stdin().read_line(&mut resp).unwrap();

        // Strip the newline off
        resp.pop();

        match resp.parse() {
            Ok(a) => return Ok(a),
            Err(_) => println!("Couldn't understand \"{}\", please try again.", resp),
        }
    }
}

fn multiply(args: &Args) -> Result<(), MFError> {
    let mut ans = Answers::default();
    let mut count = 1;
    loop {
        let a: usize = rand::random::<usize>() % args.max;
        let b: usize = rand::random::<usize>() % args.max;

        let question = format!("{})\n{} * {} = ", count, a, b);

        let now = Instant::now();
        let guess = get_ans(question)?;
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

    print_score(&ans);

    Ok(())
}

fn print_score(ans: &Answers) -> Result<(), MFError> {
    println!(" =====");
    println!(
        "Score: {}%",
        (ans.correct as f32 / ans.total as f32) * 100f32
    );

    println!("Correct: {}", ans.correct);
    println!("Total questions: {}", ans.total);

    let mut avg: Duration = Duration::default();
    for i in ans.times.iter() {
        avg += *i;
    }

    avg /= ans.times.len() as u32;

    println!("Average time per question: {:.3?}", avg);

    Ok(())
}
