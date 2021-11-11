pub mod reduce_to_line_overlap;

pub trait Solution {
    fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (([0, 0, 2, 2], [1, 1, 3, 3]), true),
            (([0, 0, 1, 1], [1, 0, 2, 1]), false),
            (([0, 0, 1, 1], [2, 2, 3, 3]), false),
            (([2, 17, 6, 20], [3, 8, 6, 20]), true),
        ];

        for ((rec1, rec2), expected) in test_cases {
            assert_eq!(S::is_rectangle_overlap(rec1.to_vec(), rec2.to_vec()), expected);
        }
    }
}
