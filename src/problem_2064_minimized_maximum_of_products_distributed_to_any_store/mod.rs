pub mod binary_search;

pub trait Solution {
    fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((6, &[11, 6] as &[_]), 3),
            ((7, &[15, 10, 10]), 5),
            ((1, &[100_000]), 100_000),
        ];

        for ((n, quantities), expected) in test_cases {
            assert_eq!(S::minimized_maximum(n, quantities.to_vec()), expected);
        }
    }
}
