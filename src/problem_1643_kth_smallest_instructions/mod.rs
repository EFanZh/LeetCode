pub mod combinations;

pub trait Solution {
    fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3] as &[_], 1), "HHHVV"),
            ((&[2, 3], 2), "HHVHV"),
            ((&[2, 3], 3), "HHVVH"),
        ];

        for ((destination, k), expected) in test_cases {
            assert_eq!(S::kth_smallest_path(destination.to_vec(), k), expected);
        }
    }
}
