pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn dfs(base: u32, last_digit: u32, n: u32, k: u32, result: &mut Vec<i32>) {
        if n == 0 {
            result.push(base as _);
        } else {
            if let Some(low) = last_digit.checked_sub(k) {
                Self::dfs(base * 10 + low, low, n - 1, k, result);
            }

            let high = last_digit + k;

            if high < 10 {
                Self::dfs(base * 10 + high, high, n - 1, k, result);
            }
        }
    }

    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let n = n as u32;
        let k = k as u32;
        let mut result = Vec::new();

        if k == 0 {
            let step = u32::pow(10, n) / 9;
            let mut value = 0;

            result.extend((0..9).map(|_| {
                value += step;

                value as i32
            }));
        } else {
            for i in 1..10 {
                Self::dfs(i, i, n - 1, k, &mut result);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        Self::nums_same_consec_diff(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
