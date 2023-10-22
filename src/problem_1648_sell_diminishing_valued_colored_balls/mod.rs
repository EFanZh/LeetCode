pub mod greedy_binary_heap;

pub trait Solution {
    fn max_profit(inventory: Vec<i32>, orders: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 5] as &[_], 4), 14),
            ((&[3, 5], 6), 19),
            ((&[2, 8, 4, 10, 6], 20), 110),
        ];

        for ((inventory, orders), expected) in test_cases {
            assert_eq!(S::max_profit(inventory.to_vec(), orders), expected);
        }
    }
}
