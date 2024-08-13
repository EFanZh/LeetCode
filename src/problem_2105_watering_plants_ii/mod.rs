pub mod iterative;

pub trait Solution {
    fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 2, 3, 3] as &[_], 5, 5), 1),
            ((&[2, 2, 3, 3], 3, 4), 2),
            ((&[5], 10, 8), 0),
            ((&[1, 2, 4, 4, 5], 6, 5), 2),
        ];

        for ((plants, capacity_a, capacity_b), expected) in test_cases {
            assert_eq!(S::minimum_refill(plants.to_vec(), capacity_a, capacity_b), expected);
        }
    }
}
