pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn get_sums(nums: &[i32], sums: &mut [i32]) -> Box<[Vec<i32>]> {
        let n = nums.len();
        let mut result = vec![Vec::new(); n + 1].into_boxed_slice();

        result[0].push(0);

        for selected in 1..sums.len() {
            let sum = sums[selected & (selected - 1)] + nums[selected.trailing_zeros() as usize];

            sums[selected] = sum;
            result[selected.count_ones() as usize].push(sum);
        }

        result
    }

    pub fn minimum_difference(nums: Vec<i32>) -> i32 {
        let (left, right) = nums.split_at(nums.len() / 2);
        let mut sums = vec![0; 1 << left.len()].into_boxed_slice();
        let mut left_sums = Self::get_sums(left, &mut sums);
        let mut right_sums = Self::get_sums(right, &mut sums);
        let get_last = |sums: &[Vec<i32>]| *sums.last().and_then(|sums| sums.first()).unwrap();
        let sum = get_last(&left_sums) + get_last(&right_sums);
        let half = sum.div_euclid(2);
        let mut min_sum = u32::MAX;

        for (left_sums, right_sums) in left_sums.iter_mut().rev().zip(&mut *right_sums) {
            right_sums.sort_unstable();

            for &left_sum in &*left_sums {
                let target = half - left_sum;
                let i = right_sums.partition_point(|&x| x < target);

                if let Some(&right_sum) = right_sums.get(i) {
                    min_sum = u32::min(min_sum, ((left_sum + right_sum) * 2 - sum) as _);
                }
            }
        }

        min_sum as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_difference(nums: Vec<i32>) -> i32 {
        Self::minimum_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
