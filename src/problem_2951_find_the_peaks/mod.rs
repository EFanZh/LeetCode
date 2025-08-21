pub mod iterative;

pub trait Solution {
    fn find_peaks(mountain: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 4, 4] as &[_], &[] as &[_]), (&[1, 4, 3, 8, 5], &[1, 3])];

        for (mountain, expected) in test_cases {
            assert_eq!(S::find_peaks(mountain.to_vec()), expected);
        }
    }
}
