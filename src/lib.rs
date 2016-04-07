trait BowlingGame {
    fn roll(&mut self, pins: i32);
    fn score(&self) -> i32;
}

const ALL_BOWLING_PINS: i32 = 10;
const BOWLING_FRAME_COUNT: usize = 10;

struct PartialScore {
    frame: Box<Frame>,
    total: i32
}

fn empty_partial_score(game: &Game) -> PartialScore {
    PartialScore{
        frame: game.create_frame(0),
        total: 0
    }
}

impl PartialScore {
    fn next_score(&self, game: &Game) -> PartialScore {
        PartialScore{
            frame: self.frame.next_frame(game),
            total: self.total + self.frame.scoreWithBonus(game)
        }
    }
}

trait Frame {
    fn score(&self, game: &Game) -> i32;
    fn bonus(&self, game: &Game) -> i32;
    fn next_frame_index(&self) -> usize;

    fn scoreWithBonus(&self, game: &Game) -> i32 {
        self.score(game) + self.bonus(game)
    }

    fn next_frame(&self, game: &Game) -> Box<Frame> {
        game.create_frame(self.next_frame_index())
    }
}

struct NormalFrame {
    frame_index: usize
}

struct StrikeFrame {
    frame_index: usize
}

struct SpareFrame {
    frame_index: usize
}

impl Frame for StrikeFrame {
    fn score(&self, game: &Game) -> i32 { ALL_BOWLING_PINS }
    fn bonus(&self, game: &Game) -> i32 { game.strike_bonus(self.frame_index) }
    fn next_frame_index(&self) -> usize { self.frame_index + 1 }
}

impl Frame for SpareFrame {
    fn score(&self, game: &Game) -> i32 { ALL_BOWLING_PINS }
    fn bonus(&self, game: &Game) -> i32 { game.spare_bonus(self.frame_index) }
    fn next_frame_index(&self) -> usize { self.frame_index + 2 }
}

impl Frame for NormalFrame {
    fn score(&self, game: &Game) -> i32 { game.two_rolls(self.frame_index) }
    fn bonus(&self, game: &Game) -> i32 { 0 }
    fn next_frame_index(&self) -> usize { self.frame_index + 2 }
}

struct Game {
    rolls: Vec<i32>
}

impl Game {
    fn two_rolls(&self, frame_index: usize) -> i32 {
        self.first_roll(frame_index) +
            self.second_roll(frame_index)
    }

    fn strike_bonus(&self, strike_index: usize) -> i32 {
        self.two_rolls(strike_index + 1)
    }

    fn spare_bonus(&self, spare_index: usize) -> i32 {
        self.roll_at(spare_index + 2)
    }

    fn first_roll(&self, index: usize) -> i32 {
        self.roll_at(index)
    }

    fn second_roll(&self, index: usize) -> i32 {
        self.roll_at(index + 1)
    }

    fn roll_at(&self, index: usize) -> i32 {
        *(self.rolls.get(index).unwrap_or(&-1))
    }

    fn is_strike(&self, frame_index: usize) -> bool {
        self.first_roll(frame_index) == ALL_BOWLING_PINS
    }

    fn is_spare(&self, frame_index: usize) -> bool {
        self.two_rolls(frame_index) == ALL_BOWLING_PINS
    }

    fn create_frame(&self, index: usize) -> Box<Frame> {
        if self.is_strike(index) {
            Box::new(StrikeFrame{frame_index: index})
        } else if self.is_spare(index) {
            Box::new(SpareFrame{frame_index: index})
        } else {
            Box::new(NormalFrame{frame_index: index})
        }
    }
}

fn new_game() -> Game {
    Game{rolls: vec![]}
}

impl BowlingGame for Game {
    fn roll(&mut self, pins: i32) {
        self.rolls.push(pins)
    }

    fn score(&self) -> i32 {
        (0..BOWLING_FRAME_COUNT).fold(empty_partial_score(self), |partial_score, _| {
            partial_score.next_score(self)
        }).total
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
