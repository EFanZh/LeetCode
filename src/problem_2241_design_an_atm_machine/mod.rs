pub mod greedy;
pub mod greedy_2;

#[expect(clippy::upper_case_acronyms, reason = "required")]
pub trait ATM {
    fn new() -> Self;
    fn deposit(&mut self, banknotes_count: Vec<i32>);
    fn withdraw(&mut self, amount: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::ATM;

    enum Operation {
        Deposit([i32; 5]),
        Withdraw(i32, Option<[i32; 5]>),
    }

    pub fn run<A: ATM>() {
        let test_cases = [&[
            Operation::Deposit([0, 0, 1, 2, 1]),
            Operation::Withdraw(600, Some([0, 0, 1, 0, 1])),
            Operation::Deposit([0, 1, 0, 1, 1]),
            Operation::Withdraw(600, None),
            Operation::Withdraw(550, Some([0, 1, 0, 0, 1])),
        ] as &[_]];

        for operations in test_cases {
            let mut atm = A::new();

            for operation in operations {
                match *operation {
                    Operation::Deposit(banknotes_count) => atm.deposit(banknotes_count.to_vec()),
                    Operation::Withdraw(amount, expected) => assert_eq!(
                        atm.withdraw(amount),
                        expected.as_ref().map_or(&[-1] as &[_], |expected| expected),
                    ),
                }
            }
        }
    }
}
