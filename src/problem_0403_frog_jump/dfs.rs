pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashSet;

impl Solution {
    fn helper(stones: &HashSet<i32>, target: i32, stone: i32, steps: i32, cache: &mut HashSet<(i32, i32)>) -> bool {
        match stone.cmp(&target) {
            Ordering::Less => {
                if cache.insert((stone, steps)) {
                    for next_steps in ((steps - 1).max(1)..steps + 2).rev() {
                        let next_stone = stone + next_steps;

                        if stones.contains(&next_stone) && Self::helper(stones, target, next_stone, next_steps, cache) {
                            return true;
                        }
                    }
                }

                false
            }
            Ordering::Equal => true,
            Ordering::Greater => false,
        }
    }

    pub fn can_cross(stones: Vec<i32>) -> bool {
        let target = *stones.last().unwrap();

        Self::helper(&stones.into_iter().collect(), target, 0, 0, &mut HashSet::new())
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
