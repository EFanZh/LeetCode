pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_area(start: u64, end: u64) -> u64 {
        (start + end) * (end + 1 - start) / 2
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let n = n as u32;
        let index = index as u32;
        let index = index.min(n - 1 - index);
        let max_sum = max_sum as u32 - n;
        let n_u64 = u64::from(n);
        let index_u64 = u64::from(index);
        let max_sum_u64 = u64::from(max_sum);

        (if max_sum_u64 <= (index_u64 + 1).pow(2) {
            f64::from(max_sum).sqrt() as u32
        } else {
            let threshold =
                Self::get_area(1, n_u64 - index_u64) + Self::get_area(n_u64 - index_u64 * 2, n_u64 - index_u64 - 1);

            if max_sum_u64 <= threshold {
                (f64::sqrt((8 * (max_sum_u64 + index_u64 * (index_u64 + 1)) + 1) as _) as u32 - index * 2 - 1) / 2
            } else {
                n - index + (max_sum - threshold as u32) / n
            }
        } + 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        Self::max_value(n, index, max_sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
