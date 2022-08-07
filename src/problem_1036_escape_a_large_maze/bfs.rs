pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;

impl Solution {
    fn unwrap_point(point: Vec<i32>) -> (u32, u32) {
        let [x, y]: [_; 2] = point.try_into().unwrap();

        (x as _, y as _)
    }

    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        const MAZE_SIZE: u32 = 1_000_000;

        let blocked = {
            let blocked = blocked;

            blocked
                .iter()
                .map(|point| {
                    let [x, y]: [_; 2] = point.as_slice().try_into().unwrap();

                    (x as u32, y as u32)
                })
                .collect::<HashSet<_>>()
        };

        let source = Self::unwrap_point(source);
        let target = Self::unwrap_point(target);
        let max_steps = blocked.len().saturating_sub(1);
        let mut queue = VecDeque::from([source]);
        let mut visited = blocked.clone();

        visited.insert(source);

        // From source.

        for _ in 0..max_steps {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                for next in [
                    (current.0.wrapping_sub(1), current.1),
                    (current.0, current.1.wrapping_sub(1)),
                    (current.0, current.1 + 1),
                    (current.0 + 1, current.1),
                ] {
                    if next == target {
                        return true;
                    }

                    if next.0 < MAZE_SIZE && next.1 < MAZE_SIZE && visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                return false;
            }
        }

        queue.clear();
        drop(visited);

        // From target.

        let mut visited = blocked;

        queue.push_back(target);
        visited.insert(target);

        for _ in 0..max_steps {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                for next in [
                    (current.0.wrapping_sub(1), current.1),
                    (current.0, current.1.wrapping_sub(1)),
                    (current.0, current.1 + 1),
                    (current.0 + 1, current.1),
                ] {
                    if next.0 < MAZE_SIZE && next.1 < MAZE_SIZE && visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        Self::is_escape_possible(blocked, source, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
