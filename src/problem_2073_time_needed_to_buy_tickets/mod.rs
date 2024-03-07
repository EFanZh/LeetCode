pub mod sliding_window;

pub trait Solution {
    fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 3, 2] as &[_], 2), 6), ((&[5, 1, 1, 1], 0), 8)];

        for ((tickets, k), expected) in test_cases {
            assert_eq!(S::time_required_to_buy(tickets.to_vec(), k), expected);
        }
    }
}
