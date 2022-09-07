use std::io::{self, stdin, stdout, Write};
use std::num::ParseIntError;
use std::time::{Duration, Instant};
use std::mem;

use chrono::Local;
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
    max: isize,

    /// Number of questions to ask
    #[clap(short, long, default_value = "10")]
    number: isize,

    /// Allow random to generate negative numbers
    #[clap(long)]
    negative: bool,
}

#[derive(Debug, Default)]
struct Answers {
    correct: isize,
    total: isize,
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

    execute_questions(&args)
}

fn get_ans(question: String) -> Result<isize, MFError> {
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

fn execute_questions(args: &Args) -> Result<(), MFError> {
    let mut ans = Answers::default();
    let mut count = 1;
    loop {
        let mut a: isize = rand::random::<isize>() % (args.max + 1);
        let mut b: isize = rand::random::<isize>() % (args.max + 1);
        if !args.negative {
            a = isize::abs(a);
            b = isize::abs(b);
        }

        let op = match args.cmd {
            Command::Multiply => "x",
            Command::Add => "+",
            Command::Subtract => {
                // Always subtract the smaller from the larger value
                if a < b {
                    mem::swap(&mut a, &mut b);
                }
                "-"
            },
        };

        let question = format!("{})\n{} {} {} = ", count, a, op, b);

        let now = Instant::now();
        let guess = get_ans(question)?;
        let try_time = now.elapsed();

        let calc = match args.cmd {
            Command::Multiply => a * b,
            Command::Add => a + b,
            Command::Subtract => a - b,
        };

        if guess == calc {
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

    print_score(&ans)
}

fn print_score(ans: &Answers) -> Result<(), MFError> {
    println!(" ===== {} =====", Local::now());
    println!(
        "Score: {:.2}%",
        (ans.correct as f32 / ans.total as f32) * 100f32
    );

    println!("Correct: {}", ans.correct);
    println!("Total questions: {}", ans.total);

    let mut total: Duration = Duration::default();
    for i in ans.times.iter() {
        total += *i;
    }

    let avg = total / ans.times.len() as u32;

    let min = total.as_secs() / 60;
    let sec = total.as_secs() % 60;

    println!("Average time per question: {:.3?}", avg);
    println!("Total time: {}m {}s", min, sec);

    Ok(())
}
