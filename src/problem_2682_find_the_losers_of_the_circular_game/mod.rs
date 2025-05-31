pub mod brute_force;

pub trait Solution {
    fn circular_game_losers(n: i32, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, 2), &[4, 5] as &[_]), ((4, 4), &[2, 3, 4])];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::circular_game_losers(n, k), expected,);
        }
    }
}
