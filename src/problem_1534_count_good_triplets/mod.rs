pub mod brute_force;
pub mod brute_force_2;

pub trait Solution {
    fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 0, 1, 1, 9, 7] as &[_], 7, 2, 3), 4),
            ((&[1, 1, 2, 2, 3], 0, 0, 1), 0),
        ];

        for ((arr, a, b, c), expected) in test_cases {
            assert_eq!(S::count_good_triplets(arr.to_vec(), a, b, c), expected);
        }
    }
}
