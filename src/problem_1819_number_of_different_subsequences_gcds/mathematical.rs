pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut nums_set =
            vec![false; nums.iter().fold(0_usize, |max, &x| max.max(x as u32 as usize))].into_boxed_slice();

        for num in nums {
            nums_set[num as u32 as usize - 1] = true;
        }

        let n = nums_set.len();

        let multiples = (0..n)
            .map(|i| {
                nums_set[i..]
                    .iter()
                    .step_by(i + 1)
                    .fold(0, |sum, &x| sum + u32::from(x))
            })
            .collect::<Box<_>>();

        drop(nums_set);

        let mut result = 0;

        for i in 0..n {
            let mut iter = multiples[i..].iter().step_by(i + 1).copied();
            let first_count = iter.next().unwrap();

            result += i32::from(first_count != 0 && iter.all(|count| count != first_count));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        Self::count_different_subsequence_gc_ds(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
