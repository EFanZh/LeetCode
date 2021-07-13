pub mod backtracking;

pub trait Solution {
    fn judge_point24(cards: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ([4, 1, 8, 7], true),
            ([1, 2, 1, 2], false),
            ([1, 1, 7, 7], false),
            ([1, 9, 1, 2], true),
        ];

        for (cards, expected) in test_cases {
            assert_eq!(S::judge_point24(cards.to_vec()), expected);
        }
    }
}
