pub mod recursive;

pub trait Solution {
    fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[17, 13, 11, 2, 3, 5, 7] as &[_], &[2, 13, 3, 11, 5, 17, 7] as &[_]),
            (&[1, 1000], &[1, 1000]),
            (
                &[477, 845, 584, 259, 304, 909, 758, 512, 784, 421, 405],
                &[259, 784, 304, 584, 405, 909, 421, 758, 477, 845, 512],
            ),
            (&[1, 2, 3, 4, 5], &[1, 5, 2, 4, 3]),
            (&[1], &[1]),
        ];

        for (deck, expected) in test_cases {
            assert_eq!(S::deck_revealed_increasing(deck.to_vec()), expected);
        }
    }
}
