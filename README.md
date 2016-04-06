# Bowling Kata (in Rust)

## Rules of bowling

- Game consists of 10 frames.
  * in each frame player has 2 opportunities (rolls) to knock down 10 pins
  * score for a frame = total number of pins knocked down
  * and bonuses for spares and strikes
- Spare is when player knocks down all 10 pins in 2 tries
  * bonus = number of pins knocked down by the next roll
- Strike is when player knocks down all 10 pins in 1 try
  * bonus = number of pins knocked down in the next 2 rolls
- 10th frame
  * player who rolls spare or strike is allowed to do extra rolls to complete frame
  * however, no more than 3 rolls can be done in 10th frame

## Requirements

- Implement a trait named `Game` that has following methods:
  * `roll(pins : i32)` called each time player does a roll. `pins` argument is
  the number of pins knocked down by that roll
  * `score() -> i32` is called only at the end of the game. It returns the total
  score of the game
