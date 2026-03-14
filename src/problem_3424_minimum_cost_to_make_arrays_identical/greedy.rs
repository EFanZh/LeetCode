pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn calculate_cost(lhs: &[i32], rhs: &[i32]) -> u64 {
        lhs.iter()
            .zip(rhs)
            .fold(0, |cost, (&x, &y)| cost + u64::from(x.abs_diff(y)))
    }

    pub fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
        let mut arr = arr;
        let mut brr = brr;
        let candidate_1 = Self::calculate_cost(&arr, &brr);

        arr.sort_unstable();
        brr.sort_unstable();

        let candidate_2 = Self::calculate_cost(&arr, &brr) + k.cast_unsigned();

        u64::min(candidate_1, candidate_2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(arr: Vec<i32>, brr: Vec<i32>, k: i64) -> i64 {
        Self::min_cost(arr, brr, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
