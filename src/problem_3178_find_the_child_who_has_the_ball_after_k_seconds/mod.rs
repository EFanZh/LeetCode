pub mod greedy;

pub trait Solution {
    fn number_of_child(n: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 5), 1), ((5, 6), 2), ((4, 2), 2)];

        for ((n, k), expected) in test_cases {
            assert_eq!(S::number_of_child(n, k), expected);
        }
    }
}
