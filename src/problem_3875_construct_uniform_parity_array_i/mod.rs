pub mod mathematical;

pub trait Solution {
    fn uniform_array(nums1: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3] as &[_], true), (&[4, 6], true)];

        for (nums1, expected) in test_cases {
            assert_eq!(S::uniform_array(nums1.to_vec()), expected);
        }
    }
}
