pub mod greedy;

pub trait Solution {
    fn maximum_number(num: String, change: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("132", [9, 8, 5, 0, 3, 6, 4, 2, 6, 8]), "832"),
            (("021", [9, 4, 3, 5, 7, 2, 1, 9, 0, 6]), "934"),
            (("5", [1, 4, 7, 5, 3, 2, 5, 6, 9, 4]), "5"),
            (("435678", [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), "435678"),
        ];

        for ((num, change), expected) in test_cases {
            assert_eq!(S::maximum_number(num.to_string(), change.to_vec()), expected);
        }
    }
}
