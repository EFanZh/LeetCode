pub mod dynamic_programming;

pub trait Solution {
    fn largest_number(cost: Vec<i32>, target: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (([4, 3, 2, 5, 6, 7, 2, 5, 5], 9), "7772"),
            (([7, 6, 5, 5, 5, 6, 8, 7, 8], 12), "85"),
            (([2, 4, 6, 2, 4, 6, 4, 4, 4], 5), "0"),
        ];

        for ((cost, target), expected) in test_cases {
            assert_eq!(S::largest_number(cost.to_vec(), target), expected);
        }
    }
}
