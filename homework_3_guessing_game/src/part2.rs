use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // base case: only one number left
        if min >= max {
            return min;
        }

        let mid = (min + max) / 2;
        let result = player.ask_to_compare(mid);

        if result == 0 {
            return mid;
        }
        else if result == -1 {
            // number is smaller than mid
            return Part2::guess_the_number(player, min, mid);
        }
        else {
            // number is greater than mid
            return Part2::guess_the_number(player, mid + 1, max);
        }
    }
}
