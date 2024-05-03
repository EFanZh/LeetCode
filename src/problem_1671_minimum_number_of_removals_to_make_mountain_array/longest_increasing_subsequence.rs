pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn update_longest_increasing_subsequence(stack: &mut Vec<i32>, num: i32) -> u16 {
        let i = stack.partition_point(|&x| x < num);

        if let Some(tail) = stack.get_mut(i) {
            *tail = num;
        } else {
            stack.push(num);
        }

        i as _
    }

    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::with_capacity(nums.len());

        let left_lengths = nums
            .iter()
            .map(|&num| Self::update_longest_increasing_subsequence(&mut stack, num))
            .collect::<Box<_>>();

        stack.clear();

        let mut max_length = 0;

        for (&num, &left_length) in nums.iter().zip(&*left_lengths).rev() {
            let right_length = Self::update_longest_increasing_subsequence(&mut stack, num);

            if left_length != 0 && right_length != 0 {
                max_length = max_length.max(left_length + right_length);
            }
        }

        i32::from(nums.len() as u16 - (max_length + 1))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        Self::minimum_mountain_removals(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
