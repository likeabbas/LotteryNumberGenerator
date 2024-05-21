use clap::{App, Arg};
use rand::rngs::OsRng;
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let matches = App::new("Lottery Number Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates random numbers for Mega Millions or Powerball")
        .arg(Arg::with_name("lottery")
             .short('l')
             .long("lottery")
             .takes_value(true)
             .required(true)
             .help("Specifies the lottery type ('mega_millions' or 'powerball')"))
        .arg(Arg::with_name("sets")
             .short('s')
             .long("sets")
             .takes_value(true)
             .required(true)
             .help("Specifies the number of sets of numbers to generate"))
        .get_matches();

    let lottery = matches.value_of("lottery").unwrap();
    let sets: usize = matches.value_of("sets").unwrap().parse().expect("Please provide a valid number for sets");

    match lottery {
        "mega_millions" => {
            for _ in 0..sets {
                generate_mega_millions_numbers();
            }
        },
        "powerball" => {
            for _ in 0..sets {
                generate_powerball_numbers();
            }
        },
        _ => println!("Unsupported lottery type. Please choose 'mega_millions' or 'powerball'."),
    }
}


fn generate_mega_millions_numbers() {
    let mut rng = OsRng;

    let mut numbers = HashSet::new();
    while numbers.len() < 5 {
        numbers.insert(rng.gen_range(1..=70));
    }

    let mut numbers = numbers.into_iter().collect::<Vec<u32>>();
    numbers.sort();

    let mega_ball = rng.gen_range(1..=25);

    println!("Mega Millions Numbers: {:?}, Mega Ball: {}", numbers, mega_ball);
}

fn generate_powerball_numbers() {
    let mut rng = OsRng;

    let mut white_balls = (1..=69).collect::<HashSet<u32>>();
    let mut numbers = Vec::new();

    while numbers.len() < 5 {
        let number = rng.gen_range(1..=white_balls.len()) as u32;
        let &number = white_balls.iter().nth((number - 1) as usize).unwrap();
        white_balls.remove(&number);
        numbers.push(number);
    }

    numbers.sort();

    let powerball = rng.gen_range(1..=26);

    println!("Powerball Numbers: {:?}, Powerball: {}", numbers, powerball);
}
