pub mod iterative;
pub mod iterative_2;
pub mod tail_recursive;

pub trait Solution {
    fn count_collisions(directions: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("RLRSLL", 5), ("LLRR", 0), ("LLSRSSRSSLLSLLLRSLSRL", 11), ("L", 0)];

        for (directions, expected) in test_cases {
            assert_eq!(S::count_collisions(directions.to_string()), expected);
        }
    }
}
