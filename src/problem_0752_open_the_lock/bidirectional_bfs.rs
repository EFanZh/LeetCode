pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

enum State {
    NotVisited,
    DeadEnd,
    LeftVisited,
    RightVisited,
}

const NOT_VISITED: State = State::NotVisited;

impl Solution {
    fn get_nexts(current: usize) -> [usize; 8] {
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
            let mut states = [NOT_VISITED; 10000];

            for deadend in deadends {
                states[deadend.parse::<usize>().unwrap()] = State::DeadEnd;
            }

            if matches!(mem::replace(&mut states[0], State::LeftVisited), State::NotVisited) {
                let target = target.parse::<usize>().unwrap();
                let mut result = 1;
                let mut left_queue = VecDeque::from(vec![0]);
                let mut right_queue = VecDeque::from(vec![target]);

                states[target] = State::RightVisited;

                loop {
                    if right_queue.len() < left_queue.len() {
                        for _ in 0..right_queue.len() {
                            let current = right_queue.pop_front().unwrap();

                            for &next in &Self::get_nexts(current) {
                                match &mut states[next] {
                                    state @ State::NotVisited => {
                                        *state = State::RightVisited;
                                        right_queue.push_back(next);
                                    }
                                    State::DeadEnd | State::RightVisited => {}
                                    State::LeftVisited => return result,
                                }
                            }
                        }

                        if right_queue.is_empty() {
                            break;
                        }
                    } else {
                        for _ in 0..left_queue.len() {
                            let current = left_queue.pop_front().unwrap();

                            for &next in &Self::get_nexts(current) {
                                match &mut states[next] {
                                    state @ State::NotVisited => {
                                        *state = State::LeftVisited;
                                        left_queue.push_back(next);
                                    }
                                    State::DeadEnd | State::LeftVisited => {}
                                    State::RightVisited => return result,
                                }
                            }
                        }

                        if left_queue.is_empty() {
                            break;
                        }
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
