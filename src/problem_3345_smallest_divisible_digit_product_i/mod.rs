pub mod brute_force;

pub trait Solution {
    fn smallest_number(n: i32, t: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((10, 2), 10), ((15, 3), 16)];

        for ((n, t), expected) in test_cases {
            assert_eq!(S::smallest_number(n, t), expected);
        }
    }
}
