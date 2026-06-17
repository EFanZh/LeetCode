pub mod iterative;

pub trait Solution {
    fn mirror_distance(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(25, 27), (10, 9), (7, 0)];

        for (n, expected) in test_cases {
            assert_eq!(S::mirror_distance(n), expected);
        }
    }
}
