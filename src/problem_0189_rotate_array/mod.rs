pub mod cheating;
pub mod cyclic_replacements;
pub mod triple_reverses;

pub trait Solution {
    fn rotate(nums: &mut Vec<i32>, k: i32);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6, 7] as &[_], 3), &[5, 6, 7, 1, 2, 3, 4] as &[_]),
            ((&[-1, -100, 3, 99], 2), &[3, 99, -1, -100]),
            ((&[-1], 2), &[-1]),
        ];

        for ((nums, k), expected) in test_cases {
            let mut nums = nums.to_vec();

            S::rotate(&mut nums, k);

            assert_eq!(nums, expected);
        }
    }
}
