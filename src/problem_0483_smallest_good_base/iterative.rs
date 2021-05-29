pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn write_u64(value: u64, mut target: String) -> String {
        use std::fmt::Write;

        target.clear();

        write!(&mut target, "{}", value).unwrap();

        target
    }

    fn get_value(base: u64, length: u32) -> u64 {
        let base = u128::from(base);

        ((base.pow(length) - 1) / (base - 1)) as _
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn smallest_good_base(n: String) -> String {
        let n_value = n.parse::<u64>().unwrap();
        let n_f64 = n_value as f64;

        for length in (3..65 - n_value.leading_zeros()).rev() {
            let base = n_f64.powf(f64::from(length - 1).recip()) as u64;

            if Self::get_value(base, length) == n_value {
                return Self::write_u64(base, n);
            }
        }

        Self::write_u64(n_value - 1, n)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_good_base(n: String) -> String {
        Self::smallest_good_base(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
