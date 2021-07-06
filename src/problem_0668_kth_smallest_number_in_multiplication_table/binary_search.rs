pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(m: u32, n: u32, k: u32, x: u32) -> bool {
        let top_rows = x / n;
        let mut count = n * top_rows;

        for i in (top_rows + 1)..=m.min(x) {
            count += x / i;
        }

        count < k
    }

    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (m, n) = if n < m {
            (n as u32, m as u32)
        } else {
            (m as u32, n as u32)
        };

        assert!(n != 0);

        let k = k as u32;
        let mut left = 1;
        let mut right = m * n;

        while left < right {
            let middle = (left + right) / 2;

            if Self::check(m, n, k, middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        Self::find_kth_number(m, n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
