pub mod mathematical;

pub trait Solution {
    fn kth_character(k: i32) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(5, 'b'), (10, 'c')];

        for (nums, expected) in test_cases {
            assert_eq!(S::kth_character(nums), expected);
        }
    }
}
