pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut buffer = vec![0; n * 2];
        let (left_max_sums, right_max_sums) = buffer.split_at_mut(n);
        let mut sum = 0;
        let mut min_sum = 0;
        let mut max_sum_without_removing = i32::MIN;

        for (target, &value) in left_max_sums.iter_mut().zip(&arr) {
            sum += value;
            *target = sum - min_sum;
            min_sum = min_sum.min(sum);
            max_sum_without_removing = max_sum_without_removing.max(*target);
        }

        sum = 0;
        min_sum = 0;

        for (target, &value) in right_max_sums.iter_mut().zip(&arr).rev() {
            sum += value;
            *target = sum - min_sum;
            min_sum = min_sum.min(sum);
        }

        let mut max_sum_with_one_deletion = i32::MIN;

        for (left_max_sum, right_max_sum) in left_max_sums.iter().zip(right_max_sums.iter().skip(2)) {
            max_sum_with_one_deletion = max_sum_with_one_deletion.max(left_max_sum + right_max_sum);
        }

        max_sum_without_removing.max(max_sum_with_one_deletion)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_sum(arr: Vec<i32>) -> i32 {
        Self::maximum_sum(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
