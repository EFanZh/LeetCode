pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let n = n as u32;
        let mut k = k as usize - 1;
        let mut lhs = 1;
        let sqrt_n = f64::from(n).sqrt() as u32;
        let mut factors = Vec::new();

        (loop {
            if lhs < sqrt_n {
                if n % lhs == 0 {
                    if k == 0 {
                        break lhs;
                    }

                    k -= 1;
                    factors.push(n / lhs);
                }

                lhs += 1;
            } else {
                if n % lhs == 0 {
                    if k == 0 {
                        break lhs;
                    }

                    k -= 1;

                    let rhs = n / lhs;

                    if lhs != rhs {
                        factors.push(rhs);
                    }
                }

                break factors
                    .get(factors.len().wrapping_sub(k + 1))
                    .copied()
                    .unwrap_or(u32::MAX);
            }
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_factor(n: i32, k: i32) -> i32 {
        Self::kth_factor(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
