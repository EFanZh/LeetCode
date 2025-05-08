pub mod greedy;

pub trait Solution {
    fn dist_money(money: i32, children: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((20, 3), 1),
            ((16, 2), 2),
            ((5, 2), 0),
            ((13, 3), 1),
            ((17, 2), 1),
            ((23, 2), 1),
        ];

        for ((money, children), expected) in test_cases {
            assert_eq!(S::dist_money(money, children), expected);
        }
    }
}
