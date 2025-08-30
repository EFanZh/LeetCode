pub mod greedy;

pub trait Solution {
    fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 4, 8, 7, 9, 3, 5, 1] as &[_], 2), true),
            ((&[2, 4, 2, 2, 5, 2], 2), false),
            ((&[4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11], 14), true),
        ];

        for ((nums, k), expected) in test_cases {
            let result = S::divide_array(nums.to_vec(), k);

            if expected {
                assert_eq!(
                    test_utilities::unstable_sorted(nums.iter().copied()),
                    test_utilities::unstable_sorted(result.iter().flatten().copied()),
                );

                for item in result {
                    let mut item: [_; 3] = item.try_into().ok().unwrap();

                    item.sort_unstable();

                    assert!(item[2] - item[0] <= k);
                }
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
