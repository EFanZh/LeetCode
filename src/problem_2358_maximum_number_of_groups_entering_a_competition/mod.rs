pub mod newtons_method;

pub trait Solution {
    fn maximum_groups(grades: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[10, 6, 12, 7, 3, 5] as &[_], 3), (&[8, 8], 1)];

        for (grades, expected) in test_cases {
            assert_eq!(S::maximum_groups(grades.to_vec()), expected);
        }
    }
}
