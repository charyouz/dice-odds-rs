# Dice Odds

Project to calculate what the odds are to getting certain die results from rolls.

## Description

Calculates odds for dice. Slowly building this software, but idea is to input the dice that you
want to roll, and the program would output the odds of succeeding, or the expected amount of dice that
will succeed.

## Usage

Currently, the main usage is for the expected values (e.g., amount of dice * odds for success of single die).
All rolls should be in the form of <amount of dice>x<die faces>d<wanted number><+/- if you want bigger or smaller>
To call this run the program with he "-e" option, followed by the rolls you want.

For example "2xd64+" means you are rolling 2 dice that have 6 sides (e.g., D6) and you want 4+ (e.g., 4, 5 or 6).
Another example: "3x6d5-" means 3 dice that have 6 sides and you want less than 5 (e.g. 4, 3, 2 or 1). Note that the minus is exclusive,
while the plus is inclusive.

You can also chain the rolls:

"4x6d4+ d4+" means that you are first rolling 4 dice and want 4 or more, and out of those that succeed you want also 4 or more (so re-rolling the succesful rolls).
Note that chaining the rolls doesn't allow to change the die sizes or amounts, you simply roll again the successes. Also, it doesn't
accurately count the precise odds when doing this. This is due to the program rounding to the closest full number after every roll (since rolling
fractions of dice is not feasible). This should even out, but of course will cause some issues, especially with long rolling chains.

### Re-Rolling

To re-roll dice, put an underscore after the roll and put "R" to re-roll succesfull dice and "r" to re-roll failing dice.
So a command to roll 8 6-sided dice for 4+ and re-rolling failing would be:
```
8x6d4+_r
```

if you want to continue the logic, rolling the dice again for 4+ and re-rolling successes would be:
```
8x6d4+_r d4+_R
```

## Getting Started

### Building the program

To build from source, download the program and build using "cargo build"

## Roadmap

- [x] Add more die sizes (D3, D12 for a start).
- [x] Add options for re-rolling failing dice.
