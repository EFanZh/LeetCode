pub mod brute_force;

pub trait Solution {
    fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (([13, 2, 3, 1, 9], "aaaaa"), "Flush"),
            (([4, 4, 2, 4, 4], "daabc"), "Three of a Kind"),
            (([10, 10, 2, 12, 9], "abcad"), "Pair"),
            (([13, 12, 3, 4, 7], "adcbc"), "High Card"),
        ];

        for ((ranks, suits), expected) in test_cases {
            assert_eq!(S::best_hand(ranks.to_vec(), suits.chars().collect()), expected);
        }
    }
}
