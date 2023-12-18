pub mod hash_map;

pub trait Solution {
    fn find_original_array(changed: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 4, 2, 6, 8] as &[_], &[1, 3, 4] as &[_]),
            (&[6, 3, 0, 1], &[]),
            (&[1], &[]),
            (&[0, 0, 0, 0], &[0, 0]),
        ];

        for (changed, expected) in test_cases {
            assert_eq!(S::find_original_array(changed.to_vec()), expected);
        }
    }
}
