pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for bank in bank {
            let offset = (u32::from(b'0') * bank.len() as u32).wrapping_neg();

            let devices = bank
                .as_bytes()
                .iter()
                .fold(offset, |sum, &c| sum.wrapping_add(u32::from(c)));

            drop(bank);

            if devices != 0 {
                result += prev * devices;
                prev = devices;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_beams(bank: Vec<String>) -> i32 {
        Self::number_of_beams(bank)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
