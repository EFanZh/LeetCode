pub mod dynamic_programming;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn remove_boxes(boxes: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 2, 2, 2, 3, 4, 3, 1] as &[_], 23), (&[1, 1, 1], 9), (&[1], 1)];

        for (boxes, expected) in test_cases {
            assert_eq!(S::remove_boxes(boxes.to_vec()), expected);
        }
    }
}
