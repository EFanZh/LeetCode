pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 4, 6, 7, 8, 20] as &[_], [2, 7, 15]), 11),
            ((&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], [2, 7, 15]), 17),
            ((&[1, 4, 6, 7, 8, 20], [7, 2, 15]), 6),
            ((&[1, 4, 6, 7, 8, 365], [2, 7, 15]), 11),
        ];

        for ((days, costs), expected) in test_cases {
            assert_eq!(S::mincost_tickets(days.to_vec(), costs.to_vec()), expected);
        }
    }
}
