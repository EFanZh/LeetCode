pub mod mathematical;

pub trait Solution {
    fn uniform_array(nums1: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 4, 7] as &[_], true),
            (&[2, 3], false),
            (&[4, 6], true),
            (&[22, 2, 13], false),
            (&[13, 6, 5], true),
        ];

        for (nums1, expected) in test_cases {
            assert_eq!(S::uniform_array(nums1.to_vec()), expected);
        }
    }
}
