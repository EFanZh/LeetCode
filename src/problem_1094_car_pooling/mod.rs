pub mod binary_heap;
pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[2, 1, 5], [3, 3, 7]] as &[_], 4), false),
            ((&[[2, 1, 5], [3, 3, 7]], 5), true),
        ];

        for ((trips, capacity), expected) in test_cases {
            assert_eq!(
                S::car_pooling(trips.iter().copied().map(Vec::from).collect(), capacity),
                expected
            );
        }
    }
}
