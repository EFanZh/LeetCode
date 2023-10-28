pub mod merge_sort;

pub trait Solution {
    fn create_sorted_array(instructions: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 5, 6, 2] as &[_], 1),
            (&[1, 2, 3, 6, 5, 4], 3),
            (&[1, 3, 3, 3, 2, 4, 2, 1, 2], 4),
        ];

        for (instructions, expected) in test_cases {
            assert_eq!(S::create_sorted_array(instructions.to_vec()), expected);
        }
    }
}
