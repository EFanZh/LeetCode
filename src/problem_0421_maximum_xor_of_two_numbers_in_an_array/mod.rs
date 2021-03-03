pub mod hash_map;
pub mod trie;

pub trait Solution {
    fn find_maximum_xor(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 10, 5, 25, 2, 8] as &[_], 28),
            (&[0], 0),
            (&[2, 4], 6),
            (&[8, 10, 2], 10),
            (&[14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70], 127),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_maximum_xor(nums.to_vec()), expected);
        }
    }
}
