pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::mem;

enum Items<T> {
    Zero,
    One(T),
    Two(T, T),
    Three(T, T, T),
}

impl<T> Iterator for Items<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match mem::replace(self, Self::Zero) {
            Self::Zero => None,
            Self::One(first) => Some(first),
            Self::Two(second, first) => {
                *self = Self::One(second);

                Some(first)
            }
            Self::Three(third, second, first) => {
                *self = Self::Two(third, second);

                Some(first)
            }
        }
    }
}

impl Solution {
    fn encode([a0, a1, a2, a3, a4]: [u8; 5]) -> u16 {
        u16::from(a0) | (u16::from(a1) << 3) | (u16::from(a2) << 6) | (u16::from(a3) << 9) | (u16::from(a4) << 12)
    }

    fn decode(value: u16) -> [u8; 5] {
        let slot_0 = (value as u8) & 0b_0111;
        let slot_1 = ((value >> 3) as u8) & 0b_0111;
        let slot_2 = ((value >> 6) as u8) & 0b_0111;
        let slot_3 = ((value >> 9) as u8) & 0b_0111;
        let slot_4 = ((value >> 12) as u8) & 0b_0111;

        [slot_0, slot_1, slot_2, slot_3, slot_4]
    }

    fn neighbors(board: [u8; 5]) -> Items<[u8; 5]> {
        match board {
            [0, a1, a2, a3, a4] => Items::Two([a3, a1, a2, 0, a4], [a1, 0, a2, a3, a4]),
            [a0, 0, a2, a3, a4] => Items::Three([a0, a4, a2, a3, 0], [a0, a2, 0, a3, a4], [0, a0, a2, a3, a4]),
            [a0, a1, 0, a3, a4] => Items::Two([a0, a1, 15 - (a0 + a1 + a3 + a4), a3, a4], [a0, 0, a1, a3, a4]),
            [a0, a1, a2, 0, a4] => Items::Two([a0, a1, a2, a4, 0], [0, a1, a2, a0, a4]),
            [a0, a1, a2, a3, 0] => Items::Three(
                [a0, a1, a2, a3, 15 - (a0 + a1 + a2 + a3)],
                [a0, a1, a2, 0, a3],
                [a0, 0, a2, a3, a1],
            ),
            [a0, a1, a2, a3, a4] => Items::Two([a0, a1, a2, a3, 0], [a0, a1, 0, a3, a4]),
        }
    }

    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let start = Self::encode([
            board[0][0] as _,
            board[0][1] as _,
            board[0][2] as _,
            board[1][0] as _,
            board[1][1] as _,
        ]);

        let mut result = 0;
        let target = Self::encode([1, 2, 3, 4, 5]);

        if start != target {
            let mut left_queue = VecDeque::from(vec![start]);
            let mut right_queue = VecDeque::from(vec![target]);
            let mut states = HashMap::new();

            states.insert(start, false);
            states.insert(target, true);

            loop {
                result += 1;

                if left_queue.len() < right_queue.len() {
                    for _ in 0..left_queue.len() {
                        for next in Self::neighbors(Self::decode(left_queue.pop_front().unwrap())).map(Self::encode) {
                            match states.entry(next) {
                                Entry::Occupied(entry) => {
                                    if *entry.into_mut() {
                                        return result;
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    entry.insert(false);
                                    left_queue.push_back(next);
                                }
                            }
                        }
                    }

                    if left_queue.is_empty() {
                        return -1;
                    }
                } else {
                    for _ in 0..right_queue.len() {
                        for next in Self::neighbors(Self::decode(right_queue.pop_front().unwrap())).map(Self::encode) {
                            match states.entry(next) {
                                Entry::Occupied(entry) => {
                                    if !*entry.into_mut() {
                                        return result;
                                    }
                                }
                                Entry::Vacant(entry) => {
                                    entry.insert(true);
                                    right_queue.push_back(next);
                                }
                            }
                        }
                    }

                    if right_queue.is_empty() {
                        return -1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        Self::sliding_puzzle(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
