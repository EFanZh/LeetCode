pub mod mathematical;

pub trait Solution {
    fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 2, 3), 3), ((2, 5, 10), 0), ((1, 10, 3), 0), ((1, 1000, 999), 1)];

        for ((start_pos, end_pos, k), expected) in test_cases {
            assert_eq!(S::number_of_ways(start_pos, end_pos, k), expected);
        }
    }
}
