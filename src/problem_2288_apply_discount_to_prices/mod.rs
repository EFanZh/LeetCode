pub mod iterative_1;
pub mod iterative_2;

pub trait Solution {
    fn discount_prices(sentence: String, discount: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("there are $1 $2 and 5$ candies in the shop", 50),
                "there are $0.50 $1.00 and 5$ candies in the shop",
            ),
            (
                ("1 2 $3 4 $5 $6 7 8$ $9 $10$", 100),
                "1 2 $0.00 4 $0.00 $0.00 7 8$ $0.00 $10$",
            ),
            (
                ("706hzu76jjh7yufr5x9ot60v149k5 $7651913186 pw2o $6", 28),
                "706hzu76jjh7yufr5x9ot60v149k5 $5509377493.92 pw2o $4.32",
            ),
        ];

        for ((sentence, discount), expected) in test_cases {
            assert_eq!(S::discount_prices(sentence.to_string(), discount), expected);
        }
    }
}
