pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Key {
    data: [u8; 26],
}

impl Key {
    fn from_str(s: &str) -> Self {
        let mut data = [0; 26];

        for c in s.bytes() {
            data[usize::from(c - b'a')] += 1;
        }

        Self { data }
    }

    fn is_empty(&self) -> bool {
        self.data.iter().all(|x| *x == 0)
    }

    fn saturating_sub(&self, other: &Self) -> Self {
        let mut data = [0; 26];

        for (target, (lhs, rhs)) in data.iter_mut().zip(self.data.iter().zip(&other.data)) {
            *target = lhs.saturating_sub(*rhs);
        }

        Self { data }
    }
}

impl Solution {
    fn helper(stickers: &[Key], target: Key, cache: &mut HashMap<Key, Option<i32>>) -> Option<i32> {
        if target.is_empty() {
            Some(0)
        } else if let Some(&result) = cache.get(&target) {
            result
        } else {
            let result = stickers
                .iter()
                .filter_map(|sticker| {
                    let next = target.saturating_sub(sticker);

                    if next == target {
                        None
                    } else {
                        Self::helper(stickers, next, cache)
                    }
                })
                .min()
                .map(|x| x + 1);

            cache.insert(target, result);

            result
        }
    }

    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let stickers = stickers.into_iter().map(|s| Key::from_str(&s)).collect::<Vec<_>>();
        let target = Key::from_str(&target);
        let mut cache = HashMap::new();

        Self::helper(&stickers, target, &mut cache).unwrap_or(-1)
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
