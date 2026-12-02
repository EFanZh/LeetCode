pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_difference(nums: Vec<i32>) -> i64 {
        let mut cache = [0; 2];
        let (chunks, tail) = nums.as_chunks::<2>();

        let mut right_num = if let &[last] = tail {
            cache[0] = i64::from(last);

            last
        } else {
            0
        };

        for &[num_1, num_2] in chunks.iter().rev() {
            for (slot, first, second) in [(1, num_2, right_num), (0, num_1, num_2)] {
                let sum = first;
                let remainder_1 = sum % 3;
                let next_1 = cache[1 - slot];
                let sum = i64::from(sum);
                let score_1 = if remainder_1 == 0 { sum - next_1 } else { sum + next_1 };

                let sum = first + second;
                let remainder_2 = sum % 3;
                let next_2 = cache[slot];
                let sum = i64::from(sum);
                let score_2 = if remainder_2 == 0 { sum - next_2 } else { sum + next_2 };

                cache[slot] = score_1.max(score_2);
            }

            right_num = num_1;
        }

        cache[0]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_difference(nums: Vec<i32>) -> i64 {
        Self::max_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
