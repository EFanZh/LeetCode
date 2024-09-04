pub mod iterative;

pub trait Solution {
    fn number_of_ways(corridor: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("SSPPSPS", 3), ("PPSPSP", 1), ("S", 0), ("P", 0)];

        for (corridor, expected) in test_cases {
            assert_eq!(S::number_of_ways(corridor.to_string()), expected);
        }
    }
}
