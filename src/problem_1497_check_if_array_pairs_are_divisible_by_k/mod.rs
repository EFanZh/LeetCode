pub mod buckets;

pub trait Solution {
    fn can_arrange(arr: Vec<i32>, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 10, 6, 7, 8, 9] as &[_], 5), true),
            ((&[1, 2, 3, 4, 5, 6], 7), true),
            ((&[1, 2, 3, 4, 5, 6], 10), false),
        ];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::can_arrange(arr.to_vec(), k), expected);
        }
    }
}
