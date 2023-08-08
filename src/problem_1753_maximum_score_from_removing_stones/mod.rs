pub mod greedy;

pub trait Solution {
    fn maximum_score(a: i32, b: i32, c: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, 4, 6), 6), ((4, 4, 6), 7), ((1, 8, 8), 8)];

        for ((a, b, c), expected) in test_cases {
            assert_eq!(S::maximum_score(a, b, c), expected);
        }
    }
}
