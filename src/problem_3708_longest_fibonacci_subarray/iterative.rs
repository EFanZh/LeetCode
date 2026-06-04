pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        if let Some((head, body)) = nums.split_first_chunk::<2>() {
            let [mut prev_1, mut prev_2] = *head;
            let mut length = 2;
            let mut result = 2;

            for &num in body {
                if prev_1.checked_add(prev_2) == Some(num) {
                    length += 1;
                } else {
                    result = result.max(length);
                    length = 2;
                }

                prev_1 = prev_2;
                prev_2 = num;
            }

            result.max(length)
        } else {
            nums.len() as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subarray(nums: Vec<i32>) -> i32 {
        Self::longest_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
