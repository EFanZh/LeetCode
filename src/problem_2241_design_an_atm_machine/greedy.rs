// ------------------------------------------------------ snip ------------------------------------------------------ //

#[expect(clippy::struct_field_names, clippy::upper_case_acronyms, reason = "optimal")]
pub struct ATM {
    count_20: u32,
    count_50: u32,
    count_100: u32,
    count_200: u32,
    count_500: u32,
}

impl ATM {
    fn new() -> Self {
        Self {
            count_20: 0,
            count_50: 0,
            count_100: 0,
            count_200: 0,
            count_500: 0,
        }
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        let [count_20, count_50, count_100, count_200, count_500] = banknotes_count.try_into().ok().unwrap();

        self.count_20 += count_20 as u32;
        self.count_50 += count_50 as u32;
        self.count_100 += count_100 as u32;
        self.count_200 += count_200 as u32;
        self.count_500 += count_500 as u32;
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount as u32;

        let count_500 = (amount / 500).min(self.count_500);

        amount -= 500 * count_500;

        let count_200 = (amount / 200).min(self.count_200);

        amount -= 200 * count_200;

        let count_100 = (amount / 100).min(self.count_100);

        amount -= 100 * count_100;

        let count_50 = (amount / 50).min(self.count_50);

        amount -= 50 * count_50;

        let count_20 = (amount / 20).min(self.count_20);

        amount -= 20 * count_20;

        if amount == 0 {
            self.count_20 -= count_20;
            self.count_50 -= count_50;
            self.count_100 -= count_100;
            self.count_200 -= count_200;
            self.count_500 -= count_500;

            vec![
                count_20 as _,
                count_50 as _,
                count_100 as _,
                count_200 as _,
                count_500 as _,
            ]
        } else {
            vec![-1]
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::ATM for ATM {
    fn new() -> Self {
        Self::new()
    }

    fn deposit(&mut self, banknotes_count: Vec<i32>) {
        self.deposit(banknotes_count);
    }

    fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        self.withdraw(amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::ATM>();
    }
}
