pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn sum(first: u32, n: u32) -> u32 {
        (first + first + n - 1) * n / 2
    }

    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let n = n as u32;
        let k = k as u32;
        let k_half = k / 2;

        (if n <= k_half {
            Self::sum(1, n)
        } else {
            Self::sum(1, k_half) + Self::sum(k, n - k_half)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_sum(n: i32, k: i32) -> i32 {
        Self::minimum_sum(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
