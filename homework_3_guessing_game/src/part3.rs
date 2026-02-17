use crate::player::PlayerTrait;

pub struct SimulatedPlayer {
    the_number: u32,
}

impl SimulatedPlayer {
    pub fn new(number: u32) -> SimulatedPlayer {
        SimulatedPlayer {
            the_number: number
        }
    }
}

impl PlayerTrait for SimulatedPlayer {
    /**
     * Return true if guess == the_number
     */
    fn ask_if_equal(&mut self, guess: u32) -> bool {
        self.the_number == guess
    }

    /**
     * Return:
     * 0 if equal
     * -1 if the_number < guess
     * 1 if the_number > guess
     */
    fn ask_to_compare(&mut self, guess: u32) -> i32 {
        if self.the_number == guess {
            0
        } else if self.the_number < guess {
            -1
        } else {
            1
        }
    }
}


#[cfg(test)]
mod part1_tests {
    use crate::part1::Part1;
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::Strategy;

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert_eq!(player.steps(), 1);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= max);
    }

    #[test]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = 50;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= max);
    }
}


#[cfg(test)]
mod bad_strategy_tests {
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::{BadStrategy, Strategy};

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
    }

    #[test]
    #[should_panic]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = 50;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);

        // This should panic because BadStrategy is wrong
        assert_eq!(answer, number);
    }
}


#[cfg(test)]
mod part2_tests {
    use crate::part2::Part2;
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::Strategy;

    #[test]
    fn the_min() {
        let min = 0;
        let max = 1024;
        let number = min;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part2::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);

        // Binary search should be fast
        assert!(player.steps() <= 20);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 1024;
        let number = max - 1;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part2::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= 20);
    }

    #[test]
    fn a_different_number() {
        let min = 0;
        let max = 1024;
        let number = 777;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part2::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= 20);
    }
}
