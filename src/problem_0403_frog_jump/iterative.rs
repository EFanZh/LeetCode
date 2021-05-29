pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let target = *stones.last().unwrap();

        let mut nexts = stones
            .iter()
            .map(|&stone| (stone, HashSet::new()))
            .collect::<HashMap<_, _>>();

        nexts.get_mut(&0).unwrap().insert(1);

        for stone in stones {
            for steps in nexts.remove(&stone).unwrap() {
                let next_stone = stone + steps;

                if next_stone == target {
                    return true;
                }

                if let Some(next_nexts) = nexts.get_mut(&(stone + steps)) {
                    next_nexts.extend((steps - 1).max(1)..steps + 2);
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_cross(stones: Vec<i32>) -> bool {
        Self::can_cross(stones)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
