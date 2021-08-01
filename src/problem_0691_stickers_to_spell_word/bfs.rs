pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};
use std::iter;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Key {
    data: u128,
}

impl Key {
    fn from_str(s: &str) -> Self {
        let mut data = 0;

        for c in s.bytes() {
            data += 1 << ((c - b'a') * 4);
        }

        Self { data }
    }

    fn is_empty(&self) -> bool {
        self.data == 0
    }

    fn saturating_sub(&self, other: &Self) -> Self {
        let mut data = [0; 16];

        for (target, (lhs, rhs)) in data
            .iter_mut()
            .zip(self.data.to_ne_bytes().iter().zip(&other.data.to_ne_bytes()))
        {
            *target = (lhs & 0x0f).saturating_sub(rhs & 0x0f) | (lhs & 0xf0).saturating_sub(rhs & 0xf0);
        }

        Self {
            data: u128::from_ne_bytes(data),
        }
    }
}

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let stickers = stickers.into_iter().map(|s| Key::from_str(&s)).collect::<Vec<_>>();
        let target = Key::from_str(&target);
        let mut result = 1;
        let mut visited = iter::once(target.clone()).collect::<HashSet<_>>();
        let mut queue = VecDeque::from(vec![target]);

        loop {
            for _ in 0..queue.len() {
                let key = queue.pop_front().unwrap();

                for sticker in &stickers {
                    let next = key.saturating_sub(sticker);

                    if next.is_empty() {
                        return result;
                    }

                    if visited.insert(next.clone()) {
                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            result += 1;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        Self::min_stickers(stickers, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
