pub mod gcd;

pub trait Solution {
    fn has_groups_size_x(deck: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4, 4, 3, 2, 1] as &[_], true),
            (&[1, 1, 1, 2, 2, 2, 3, 3], false),
            (&[1, 1, 1, 1, 2, 2, 2, 2, 2, 2], true),
        ];

        for (deck, expected) in test_cases {
            assert_eq!(S::has_groups_size_x(deck.to_vec()), expected);
        }
    }
}
