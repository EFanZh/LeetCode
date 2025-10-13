pub mod quick_select;

pub trait Solution {
    fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 2] as &[_], &[4, 3, 1, 5, 2] as &[_]), 2),
            ((&[5, 5, 5], &[2, 4, 2, 7]), 4),
        ];

        for ((apple, capacity), expected) in test_cases {
            assert_eq!(S::minimum_boxes(apple.to_vec(), capacity.to_vec()), expected);
        }
    }
}
