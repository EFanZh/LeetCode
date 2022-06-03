pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut min_sub_sub = i32::MAX / 2;
        let mut min_sub_plus = i32::MAX / 2;
        let mut min_plus_sub = i32::MAX / 2;
        let mut min_plus_plus = i32::MAX / 2;

        for (i, (x, y)) in (0..).zip(arr1.into_iter().zip(arr2)) {
            let sub_sub = -x - y + i;
            let sub_plus = -x + y + i;
            let plus_sub = x - y + i;
            let plus_plus = x + y + i;

            result = result.max(sub_sub - min_sub_sub);
            result = result.max(sub_plus - min_sub_plus);
            result = result.max(plus_sub - min_plus_sub);
            result = result.max(plus_plus - min_plus_plus);

            min_sub_sub = min_sub_sub.min(sub_sub);
            min_sub_plus = min_sub_plus.min(sub_plus);
            min_plus_sub = min_plus_sub.min(plus_sub);
            min_plus_plus = min_plus_plus.min(plus_plus);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        Self::max_abs_val_expr(arr1, arr2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
