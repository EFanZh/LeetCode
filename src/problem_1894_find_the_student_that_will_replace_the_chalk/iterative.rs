pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut k = k as u32;

        if let Some(sum) = chalk.iter().try_fold(0_u32, |sum, &x| sum.checked_add(x as _)) {
            k %= sum;
        }

        let mut i = 0;
        let mut iter = chalk.iter().map(|&x| x as u32);

        loop {
            let required = iter.next().unwrap();

            if k < required {
                return i;
            }

            k -= required;
            i += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        Self::chalk_replacer(chalk, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
