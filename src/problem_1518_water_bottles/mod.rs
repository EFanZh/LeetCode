pub mod mathematical;

pub trait Solution {
    fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((9, 3), 13), ((15, 4), 19)];

        for ((num_bottles, num_exchange), expected) in test_cases {
            assert_eq!(S::num_water_bottles(num_bottles, num_exchange), expected);
        }
    }
}
