pub mod hash_map;

pub trait Solution {
    fn find_shortest_sub_array(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 2, 3, 1] as &[_], 2), (&[1, 2, 2, 3, 1, 4, 2], 6)];

        for (nums, expected) in test_cases {
            assert_eq!(S::find_shortest_sub_array(nums.to_vec()), expected);
        }
    }
}
