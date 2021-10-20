pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len() as u32;
        let k = k as u32;

        let cumulative_sums = {
            let mut sum = 0;

            nums.iter()
                .map(|&num| {
                    sum += num;

                    f64::from(sum)
                })
                .collect::<Vec<_>>()
        };

        let get_average = |start, end| {
            (cumulative_sums[end as usize - 1] - cumulative_sums[start as usize - 1]) / f64::from(end - start)
        };

        let mut cache = (1..)
            .zip(&cumulative_sums)
            .map(|(count, &sum)| sum / f64::from(count))
            .collect::<Vec<_>>();

        for max_count in 2..=k {
            for length in (max_count..=n).rev() {
                cache[length as usize - 1] = (max_count - 1..)
                    .zip(&cache[max_count as usize - 2..length as usize - 1])
                    .map(|(start, &max_sum)| max_sum + get_average(start, length))
                    .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
                    .unwrap();
            }
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        Self::largest_sum_of_averages(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
