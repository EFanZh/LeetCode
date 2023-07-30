pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn min_operations(boxes: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("110", &[1, 1, 3] as &[_]), ("001011", &[11, 8, 5, 4, 3, 4])];

        for (boxes, expected) in test_cases {
            assert_eq!(S::min_operations(boxes.to_string()), expected);
        }
    }
}
