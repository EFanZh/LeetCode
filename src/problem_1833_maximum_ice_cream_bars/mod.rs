pub mod counting_sort;

pub trait Solution {
    fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 2, 4, 1] as &[_], 7), 4),
            ((&[10, 6, 8, 7, 7, 8], 5), 0),
            ((&[1, 6, 3, 1, 2, 5], 20), 6),
        ];

        for ((costs, coins), expected) in test_cases {
            assert_eq!(S::max_ice_cream(costs.to_vec(), coins), expected);
        }
    }
}
