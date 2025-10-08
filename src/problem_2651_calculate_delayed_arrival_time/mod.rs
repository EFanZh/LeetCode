pub mod modular_arithmetic;

pub trait Solution {
    fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((15, 5), 20), ((13, 11), 0)];

        for ((arrival_time, delayed_time), expected) in test_cases {
            assert_eq!(S::find_delayed_arrival_time(arrival_time, delayed_time), expected);
        }
    }
}
