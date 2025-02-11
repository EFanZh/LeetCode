pub mod iterative;

pub trait Solution {
    fn find_array(pref: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[5, 2, 0, 3, 1] as &[_], &[5, 7, 2, 3, 2] as &[_]), (&[13], &[13])];

        for (pref, expected) in test_cases {
            assert_eq!(S::find_array(pref.to_vec()), expected);
        }
    }
}
