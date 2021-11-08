pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn racecar(target: i32) -> i32 {
        let mut steps = 1;
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from(vec![(target, -1)]);

        loop {
            for _ in 0..queue.len() {
                let (position, speed) = queue.pop_front().unwrap();

                for &(next_position, next_speed) in &[
                    (position + speed, speed * 2),
                    (position, if speed > 0 { -1 } else { 1 }),
                ] {
                    let (next_position, next_speed) = match next_position.cmp(&0) {
                        Ordering::Less => (-next_position, -next_speed),
                        Ordering::Equal => return steps,
                        Ordering::Greater => (next_position, next_speed),
                    };

                    if visited.insert((next_position, next_speed)) {
                        queue.push_back((next_position, next_speed));
                    }
                }
            }

            steps += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn racecar(target: i32) -> i32 {
        Self::racecar(target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
