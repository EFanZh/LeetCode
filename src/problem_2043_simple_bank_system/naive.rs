// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;

pub struct Bank {
    balance: Box<[i64]>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self {
            balance: balance.into_boxed_slice(),
        }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let balance = Cell::from_mut(self.balance.as_mut()).as_slice_of_cells();

        if let Some(from) = balance.get((account1 as u32 as usize).wrapping_sub(1)) {
            if let Some(to) = balance.get((account2 as u32 as usize).wrapping_sub(1)) {
                let from_money = from.get();

                if money <= from_money {
                    from.set(from_money - money);
                    to.set(to.get() + money);

                    return true;
                }
            }
        }

        false
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        self.balance
            .get_mut((account as u32 as usize).wrapping_sub(1))
            .is_some_and(|value| {
                *value += money;

                true
            })
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if let Some(value) = self.balance.get_mut((account as u32 as usize).wrapping_sub(1)) {
            if money <= *value {
                *value -= money;

                return true;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Bank for Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self::new(balance)
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        self.transfer(account1, account2, money)
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        self.deposit(account, money)
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        self.withdraw(account, money)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Bank>();
    }
}
