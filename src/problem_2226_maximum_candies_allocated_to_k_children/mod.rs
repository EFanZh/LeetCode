pub mod binary_search;

pub trait Solution {
    fn maximum_candies(candies: Vec<i32>, k: i64) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 8, 6] as &[_], 3), 5), ((&[2, 5], 11), 0)];

        for ((tickets, k), expected) in test_cases {
            assert_eq!(S::maximum_candies(tickets.to_vec(), k), expected);
        }
    }
}
