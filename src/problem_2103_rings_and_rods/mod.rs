pub mod iterative;

pub trait Solution {
    fn count_points(rings: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("B0B6G0R6R0R6G9", 1), ("B0R0G0R9R0B0G0", 1), ("G4", 0)];

        for (rings, expected) in test_cases {
            assert_eq!(S::count_points(rings.to_string()), expected);
        }
    }
}
