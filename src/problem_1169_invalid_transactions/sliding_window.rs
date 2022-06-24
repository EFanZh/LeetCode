pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, VecDeque};

impl Solution {
    fn parse_transaction(transaction: &str) -> (&str, u16, u16, &str) {
        let mut iter = transaction.split(',');
        let name = iter.next().unwrap();
        let time = iter.next().unwrap().parse().unwrap();
        let amount = iter.next().unwrap().parse().unwrap();
        let city = iter.next().unwrap();

        (name, time, amount, city)
    }

    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut accounts = HashMap::new();

        for transaction in transactions.iter().map(String::as_str) {
            let (name, time, amount, city) = Self::parse_transaction(transaction);

            accounts
                .entry(name)
                .and_modify(|transactions| Vec::push(transactions, (time, amount, city, transaction)))
                .or_insert_with(|| vec![(time, amount, city, transaction)]);
        }

        let mut result = Vec::new();
        let mut window = VecDeque::<(u16, &str, Option<&str>)>::new();

        for transactions in accounts.values_mut().map(Vec::as_mut_slice) {
            transactions.sort_unstable_by_key(|&(time, _, _, _)| time);

            for &(time, amount, city, transaction) in &*transactions {
                while let Some(&(front_time, _, _)) = window.front() {
                    if front_time + 60 < time {
                        window.pop_front();
                    } else {
                        break;
                    }
                }

                let mut different_city = false;

                for (_, prev_city, prev_transaction) in &mut window {
                    if *prev_city != city {
                        if let Some(prev_transaction) = prev_transaction.take() {
                            result.push(prev_transaction.to_string());
                        }

                        different_city = true;
                    }
                }

                window.push_back((
                    time,
                    city,
                    if amount > 1000 || different_city {
                        result.push(transaction.to_string());

                        None
                    } else {
                        Some(transaction)
                    },
                ));
            }

            window.clear();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        Self::invalid_transactions(transactions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
