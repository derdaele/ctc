use rand::distributions::{Distribution, Standard, Uniform};
use rand::Rng;
use std::env;
use std::fmt;
use std::io;
use std::ops::Add;
use std::process::Command;
use std::str::FromStr;

fn run_target_command() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let mut command = Command::new(&args[1]);
        if args.len() > 2 {
            command.args(&args[2..args.len()]);
        }
        command.status().expect("Cannot start target command.");
    }
}

#[derive(PartialEq, Eq)]
struct Hours(u8);

impl Distribution<Hours> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Hours {
        let hour: u8 = Uniform::from(0..24).sample(rng);
        Hours(hour)
    }
}

#[derive(PartialEq, Eq)]
struct Minutes(u8);

impl Distribution<Minutes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Minutes {
        let minutes: u8 = Uniform::from(0..61).sample(rng);
        Minutes(minutes)
    }
}

#[derive(PartialEq, Eq)]
struct Time {
    hours: Hours,
    minutes: Minutes,
}

impl Add<&Minutes> for &Time {
    type Output = Time;

    fn add(self, minutes: &Minutes) -> Self::Output {
        Time {
            hours: Hours((self.hours.0 + (self.minutes.0 + minutes.0) / 60) % 24),
            minutes: Minutes((self.minutes.0 + minutes.0) % 60),
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}h{:02}", self.hours.0, self.minutes.0)
    }
}

impl Distribution<Time> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Time {
        Time {
            hours: rng.gen(),
            minutes: rng.gen(),
        }
    }
}

enum Question {
    Time {
        base_time: Time,
        minutes_diff: Minutes,
    },
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Question::Time {
                base_time,
                minutes_diff,
            } => write!(f, "{} + {}m ?", base_time, minutes_diff.0),
        }
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let coords: Vec<&str> = s.trim().split('h').collect();

        let hours = Hours(coords[0].parse().unwrap());
        let minutes = Minutes(coords[1].parse().unwrap());

        Ok(Time { hours, minutes })
    }
}

impl Distribution<Question> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Question {
        match rng.gen_range(0, 1) {
            _ => Question::Time {
                base_time: rng.gen(),
                minutes_diff: Minutes(rng.gen_range(1, 61)),
            },
        }
    }
}

fn is_correct(input: &str, question: &Question) -> bool {
    match question {
        Question::Time {
            base_time,
            minutes_diff,
        } => {
            let time: Time = input.parse().unwrap();
            println!("{}", base_time + minutes_diff);
            time == base_time + minutes_diff
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let question: Question = rng.gen();
    println!("{}", question);

    let mut input = String::new();
    let mut correct = false;
    while !correct {
        io::stdin().read_line(&mut input).unwrap();
        if is_correct(&input, &question) {
            correct = true;
        } else {
            println!("Wrong! Try again.");
            println!("{}", question);
        }
    }
    run_target_command();
}