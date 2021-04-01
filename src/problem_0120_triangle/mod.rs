pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn minimum_total(triangle: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[&[2] as &[_], &[3, 4], &[6, 5, 7], &[4, 1, 8, 3]] as &[&[_]], 11)];

        for (triangle, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::minimum_total(triangle.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
