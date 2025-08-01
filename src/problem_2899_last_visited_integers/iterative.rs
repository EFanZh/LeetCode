pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut seen = 0;
        let mut k = 0;
        let mut result = Vec::new();

        for i in 0..nums.len() {
            let num = nums[i];

            if num == -1 {
                k += 1;
                result.push(nums.get(usize::wrapping_sub(seen, k)).copied().unwrap_or(-1));
            } else {
                k = 0;
                nums[seen] = num;
                seen += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn last_visited_integers(nums: Vec<i32>) -> Vec<i32> {
        Self::last_visited_integers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
