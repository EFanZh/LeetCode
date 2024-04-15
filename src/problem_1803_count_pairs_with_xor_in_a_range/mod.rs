pub mod trie;

pub trait Solution {
    fn count_pairs(nums: Vec<i32>, low: i32, high: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 4, 2, 7] as &[_], 2, 6), 6), ((&[9, 8, 4, 2, 1], 5, 14), 8)];

        for ((nums, low, high), expected) in test_cases {
            assert_eq!(S::count_pairs(nums.to_vec(), low, high), expected);
        }
    }
}
