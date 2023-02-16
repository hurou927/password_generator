use clap::Parser;

use crate::alphabet::Alphabets;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'l', default_value_t = 12)]
    pub length: usize,

    #[arg(short = 'a', default_value_t = 1)]
    pub lower_min_length: usize,

    #[arg(short = 'A', default_value_t = 1)]
    pub upper_min_length: usize,

    #[arg(short = 's', default_value_t = 1)]
    pub simbol_min_length: usize,

    #[arg(short = 'n', default_value_t = 1)]
    pub number_min_length: usize,

    #[arg(
        short = 'S',
        long,
        default_value = r###"~`!@#$%^&*()_-+={[}]|\:;"'<,>.?/"###
    )]
    pub simbol_chars: String,

    #[arg(short = 'N', default_value_t = 1)]
    pub num_passwords: usize,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "1: week password(half-width & number), otherwise: custom"
    )]
    pub preset: u16,
}

fn validate(cli: &Cli) -> Result<(), String> {
    if cli.preset > 1 {
        return Err("invalid preset".to_string());
    }
    let sum_of_min_length =
        cli.lower_min_length + cli.lower_min_length + cli.simbol_min_length + cli.number_min_length;

    if sum_of_min_length == 0 {
        Err("the sum of min_length-s == 0".to_string())
    } else if cli.length < sum_of_min_length {
        Err("exceed max_length".to_string())
    } else {
        Ok(())
    }
}

pub fn to_alphabets(cli: &Cli) -> Result<Alphabets, String> {
    validate(cli)?;

    let simbol_chars = cli.simbol_chars.as_bytes().to_vec();

    let alphabets = if cli.preset == 1 {
        Alphabets::new(cli.length, 1, 0, 1, 0, Vec::new())
    } else {
        Alphabets::new(
            cli.length,
            cli.lower_min_length,
            cli.upper_min_length,
            cli.number_min_length,
            cli.simbol_min_length,
            simbol_chars,
        )
    };
    Ok(alphabets)
}
