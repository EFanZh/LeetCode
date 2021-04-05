pub struct Solution;

use std::collections::VecDeque;
use std::iter;

impl Solution {
    fn is_neighbor(lhs: &str, rhs: &str) -> bool {
        let mut iter_1 = lhs.bytes();
        let mut iter_2 = rhs.bytes();

        while let (Some(c_1), Some(c_2)) = (iter_1.next(), iter_2.next()) {
            if c_1 != c_2 {
                return iter_1.eq(iter_2);
            }
        }

        false
    }

    fn drain_filter<'a, T>(items: &'a mut Vec<T>, mut f: impl 'a + FnMut(&T) -> bool) -> impl 'a + Iterator<Item = T> {
        let mut i = 0;

        iter::from_fn(move || {
            while let Some(item) = items.get(i) {
                if f(item) {
                    return Some(items.swap_remove(i));
                }

                i += 1;
            }

            None
        })
    }

    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
        let mut queue = VecDeque::from(vec![begin_word]);
        let mut length = 1;

        loop {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                if current == end_word {
                    return length;
                }

                queue.extend(Self::drain_filter(&mut word_list, |item| {
                    Self::is_neighbor(&current, item)
                }));
            }

            if queue.is_empty() {
                break;
            }

            length += 1;
        }

        0
    }
}

impl super::Solution for Solution {
    fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        Self::ladder_length(begin_word, end_word, word_list)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
