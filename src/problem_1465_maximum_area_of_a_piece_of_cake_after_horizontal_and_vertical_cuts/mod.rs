pub mod greedy;

pub trait Solution {
    fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, 4, &[1, 2, 4] as &[_], &[1, 3] as &[_]), 4),
            ((5, 4, &[3, 1], &[1]), 6),
            ((5, 4, &[3], &[3]), 9),
        ];

        for ((h, w, horizontal_cuts, vertical_cuts), expected) in test_cases {
            assert_eq!(
                S::max_area(h, w, horizontal_cuts.to_vec(), vertical_cuts.to_vec()),
                expected,
            );
        }
    }
}
