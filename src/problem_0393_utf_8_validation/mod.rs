pub mod iterative;

pub trait Solution {
    fn valid_utf8(data: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[197, 130, 1] as &[_], true), (&[235, 140, 4], false)];

        for (data, expected) in test_cases.iter().copied() {
            assert_eq!(S::valid_utf8(data.to_vec()), expected);
        }
    }
}
