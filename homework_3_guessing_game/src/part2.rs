use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        while min < max {
            let mid = (min + max) / 2;
            let result = player.ask_to_compare(mid);

            if result == 0 {
                return mid;

            } else if result == -1 {
                max = mid;
                
            } else {
                min = mid + 1;
            }
        }
        min
    }
}