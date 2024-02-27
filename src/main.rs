use std::io;

mod parse;
mod calculate_odds;

fn main() {
    println!("Input dice as <amount>x<die face amount>d<wanted value>");
    let stdin = io::stdin();
    let buffer = &mut String::new();
    stdin.read_line(buffer).expect("what?");
    let roll = parse::parse_dice_str(buffer).unwrap();
    let output = calculate_odds::calculate_roll_odds(&roll);
    println!("{}", output);
}
