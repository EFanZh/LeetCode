pub mod two_pointers;

pub trait Solution {
    fn recover_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            &[2, 10, 6, 4, 8, 12] as &[_],
            &[1, 1, 3, 3],
            &[5, 435],
            &[1, 50, 99, 101, 150, 199],
            &[3, 7, 7, 1, 5, 3],
        ];

        for nums in test_cases {
            let result = S::recover_array(nums.to_vec());

            assert_eq!(result.len() * 2, nums.len());

            let k = result[0] - nums.iter().copied().min().unwrap();

            assert!(k > 0);

            assert_eq!(
                test_utilities::unstable_sorted(result.iter().flat_map(|x| [x - k, x + k])),
                test_utilities::unstable_sorted(nums.iter().copied()),
            );
        }
    }
}
