use std::cmp::Ordering;
use std::error::Error;
use std::io;
use rand::Rng;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Value must be between 1 and 100.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// if main returns a result, the Ok type should be the unit type
fn main() -> Result<(), Box<dyn Error>> {
    println!("Please enter a guess");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let correct_guess: Guess = loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let parsed: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Input must be a number.");
                continue;
            },
        };

        let validated = Guess::new(parsed);

        match validated.value().cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                break validated
            }
        };
    };

    println!("You win! Your guess: {:#?}", correct_guess);
    Ok(())
}
