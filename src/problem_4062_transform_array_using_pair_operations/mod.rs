pub mod mathematical;

pub trait Solution {
    fn can_transform(source: Vec<i32>, target: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3] as &[_], &[0, 2, 4] as &[_]), true),
            ((&[-5, -5], &[-15, 5]), true),
            ((&[1, 2, 1], &[0, 2, 5]), false),
        ];

        for ((source, target), expected) in test_cases {
            assert_eq!(S::can_transform(source.to_vec(), target.to_vec()), expected);
        }
    }
}
