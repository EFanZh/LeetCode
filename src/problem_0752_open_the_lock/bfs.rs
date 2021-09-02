pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    fn get_nexts(current: u16) -> [u16; 8] {
        let mut result = [0; 8];
        let mut base = 1;

        for i in 0..4 {
            let (next_1, next_2) = match (current / base) % 10 {
                0 => (current + base, current + base * 9),
                9 => (current - base * 9, current - base),
                _ => (current - base, current + base),
            };

            result[i * 2] = next_1;
            result[i * 2 + 1] = next_2;
            base *= 10;
        }

        result
    }

    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        if target == "0000" {
            0
        } else {
            let mut visited = [false; 10000];

            for deadend in deadends {
                visited[deadend.parse::<usize>().unwrap()] = true;
            }

            if !mem::replace(&mut visited[0], true) {
                let target = target.parse::<u16>().unwrap();
                let mut result = 1;
                let mut queue = VecDeque::from(vec![0]);

                loop {
                    for _ in 0..queue.len() {
                        let current = queue.pop_front().unwrap();

                        for &next in &Self::get_nexts(current) {
                            if next == target {
                                return result;
                            }

                            if !mem::replace(&mut visited[usize::from(next)], true) {
                                queue.push_back(next);
                            }
                        }
                    }

                    if queue.is_empty() {
                        break;
                    }

                    result += 1;
                }
            }

            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        Self::open_lock(deadends, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
