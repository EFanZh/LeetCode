pub mod iterative;

pub trait Solution {
    fn traffic_signal(n: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(60, "Red"), (5, "Invalid")];

        for (n, expected) in test_cases {
            assert_eq!(S::traffic_signal(n), expected);
        }
    }
}
