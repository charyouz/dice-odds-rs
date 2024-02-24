use std::io;

mod parse;
mod calculate_odds;

fn main() {
    println!("Input dice as <amount>d<wanted value>");
    let stdin = io::stdin();
    let buffer = &mut String::new();
    stdin.read_line(buffer).expect("what?");
    let _output = parse::parse_dice_str(buffer);
}
