pub mod iterative;

pub trait Solution {
    fn valid_utf8(data: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[197, 130, 1] as &[_], true),
            (&[235, 140, 4], false),
            (&[206, 210, 189, 208, 197, 163, 182, 171, 212, 243, 10, 0, 10], false),
            (&[230, 136, 145], true),
            (&[240, 162, 138, 147], true),
            (&[240, 255], false),
            (&[255], false),
        ];

        for (data, expected) in test_cases.iter().copied() {
            assert_eq!(S::valid_utf8(data.to_vec()), expected);
        }
    }
}
