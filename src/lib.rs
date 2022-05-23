use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::io;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn evaluate(&self, other: &Choice) -> Option<bool> {
        match self {
            Choice::Rock => match other {
                Choice::Rock => None,
                Choice::Paper => Some(false),
                Choice::Scissors => Some(true),
            },
            Choice::Paper => match other {
                Choice::Rock => Some(true),
                Choice::Paper => None,
                Choice::Scissors => Some(false),
            },
            Choice::Scissors => match other {
                Choice::Rock => Some(false),
                Choice::Paper => Some(true),
                Choice::Scissors => None,
            },
        }
    }
}

impl Distribution<Choice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
        match rng.gen_range(0..=2) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => Choice::Rock,
        }
    }
}

pub fn run() {
    loop {
        print!("\x1B[2J\x1B[1;1H");

        let mut user_score = 0;
        let mut computers_score = 0;

        while user_score < 3 && computers_score < 3 {
            println!("Your Score: {}", user_score);
            println!("Computer's Score: {}", computers_score);
            println!("");

            println!("[1] Rock");
            println!("[2] Paper");
            println!("[3] Scissors");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");

            let choice = choice.trim().parse().expect("Please type a number!");

            if choice < 1 || choice > 3 {
                println!("Enter either 1 (Rock), 2 (Paper), 3 (Scissors)");
                continue;
            }

            let user_choice = match choice {
                1 => Choice::Rock,
                2 => Choice::Paper,
                3 => Choice::Scissors,
                _ => Choice::Rock,
            };

            let computer_choice: Choice = rand::random();

            print!("\x1B[2J\x1B[1;1H");
            println!("You Chose: {:?}", user_choice);
            println!("Computer Chose: {:?}", computer_choice);
            println!("");

            if let Some(won) = user_choice.evaluate(&computer_choice) {
                if won {
                    user_score += 1;
                } else {
                    computers_score += 1;
                }
            }
        }

        print!("\x1B[2J\x1B[1;1H");
        println!("You scored: {}", user_score);
        println!("Computer scored: {}", computers_score);

        if user_score > computers_score {
            println!("🎉 Congratulations you won!");
        } else {
            println!("😭 Oh oh you lost!");
        }

        println!("");

        loop {
            println!("Wanna play again? (y/n)");
            let mut res = String::new();
            io::stdin()
                .read_line(&mut res)
                .expect("Failed to read input");

            let res = res.trim();

            if "yYnN".contains(res) {
                if res == "y" || res == "Y" {
                    break;
                } else if res == "n" || res == "N" {
                    return;
                }
            }

            println!("Please enter either y or n");
        }
    }
}
