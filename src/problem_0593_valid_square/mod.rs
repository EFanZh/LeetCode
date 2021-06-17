pub mod mathematical;

pub trait Solution {
    fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (([0, 0], [1, 1], [1, 0], [0, 1]), true),
            (([0, 0], [1, 1], [1, 0], [0, 12]), false),
            (([1, 0], [-1, 0], [0, 1], [0, -1]), true),
            (([0, 0], [0, 0], [0, 0], [0, 0]), false),
        ];

        for ((p1, p2, p3, p4), expected) in test_cases {
            assert_eq!(
                S::valid_square(p1.to_vec(), p2.to_vec(), p3.to_vec(), p4.to_vec()),
                expected
            );
        }
    }
}
