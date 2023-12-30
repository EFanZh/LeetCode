pub mod naive;

pub trait Bank {
    fn new(balance: Vec<i64>) -> Self;
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool;
    fn deposit(&mut self, account: i32, money: i64) -> bool;
    fn withdraw(&mut self, account: i32, money: i64) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Bank;

    enum Operation {
        Transfer((i32, i32, i64), bool),
        Deposit((i32, i64), bool),
        Withdraw((i32, i64), bool),
    }

    pub fn run<B: Bank>() {
        let test_cases = [
            (
                &[10_i64, 100, 20, 50, 30] as &[_],
                &[
                    Operation::Withdraw((3, 10), true),
                    Operation::Transfer((5, 1, 20), true),
                    Operation::Deposit((5, 20), true),
                    Operation::Transfer((3, 4, 15), false),
                    Operation::Withdraw((10, 50), false),
                ] as &[_],
            ),
            (
                &[22, 34, 12, 59, 0, 58, 61, 83],
                &[
                    Operation::Withdraw((24, 16), false),
                    Operation::Deposit((7, 40), true),
                    Operation::Deposit((1, 10), true),
                    Operation::Deposit((6, 90), true),
                    Operation::Transfer((8, 7, 65), true),
                    Operation::Transfer((4, 55, 80), false),
                    Operation::Withdraw((70, 19), false),
                ],
            ),
            (
                &[92, 62, 12, 81, 77, 38, 71, 8, 42, 38],
                &[
                    Operation::Transfer((3, 2, 18), false),
                    Operation::Transfer((29, 3, 99), false),
                    Operation::Deposit((8, 97), true),
                ],
            ),
            (
                &[
                    767, 653, 252, 849, 480, 187, 761, 243, 408, 385, 334, 732, 289, 886, 149, 320, 827, 111, 315, 155,
                    695, 110, 473, 585, 83, 936, 188, 818, 33, 984, 66, 549, 954, 761, 662, 212, 208, 215, 251, 792,
                    956, 261, 863, 374, 411, 639, 599, 418, 909, 208, 984, 602, 741, 302, 911, 616, 537, 422, 61, 746,
                    206, 396, 446, 661, 48, 156, 725, 662, 422, 624, 704, 143, 94, 702, 126, 76, 539, 83, 270, 717,
                    736, 393, 607, 895, 661,
                ],
                &[
                    Operation::Deposit((68, 668), true),
                    Operation::Deposit((25, 978), true),
                    Operation::Transfer((8, 31, 924), false),
                    Operation::Transfer((2, 6, 857), false),
                    Operation::Transfer((20, 43, 59), true),
                    Operation::Deposit((71, 307), true),
                    Operation::Transfer((11, 46, 577), false),
                    Operation::Withdraw((37, 377), false),
                    Operation::Deposit((72, 835), true),
                    Operation::Withdraw((82, 574), false),
                    Operation::Transfer((67, 9, 939), false),
                    Operation::Transfer((24, 49, 251), true),
                ],
            ),
        ];

        for (balance, operations) in test_cases {
            let mut bank = B::new(balance.to_vec());

            for operation in operations {
                match *operation {
                    Operation::Transfer((account1, account2, money), expected) => {
                        assert_eq!(bank.transfer(account1, account2, money), expected);
                    }
                    Operation::Deposit((account, money), expected) => {
                        assert_eq!(bank.deposit(account, money), expected);
                    }
                    Operation::Withdraw((account, money), expected) => {
                        assert_eq!(bank.withdraw(account, money), expected);
                    }
                }
            }
        }
    }
}
