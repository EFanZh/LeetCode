pub mod greedy;

pub trait Solution {
    fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 5, 4] as &[_], 1), 1), ((&[4, 3, 1, 1, 3, 3, 2], 3), 2)];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::find_least_num_of_unique_ints(arr.to_vec(), k), expected);
        }
    }
}
