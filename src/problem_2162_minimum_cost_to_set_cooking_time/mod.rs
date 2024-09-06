pub mod mathematical;

pub trait Solution {
    fn min_cost_set_time(start_at: i32, move_cost: i32, push_cost: i32, target_seconds: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1, 2, 1, 600), 6),
            ((0, 1, 2, 76), 6),
            ((7, 220, 479, 6000), 2576),
            ((7, 85276, 26772, 107), 336_144),
            ((0, 1, 4, 9), 5),
        ];

        for ((start_at, move_cost, push_cost, target_seconds), expected) in test_cases {
            assert_eq!(
                S::min_cost_set_time(start_at, move_cost, push_cost, target_seconds),
                expected,
            );
        }
    }
}
