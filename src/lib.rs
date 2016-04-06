trait BowlingGame {
    fn roll(&mut self, pins: i32);
    fn score(&self) -> i32;
}

struct Game {
    rolls: Vec<i32>
}

fn new_game() -> Game {
    Game{rolls: vec![]}
}

impl BowlingGame for Game {
    fn roll(&mut self, pins: i32) {
        self.rolls.push(pins)
    }

    fn score(&self) -> i32 {
        let mut total = 0;

        let mut i = 0;
        for _ in 0..10 {
            let first_roll = self.rolls[i];
            i += 1;

            if first_roll == 10 {

                if i + 1 < self.rolls.len() {
                    total += first_roll + self.rolls[i] + self.rolls[i + 1];
                }

            } else {

                let second_roll = self.rolls[i];
                total += first_roll + second_roll;
                i += 1;

                if first_roll + second_roll == 10 {
                    if i < self.rolls.len() {
                        total += self.rolls[i];
                    }
                }

            }
        }

        total
    }
}

#[cfg(test)]
mod test {
    use super::Game;
    use super::new_game;
    use super::BowlingGame;

    fn game_with(rolls: &[i32]) -> Game {
        let mut game = new_game();

        for pins in rolls.iter() {
            game.roll(*pins);
        }

        game
    }

    fn game_with_all(pins: i32) -> Game {
        let mut rolls = vec![];

        for _ in 0..20 {
            rolls.push(pins)
        }

        game_with(rolls.as_slice())
    }

    #[test]
    fn gutter_game() {
        assert!(0 == game_with_all(0).score());
    }

    #[test]
    fn game_of_ones() {
        assert!(20 == game_with_all(1).score());
    }

    #[test]
    fn game_with_one_spare() {
        assert!(21 == game_with(&[0, 0, 0, 0, 8, 2, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).score())
    }

    #[test]
    fn game_with_spare_looking_sequence_but_no_spare() {
        assert!(18 == game_with(&[0, 0, 0, 0, 0, 8, 2, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).score())
    }

    #[test]
    fn game_with_one_strike() {
        assert!(26 == game_with(&[0, 0, 0, 0, 10, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).score())
    }

    #[test]
    fn game_with_spare_that_looks_like_strike() {
        assert!(21 == game_with(&[0, 0, 0, 0, 0, 10, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).score())
    }

    #[test]
    fn game_with_one_strike_after_another() {
        assert!(10 + 13 + 10 + 8 + 3 + 5 == game_with(&[0, 0, 0, 0, 10, 10, 3, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).score())
    }

    #[test]
    fn perfect_game() {
        assert!(300 == game_with(&[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]).score())
    }

    #[test]
    fn almost_perfect_game() {
        assert!(240 + 24 + 19 == game_with(&[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 4, 5]).score())
    }

    #[test]
    fn almost_perfect_game_with_spare_at_the_end() {
        assert!(210 + 24 + 20 + 13 == game_with(&[10, 10, 10, 10, 10, 10, 10, 10, 10, 4, 6, 3]).score())
    }
}
