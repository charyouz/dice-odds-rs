use std::env;
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
        println!("Calculating expected value from {}...", arguments[0]);
        output = "Expected number of successful dice:".to_string();
        for roll in rolls {
            odds = expected_value::calculate_expected_amount(&roll).unwrap();
        }
    }
    else {
        for roll in rolls {
            odds = calculate_odds::calculate_roll_odds(&roll);
        }
    }
    println!("{} {}", output, odds);
}
