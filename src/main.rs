use std::env;
use itertools::Itertools;
use std::num::NonZeroU8;
//use clap::Parser;

mod calculate_odds;
mod expected_value;
mod parse;
mod die_errors;

//#[derive(Parser, Debug)]
//#[command(version, about, long_about=None)]
//struct Args {
    /// Default or expected value (-e)
//    expected: String,

    /// Dice as <amount>x<die size>d<wanted value>
 //   input: String,
//}


fn main() {
    println!("Input dice as <amount>x<die face amount>d<wanted value>");
    //let a = Args::parse();
    let mut args: Vec<String> = env::args().collect();
    let mut odds: f64 = 0.0;
    let mut output = String::new();
    args.drain(0..1); // remove first argument that is always the file path
    let mut commands: Vec<String> = vec![];
    let mut arguments: Vec<String> = vec![];
    for i in &args{
        if i.starts_with("-") {
            commands.push(i.to_string());
        }
        else {
            arguments.push(i.to_string());
        }
    }
    let mut rolls: Vec<parse::Roll> = vec![];
    for arg in &arguments {
        rolls.push(parse::parse_dice_str(arg).unwrap());
    }
    if commands.contains(&"-e".to_string()) {
        println!("=== Expected value calculation ===");
        println!("This shows only the full amount of successful dice (e.g. no fractions)!");
        println!("Calculating expected value from {}...", arguments.iter().format(" -> "));
        output = "Expected number of successful dice:".to_string();
        odds = expected_value_function(&rolls);
    }
    else {
        for roll in rolls {
            odds = calculate_odds::calculate_roll_odds(&roll);
        }
    }
    println!("{} {}", output, odds);
}


fn expected_value_function(rolls: &Vec<parse::Roll>) -> f64 {
    let mut odds: f64 = 0.0;
    let mut dices: u8;
    let subt_die = parse::DieBuilder::default().size(rolls[0].dice.size.clone()).req_value(rolls[0].dice.req_value).build().unwrap();
    let mut subs_rolls = parse::RollBuilder::default().dice(subt_die).amount(rolls[0].amount).build().unwrap();
    for i in 0..rolls.len() {
        subs_rolls.dice.req_value = rolls[i].dice.req_value;
        subs_rolls.dice.above_below = rolls[i].dice.above_below.clone();
        odds = expected_value::calculate_expected_amount(&subs_rolls).unwrap();
        dices = odds.round_ties_even() as u8;
        if dices == 0 {
           return 0.0
        }
        subs_rolls.amount = NonZeroU8::new(dices).unwrap();
    }
   return odds
}
