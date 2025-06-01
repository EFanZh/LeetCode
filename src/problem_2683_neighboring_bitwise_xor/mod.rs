pub mod mathematical;

pub trait Solution {
    fn does_valid_array_exist(derived: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 1, 0] as &[_], true), (&[1, 1], true), (&[1, 0], false)];

        for (derived, expected) in test_cases {
            assert_eq!(S::does_valid_array_exist(derived.to_vec()), expected);
        }
    }
}
