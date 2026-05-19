pub mod direct_address_table;

pub trait Solution {
    fn recover_order(order: Vec<i32>, friends: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 1, 2, 5, 4] as &[_], &[1, 3, 4] as &[_]), &[3, 1, 4] as &[_]),
            ((&[1, 4, 5, 3, 2], &[2, 5]), &[5, 2]),
        ];

        for ((order, friends), expected) in test_cases {
            assert_eq!(S::recover_order(order.to_vec(), friends.to_vec()), expected);
        }
    }
}
