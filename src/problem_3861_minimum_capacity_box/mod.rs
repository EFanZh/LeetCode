pub mod iterative;

pub trait Solution {
    fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 5, 3, 7] as &[_], 3), 2),
            ((&[3, 5, 4, 3], 2), 0),
            ((&[4], 5), -1),
        ];

        for ((capacity, item_size), expected) in test_cases {
            assert_eq!(S::minimum_index(capacity.to_vec(), item_size), expected);
        }
    }
}
