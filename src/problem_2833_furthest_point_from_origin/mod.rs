pub mod greedy;

pub trait Solution {
    fn furthest_distance_from_origin(moves: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("L_RL__R", 3), ("_R__LL_", 5), ("_______", 7)];

        for (moves, expected) in test_cases {
            assert_eq!(S::furthest_distance_from_origin(moves.to_string()), expected);
        }
    }
}
