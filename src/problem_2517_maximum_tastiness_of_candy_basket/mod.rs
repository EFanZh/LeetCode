pub mod binary_search;

pub trait Solution {
    fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[13, 5, 1, 8, 21, 2] as &[_], 3), 8),
            ((&[1, 3, 1], 2), 2),
            ((&[7, 7, 7, 7], 2), 0),
        ];

        for ((price, k), expected) in test_cases {
            assert_eq!(S::maximum_tastiness(price.to_vec(), k), expected);
        }
    }
}
