pub mod dynamic_programming;

pub trait Solution {
    fn change(amount: i32, coins: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, &[1, 2, 5] as &[_]), 4), ((3, &[2]), 0), ((10, &[10]), 1)];

        for ((amount, coins), expected) in test_cases {
            assert_eq!(S::change(amount, coins.to_vec()), expected);
        }
    }
}
