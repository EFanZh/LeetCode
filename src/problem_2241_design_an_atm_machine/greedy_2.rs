// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

#[allow(clippy::struct_field_names, clippy::upper_case_acronyms)]
pub struct ATM {
    counts: [u32; 5],
}

impl ATM {
    fn new() -> Self {
        Self { counts: [0; 5] }
    }

    pub fn deposit(&mut self, banknotes_count: Vec<i32>) {
        let banknotes_count: &[_; 5] = banknotes_count.as_slice().try_into().ok().unwrap();

        for (target, &count) in self.counts.iter_mut().zip(banknotes_count) {
            *target += count as u32;
        }
    }

    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        let mut amount = amount as u32;
        let mut result = vec![0; 5];
        let result_ref: &mut [_; 5] = result.as_mut_slice().try_into().ok().unwrap();

        let mut iter = result_ref
            .iter_mut()
            .zip([20, 50, 100, 200, 500].iter().zip(&self.counts));

        loop {
            if let Some((target, (&base, &available))) = iter.next_back() {
                let count = (amount / base).min(available);

                *target = count as _;
                amount -= base * count;

                if amount != 0 {
                    continue;
                }

                for (available, required) in self.counts.iter_mut().zip(&*result_ref) {
                    *available -= *required as u32;
                }
            } else {
                result_ref[0] = -1;
                result.truncate(1);
            }

            break;
        }

        result
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
