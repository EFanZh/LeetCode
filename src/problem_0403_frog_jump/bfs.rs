pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let target = *stones.last().unwrap();
        let stones = stones.into_iter().collect::<HashSet<_>>();
        let mut current = (0, 0);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        loop {
            let (stone, steps) = current;

            if stone == target {
                return true;
            }

            for next_steps in ((steps - 1).max(1)..steps + 2).rev() {
                let next_stone = stone + next_steps;

                if stones.contains(&next_stone) && visited.insert((next_stone, next_steps)) {
                    queue.push_back((next_stone, next_steps));
                }
            }

            if let Some(next) = queue.pop_front() {
                current = next;
            } else {
                break;
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
