mod alphabet;
mod option;
mod output;

use option::to_alphabets;
use std::process::exit;

use alphabet::Alphabets;
use option::Cli;

use clap::Parser;

fn main() {
    let mut rng = rand::thread_rng();
    let cli = Cli::parse();

    let alphabets: Alphabets = match to_alphabets(&cli) {
        Ok(a) => a,
        Err(reason) => {
            println!("Invalid option: err: ${reason}");
            exit(1);
        }
    };

    let passwords = (0..(cli.num_passwords))
        .map(|_| alphabets.gen_password(&mut rng))
        .collect::<Vec<String>>();
    match output::Output::new(passwords).to_json() {
        Ok(ps) => println!("{ps}"),
        Err(reason) => {
            println!("Invalid json: err: ${reason}");
            exit(1);
        }
    };

    exit(0);
}
