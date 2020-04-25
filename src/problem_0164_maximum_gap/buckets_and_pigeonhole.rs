pub struct Solution {}

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let maybe_min_value = nums.iter().min();
        let maybe_max_value = nums.iter().max();

        if maybe_min_value != maybe_max_value {
            let min_value = *maybe_min_value.unwrap();
            let max_value = *maybe_max_value.unwrap();
            let length = max_value - min_value;

            // The maximum gap must be greater than or equal to length / (n - 1), so as long as the bucket size is less
            // than length / (n - 1) + 1, the maximum gap can not be inside one bucket. So we only need to check gaps
            // between buckets.

            let bucket_size = {
                let n = nums.len() as i32;

                (length + n - 2) / (n - 1)
            };

            let mut buckets = vec![(-1, 0); (length / bucket_size + 1) as _];

            for num in nums {
                let (left, right) = &mut buckets[((num - min_value) / bucket_size) as usize];

                if *left == -1 {
                    *left = num;
                    *right = num;
                } else if num < *left {
                    *left = num;
                } else if num > *right {
                    *right = num;
                }
            }

            let mut bucket_iter = buckets.into_iter().filter(|(left, _)| *left != -1);
            let mut previous = bucket_iter.next().unwrap().1;

            for (left, right) in bucket_iter {
                result = result.max(left - previous);

                previous = right;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run;
    use super::Solution;

    #[test]
    fn test_solution() {
        run::<Solution>();
    }
}
