pub mod sorting;

pub trait Solution {
    fn minimum_removal(beans: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 1, 6, 5] as &[_], 4), (&[2, 10, 3, 2], 7)];

        for (beans, expected) in test_cases {
            assert_eq!(S::minimum_removal(beans.to_vec()), expected);
        }
    }
}
