pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn max_distance(moves: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("L_D_", 4), ("U_R", 3)];

        for (moves, expected) in test_cases {
            assert_eq!(S::max_distance(moves.to_string()), expected);
        }
    }
}
