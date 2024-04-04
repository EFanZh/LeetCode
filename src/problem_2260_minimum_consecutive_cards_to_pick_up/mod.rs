pub mod iterative;

pub trait Solution {
    fn minimum_card_pickup(cards: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[3, 4, 2, 3, 4, 7] as &[_], 4), (&[1, 0, 5, 3], -1)];

        for (cards, expected) in test_cases {
            assert_eq!(S::minimum_card_pickup(cards.to_vec()), expected);
        }
    }
}
